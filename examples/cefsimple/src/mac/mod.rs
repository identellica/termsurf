use crate::shared::simple_handler::*;
use cef::application_mac::{CefAppProtocol, CrAppControlProtocol, CrAppProtocol};
use objc2::{
    ClassType, DefinedClass, MainThreadMarker, MainThreadOnly, define_class, extern_methods,
    msg_send,
    rc::Retained,
    runtime::{AnyObject, Bool, NSObject, NSObjectProtocol, ProtocolObject},
    sel,
};
use objc2_app_kit::{
    NSApp, NSApplication, NSApplicationDelegate, NSApplicationTerminateReply, NSEvent,
    NSUserInterfaceValidations, NSValidatedUserInterfaceItem,
};
use objc2_foundation::{NSBundle, NSObjectNSThreadPerformAdditions, ns_string};
use std::{cell::Cell, ptr};

define_class! {
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    pub struct SimpleAppDelegate;

    impl SimpleAppDelegate {
        /// Create the application on the UI thread.
        #[unsafe(method(createApplication:))]
        unsafe fn create_application(&self, _object: Option<&AnyObject>) {
            let app = NSApp(MainThreadMarker::new().expect("Not running on the main thread"));
            assert!(app.isKindOfClass(SimpleApplication::class()));
            assert!(
                app.delegate()
                    .unwrap()
                    .isKindOfClass(SimpleAppDelegate::class())
            );

            let main_bundle = NSBundle::mainBundle();
            let _: Bool = msg_send![&main_bundle,
                loadNibNamed: ns_string!("MainMenu"),
                owner: &*app,
                topLevelObjects: ptr::null_mut::<*const AnyObject>()
            ];
        }
    }

    unsafe impl NSObjectProtocol for SimpleAppDelegate {}

    unsafe impl NSApplicationDelegate for SimpleAppDelegate {
        #[unsafe(method(applicationShouldTerminate:))]
        unsafe fn application_should_terminate(&self, _sender: &NSApplication) -> NSApplicationTerminateReply {
            NSApplicationTerminateReply::TerminateNow
        }

        /// Called when the user clicks the app dock icon while the application is
        /// already running.
        #[unsafe(method(applicationShouldHandleReopen:hasVisibleWindows:))]
        unsafe fn application_should_handle_reopen(&self, _sender: &NSApplication, _has_visible_windows: Bool) -> Bool {
            if let Some(handler) = SimpleHandler::instance() {
                let mut handler = handler.lock().expect("Failed to lock SimpleHandler");
                if !handler.is_closing() {
                    handler.show_main_window();
                }
            }
            Bool::NO
        }

        /// Requests that any state restoration archive be created with secure encoding
        /// (macOS 12+ only). See https://crrev.com/c737387656 for details. This also
        /// fixes an issue with macOS default behavior incorrectly restoring windows
        /// after hard reset (holding down the power button).
        #[unsafe(method(applicationSupportsSecureRestorableState:))]
        unsafe fn application_supports_secure_restorable_state(&self, _sender: &NSApplication) -> Bool {
            Bool::YES
        }
    }

    unsafe impl NSUserInterfaceValidations for SimpleAppDelegate {
        #[unsafe(method(validateUserInterfaceItem:))]
        unsafe fn validate_user_interface_item(&self, item: &ProtocolObject<dyn NSValidatedUserInterfaceItem>) -> Bool {
            const IDC_FIND: isize = 37000;

            let tag = item.tag();
            if tag == IDC_FIND {
                Bool::YES
            } else {
                Bool::NO
            }
        }
    }
}

impl SimpleAppDelegate {
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = SimpleAppDelegate::alloc(mtm).set_ivars(());
        unsafe { msg_send![super(this), init] }
    }
}

/// Instance variables of `SimpleApplication`.
#[derive(Default)]
pub struct SimpleApplicationIvars {
    handling_send_event: Cell<Bool>,
}

