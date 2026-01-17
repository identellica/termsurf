use crate::sessionhandler::{PduSender, SessionHandler};
use anyhow::Context;
use async_ossl::AsyncSslStream;
use codec::{DecodedPdu, Pdu};
use futures::FutureExt;
use mux::{Mux, MuxNotification};
use smol::prelude::*;
use smol::Async;
use wezterm_uds::UnixStream;

/// Result of attempting to send a PDU to a client
enum SendResult {
    /// PDU was sent successfully
    Ok,
    /// Client disconnected (BrokenPipe) - should exit gracefully
    ClientDisconnected,
}

/// Send a PDU to the client, handling BrokenPipe gracefully.
/// Returns SendResult::ClientDisconnected if the client has disconnected.
async fn send_pdu<T>(stream: &mut Async<T>, pdu: Pdu) -> anyhow::Result<SendResult>
where
    T: std::io::Read + std::io::Write + std::fmt::Debug + async_io::IoSafe,
{
    match pdu.encode_async(stream, 0).await {
        Ok(()) => {}
        Err(err) => {
            if is_broken_pipe(&err) {
                return Ok(SendResult::ClientDisconnected);
            }
            return Err(err).context("encoding PDU to client");
        }
    }
    match stream.flush().await {
        Ok(()) => Ok(SendResult::Ok),
        Err(err) if err.kind() == std::io::ErrorKind::BrokenPipe => {
            Ok(SendResult::ClientDisconnected)
        }
        Err(err) => Err(err).context("flushing PDU to client"),
    }
}

/// Check if an anyhow error is caused by a BrokenPipe
fn is_broken_pipe(err: &anyhow::Error) -> bool {
    // Check the error chain for a BrokenPipe io::Error
    for cause in err.chain() {
        if let Some(io_err) = cause.downcast_ref::<std::io::Error>() {
            if io_err.kind() == std::io::ErrorKind::BrokenPipe {
                return true;
            }
        }
    }
    false
}

/// Macro to send a notification PDU and return early if client disconnected
macro_rules! send_notification {
    ($stream:expr, $pdu:expr) => {
        match send_pdu($stream, $pdu).await? {
            SendResult::Ok => {}
            SendResult::ClientDisconnected => return Ok(()),
        }
    };
}

#[cfg(unix)]
pub trait AsRawDesc: std::os::unix::io::AsRawFd + std::os::fd::AsFd {}
#[cfg(windows)]
pub trait AsRawDesc: std::os::windows::io::AsRawSocket + std::os::windows::io::AsSocket {}

impl AsRawDesc for UnixStream {}
impl AsRawDesc for AsyncSslStream {}

#[derive(Debug)]
enum Item {
    Notif(MuxNotification),
    WritePdu(DecodedPdu),
    Readable,
}

pub async fn process<T>(stream: T) -> anyhow::Result<()>
where
    T: 'static,
    T: std::io::Read,
    T: std::io::Write,
    T: AsRawDesc,
    T: std::fmt::Debug,
    T: async_io::IoSafe,
{
    let stream = smol::Async::new(stream)?;
    process_async(stream).await
}

