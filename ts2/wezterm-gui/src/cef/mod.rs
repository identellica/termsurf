//! CEF (Chromium Embedded Framework) integration for browser panes.
//!
//! This module provides browser functionality for TermSurf 2.0, enabling web content
//! to be rendered alongside terminal panes.

use anyhow::Result;
use std::sync::atomic::{AtomicBool, Ordering};

// Use ::cef to refer to the external cef crate, since this module is also named 'cef'
use ::cef as cef_crate;

// Import traits required by the wrap_app! macro
use cef_crate::{rc::Rc, App, ImplApp, WrapApp};

/// Global flag indicating whether CEF is initialized and available.
static CEF_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Returns true if CEF has been successfully initialized.
pub fn is_cef_available() -> bool {
    CEF_INITIALIZED.load(Ordering::Relaxed)
}

/// Minimal CEF App implementation.
/// This can be extended later for custom browser process handlers.
#[derive(Clone)]
struct MinimalApp;

cef_crate::wrap_app! {
    struct MinimalAppBuilder {
        app: MinimalApp,
    }

    impl App {
        // Minimal implementation - no custom behavior for now
    }
}

impl MinimalAppBuilder {
    fn build() -> cef_crate::App {
        Self::new(MinimalApp)
    }
}

/// CEF context that manages the CEF runtime.
/// This must be kept alive for the lifetime of the application while using CEF.
pub struct CefContext {
    #[cfg(target_os = "macos")]
    _library_loader: Option<cef_crate::library_loader::LibraryLoader>,
}

impl CefContext {
    /// Initialize CEF and return a context handle.
    /// Returns None if CEF framework is not available (e.g., running unbundled).
    pub fn init() -> Result<Option<Self>> {
        // Check if already initialized
        if CEF_INITIALIZED.load(Ordering::Relaxed) {
            return Ok(None);
        }

        #[cfg(target_os = "macos")]
        {
            let exe_path = std::env::current_exe()?;

            // Check if CEF framework exists
            let framework_path = exe_path.parent().and_then(|p| {
                p.join("../Frameworks/Chromium Embedded Framework.framework/Chromium Embedded Framework")
                    .canonicalize()
                    .ok()
            });

            if framework_path.is_none() {
                log::info!("CEF framework not found - browser features disabled");
                return Ok(None);
            }

            // Load the CEF library
            let loader = cef_crate::library_loader::LibraryLoader::new(&exe_path, false);
            if !loader.load() {
                log::warn!("Failed to load CEF library");
                return Ok(None);
            }

            // Initialize CEF settings
            let cache_path = Self::cache_path();
            let settings = cef_crate::Settings {
                windowless_rendering_enabled: 1,
                external_message_pump: 1,
                no_sandbox: 1,
                cache_path: cache_path.to_string_lossy().as_ref().into(),
                log_severity: cef_crate::LogSeverity::WARNING,
                ..Default::default()
            };

            // Create minimal app handler
            let mut app = MinimalAppBuilder::build();

            // Initialize CEF
            let args = cef_crate::args::Args::new();
            let result = cef_crate::initialize(
                Some(args.as_main_args()),
                Some(&settings),
                Some(&mut app),
                std::ptr::null_mut(),
            );

            if result != 1 {
                log::error!("CEF initialization failed");
                return Ok(None);
            }

            // Register message pump hook with the window crate's event loop
            ::window::set_message_pump_hook(cef_message_pump);

            CEF_INITIALIZED.store(true, Ordering::Relaxed);
            log::info!("CEF initialized successfully");

            Ok(Some(CefContext {
                _library_loader: Some(loader),
            }))
        }

        #[cfg(not(target_os = "macos"))]
        {
            // TODO: Implement CEF initialization for other platforms
            log::info!("CEF not yet implemented for this platform");
            Ok(None)
        }
    }

    /// Get the cache path for CEF data.
    fn cache_path() -> std::path::PathBuf {
        config::CACHE_DIR.join("cef")
    }
}

impl Drop for CefContext {
    fn drop(&mut self) {
        if CEF_INITIALIZED.load(Ordering::Relaxed) {
            log::info!("Shutting down CEF");
            cef_crate::shutdown();
            CEF_INITIALIZED.store(false, Ordering::Relaxed);
        }
    }
}

/// Message pump function called on each iteration of the main event loop.
/// This processes pending CEF work items.
fn cef_message_pump() {
    if CEF_INITIALIZED.load(Ordering::Relaxed) {
        cef_crate::do_message_loop_work();
    }
}