define_class!(
    /// A `NSApplication` subclass that implements the required CEF protocols.
    ///
    /// This class provides the necessary `CefAppProtocol` conformance to
    /// ensure that events are handled correctly by the Chromium framework on macOS.
    #[unsafe(super(NSApplication))]
    #[ivars = SimpleApplicationIvars]
    pub struct SimpleApplication;

    impl SimpleApplication {
        #[unsafe(method(sendEvent:))]
        unsafe fn send_event(&self, event: &NSEvent) {
            let was_sending_event = self.is_handling_send_event();
            if !was_sending_event {
                self.set_handling_send_event(true);
            }

            let _: () = msg_send![super(self), sendEvent:event];

            if !was_sending_event {
                self.set_handling_send_event(false);
            }
        }

        /// |-terminate:| is the entry point for orderly "quit" operations in Cocoa. This
        /// includes the application menu's quit menu item and keyboard equivalent, the
        /// application's dock icon menu's quit menu item, "quit" (not "force quit") in
        /// the Activity Monitor, and quits triggered by user logout and system restart
        /// and shutdown.
        ///
        /// The default |-terminate:| implementation ends the process by calling exit(),
        /// and thus never leaves the main run loop. This is unsuitable for Chromium
        /// since Chromium depends on leaving the main run loop to perform an orderly
        /// shutdown. We support the normal |-terminate:| interface by overriding the
        /// default implementation. Our implementation, which is very specific to the
        /// needs of Chromium, works by asking the application delegate to terminate
        /// using its |-tryToTerminateApplication:| method.
        ///
        /// |-tryToTerminateApplication:| differs from the standard
        /// |-applicationShouldTerminate:| in that no special event loop is run in the
        /// case that immediate termination is not possible (e.g., if dialog boxes
        /// allowing the user to cancel have to be shown). Instead, this method tries to
        /// close all browsers by calling CloseBrowser(false) via
        /// ClientHandler::CloseAllBrowsers. Calling CloseBrowser will result in a call
        /// to ClientHandler::DoClose and execution of |-performClose:| on the NSWindow.
        /// DoClose sets a flag that is used to differentiate between new close events
        /// (e.g., user clicked the window close button) and in-progress close events
        /// (e.g., user approved the close window dialog). The NSWindowDelegate
        /// |-windowShouldClose:| method checks this flag and either calls
        /// CloseBrowser(false) in the case of a new close event or destructs the
        /// NSWindow in the case of an in-progress close event.
        /// ClientHandler::OnBeforeClose will be called after the CEF NSView hosted in
        /// the NSWindow is dealloc'ed.
        ///
        /// After the final browser window has closed ClientHandler::OnBeforeClose will
        /// begin actual tear-down of the application by calling CefQuitMessageLoop.
        /// This ends the NSApplication event loop and execution then returns to the
        /// main() function for cleanup before application termination.
        ///
        /// The standard |-applicationShouldTerminate:| is not supported, and code paths
        /// leading to it must be redirected.
        #[unsafe(method(terminate:))]
        unsafe fn terminate(&self, _sender: &AnyObject) {
            if let Some(handler) = SimpleHandler::instance() {
                let mut handler = handler.lock().expect("Failed to lock SimpleHandler");
                if !handler.is_closing() {
                    handler.close_all_browsers(false);
                }
            }
        }
    }

    unsafe impl CrAppControlProtocol for SimpleApplication {
        #[unsafe(method(setHandlingSendEvent:))]
        unsafe fn _set_handling_send_event(&self, handling_send_event: Bool) {
            self.ivars().handling_send_event.set(handling_send_event);
        }
    }

    unsafe impl CrAppProtocol for SimpleApplication {
        #[unsafe(method(isHandlingSendEvent))]
        unsafe fn _is_handling_send_event(&self) -> Bool {
            self.ivars().handling_send_event.get()
        }
    }

    unsafe impl CefAppProtocol for SimpleApplication {}
);

impl SimpleApplication {
    extern_methods! {
        #[unsafe(method(sharedApplication))]
        fn shared_application() -> Retained<Self>;

        #[unsafe(method(setHandlingSendEvent:))]
        fn set_handling_send_event(&self, handling_send_event: bool);

        #[unsafe(method(isHandlingSendEvent))]
        fn is_handling_send_event(&self) -> bool;
    }
}

pub fn setup_simple_application() {
    // Initialize the SimpleApplication instance.
    // SAFETY: mtm ensures that here is the main thread.
    let _ = SimpleApplication::shared_application();

    // If there was an invocation to NSApp prior to here,
    // then the NSApp will not be a SimpleApplication.
    // The following assertion ensures that this doesn't happen.
    assert!(
        NSApp(MainThreadMarker::new().expect("Not running on the main thread"))
            .isKindOfClass(SimpleApplication::class())
    );
}

pub fn setup_simple_app_delegate() -> Retained<SimpleAppDelegate> {
    let mtm = MainThreadMarker::new().expect("Not running on the main thread");

    // Create the application delegate.
    let simple_delegate = SimpleAppDelegate::new(mtm);
    let delegate_proto =
        ProtocolObject::<dyn NSApplicationDelegate>::from_retained(simple_delegate.clone());
    let app = NSApp(MainThreadMarker::new().expect("Not running on the main thread"));
    assert!(app.isKindOfClass(SimpleApplication::class()));
    app.setDelegate(Some(&delegate_proto));
    assert!(
        app.delegate()
            .unwrap()
            .isKindOfClass(SimpleAppDelegate::class())
    );

    unsafe {
        simple_delegate.performSelectorOnMainThread_withObject_waitUntilDone(
            sel!(createApplication:),
            None,
            false,
        );
    }

    simple_delegate
}