pub async fn process_async<T>(mut stream: Async<T>) -> anyhow::Result<()>
where
    T: 'static,
    T: std::io::Read,
    T: std::io::Write,
    T: std::fmt::Debug,
    T: async_io::IoSafe,
{
    log::trace!("process_async called");

    let (item_tx, item_rx) = smol::channel::unbounded::<Item>();

    let pdu_sender = PduSender::new({
        let item_tx = item_tx.clone();
        move |pdu| {
            item_tx
                .try_send(Item::WritePdu(pdu))
                .map_err(|e| anyhow::anyhow!("{:?}", e))
        }
    });
    let mut handler = SessionHandler::new(pdu_sender);

    {
        let mux = Mux::get();
        let tx = item_tx.clone();
        mux.subscribe(move |n| tx.try_send(Item::Notif(n)).is_ok());
    }

    loop {
        let rx_msg = item_rx.recv();
        let wait_for_read = stream.readable().map(|_| Ok(Item::Readable));

        match smol::future::or(rx_msg, wait_for_read).await {
            Ok(Item::Readable) => {
                let decoded = match Pdu::decode_async(&mut stream, None).await {
                    Ok(data) => data,
                    Err(err) => {
                        if let Some(err) = err.root_cause().downcast_ref::<std::io::Error>() {
                            if err.kind() == std::io::ErrorKind::UnexpectedEof {
                                // Client disconnected: no need to make a noise
                                return Ok(());
                            }
                        }
                        return Err(err).context("reading Pdu from client");
                    }
                };
                handler.process_one(decoded);
            }
            Ok(Item::WritePdu(decoded)) => {
                match decoded.pdu.encode_async(&mut stream, decoded.serial).await {
                    Ok(()) => {}
                    Err(err) => {
                        if let Some(err) = err.root_cause().downcast_ref::<std::io::Error>() {
                            if err.kind() == std::io::ErrorKind::BrokenPipe {
                                // Client disconnected: no need to make a noise
                                return Ok(());
                            }
                        }
                        return Err(err).context("encoding PDU to client");
                    }
                };
                match stream.flush().await {
                    Ok(()) => {}
                    Err(err) => {
                        if err.kind() == std::io::ErrorKind::BrokenPipe {
                            // Client disconnected: no need to make a noise
                            return Ok(());
                        }
                        return Err(err).context("flushing PDU to client");
                    }
                }
            }
            Ok(Item::Notif(MuxNotification::PaneOutput(pane_id))) => {
                handler.schedule_pane_push(pane_id);
            }
            Ok(Item::Notif(MuxNotification::PaneAdded(_pane_id))) => {}
            Ok(Item::Notif(MuxNotification::PaneRemoved(pane_id))) => {
                send_notification!(
                    &mut stream,
                    Pdu::PaneRemoved(codec::PaneRemoved { pane_id })
                );
            }
            Ok(Item::Notif(MuxNotification::Alert { pane_id, alert })) => {
                {
                    let per_pane = handler.per_pane(pane_id);
                    let mut per_pane = per_pane.lock().unwrap();
                    per_pane.notifications.push(alert);
                }
                handler.schedule_pane_push(pane_id);
            }
            Ok(Item::Notif(MuxNotification::SaveToDownloads { .. })) => {}
            Ok(Item::Notif(MuxNotification::AssignClipboard {
                pane_id,
                selection,
                clipboard,
            })) => {
                send_notification!(
                    &mut stream,
                    Pdu::SetClipboard(codec::SetClipboard {
                        pane_id,
                        clipboard,
                        selection,
                    })
                );
            }
            Ok(Item::Notif(MuxNotification::TabAddedToWindow { tab_id, window_id })) => {
                send_notification!(
                    &mut stream,
                    Pdu::TabAddedToWindow(codec::TabAddedToWindow { tab_id, window_id })
                );
            }
            Ok(Item::Notif(MuxNotification::WindowRemoved(_window_id))) => {}
            Ok(Item::Notif(MuxNotification::WindowCreated(_window_id))) => {}
            Ok(Item::Notif(MuxNotification::WindowInvalidated(_window_id))) => {}
            Ok(Item::Notif(MuxNotification::WindowWorkspaceChanged(window_id))) => {
                let workspace = {
                    let mux = Mux::get();
                    mux.get_window(window_id)
                        .map(|w| w.get_workspace().to_string())
                };
                if let Some(workspace) = workspace {
                    send_notification!(
                        &mut stream,
                        Pdu::WindowWorkspaceChanged(codec::WindowWorkspaceChanged {
                            window_id,
                            workspace,
                        })
                    );
                }
            }
            Ok(Item::Notif(MuxNotification::PaneFocused(pane_id))) => {
                send_notification!(
                    &mut stream,
                    Pdu::PaneFocused(codec::PaneFocused { pane_id })
                );
            }
            Ok(Item::Notif(MuxNotification::TabResized(tab_id))) => {
                send_notification!(
                    &mut stream,
                    Pdu::TabResized(codec::TabResized { tab_id })
                );
            }
            Ok(Item::Notif(MuxNotification::TabTitleChanged { tab_id, title })) => {
                send_notification!(
                    &mut stream,
                    Pdu::TabTitleChanged(codec::TabTitleChanged { tab_id, title })
                );
            }
            Ok(Item::Notif(MuxNotification::WindowTitleChanged { window_id, title })) => {
                send_notification!(
                    &mut stream,
                    Pdu::WindowTitleChanged(codec::WindowTitleChanged { window_id, title })
                );
            }
            Ok(Item::Notif(MuxNotification::WorkspaceRenamed {
                old_workspace,
                new_workspace,
            })) => {
                send_notification!(
                    &mut stream,
                    Pdu::RenameWorkspace(codec::RenameWorkspace {
                        old_workspace,
                        new_workspace,
                    })
                );
            }
            Ok(Item::Notif(MuxNotification::ActiveWorkspaceChanged(_))) => {}
            Ok(Item::Notif(MuxNotification::Empty)) => {}
            // WebOpen/WebClosed are handled by the GUI, not the server dispatcher
            Ok(Item::Notif(MuxNotification::WebOpen { .. })) => {}
            Ok(Item::Notif(MuxNotification::WebClosed { .. })) => {}
            Err(err) => {
                log::error!("process_async Err {}", err);
                return Ok(());
            }
        }
    }
}
