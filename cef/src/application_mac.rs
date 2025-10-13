use std::cell::Cell;

use objc2::{
    define_class, extern_protocol, msg_send, rc::Retained, runtime::*, ClassType, DefinedClass,
    MainThreadMarker,
};
use objc2_app_kit::{NSApp, NSApplication};

extern_protocol!(
    /// The binding of `CrAppProtocol`.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait CrAppProtocol {
        #[unsafe(method(isHandlingSendEvent))]
        unsafe fn is_handling_send_event(&self) -> Bool;
    }
);

extern_protocol!(
    /// The binding of `CrAppControlProtocol`.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait CrAppControlProtocol: CrAppProtocol {
        #[unsafe(method(setHandlingSendEvent:))]
        unsafe fn set_handling_send_event(&self, handling_send_event: Bool);
    }
);

extern_protocol!(
    /// The binding of `CefAppProtocol`.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait CefAppProtocol: CrAppControlProtocol {}
);

/// Instance variables of `SimpleApplication`.
pub struct SimpleApplicationIvars {
    handling_send_event: Cell<Bool>,
}

define_class!(
    /// A default `NSApplication` subclass that implements the required CEF protocols.
    ///
    /// This class provides the necessary `CefAppProtocol` conformance to
    /// ensure that events are handled correctly by the Chromium framework on macOS.
    ///
    /// # Usage
    ///
    /// For most new applications built with cef-rs, this is the class you should use.
    /// It must be activated by calling the `init` method at the very beginning
    /// of your `main` function.
    ///
    /// ```no_run
    /// // In your main function:
    /// #[cfg(target_os = "macos")]
    /// cef::application_mac::SimpleApplication::init();
    /// ```
    ///
    /// # Custom Implementations
    ///
    /// You should not use this implementation if you are integrating cef-rs
    /// into an existing macOS application that already has its own custom
    /// `NSApplication` subclass.
    ///
    /// In that scenario, you should instead implement the [`CrAppProtocol`],
    /// [`CrAppControlProtocol`], and [`CefAppProtocol`] traits on your existing
    /// application class with objc2 crate.
    #[unsafe(super(NSApplication))]
    #[ivars = SimpleApplicationIvars]
    pub struct SimpleApplication;

    unsafe impl CrAppControlProtocol for SimpleApplication {
        #[unsafe(method(setHandlingSendEvent:))]
        unsafe fn set_handling_send_event(&self, handling_send_event: Bool) {
            self.ivars().handling_send_event.set(handling_send_event);
        }
    }

    unsafe impl CrAppProtocol for SimpleApplication {
        #[unsafe(method(isHandlingSendEvent))]
        unsafe fn is_handling_send_event(&self) -> Bool {
            self.ivars().handling_send_event.get()
        }
    }

    unsafe impl CefAppProtocol for SimpleApplication {}
);

impl SimpleApplication {
    /// Initializes the global `NSApplication` instance with our custom `SimpleApplication`.
    ///
    /// This function must be called on the main thread before any other UI code
    /// creates the shared application instance. It ensures that CEF's event
    /// handling requirements are met.
    ///
    /// If other UI code has already created a shared application instance,
    /// this will return an error.
    ///
    /// # Safety
    ///
    /// This function interacts with UI and should only be called once
    /// at the beginning
    pub fn init() -> Result<Retained<Self>, ()> {
        let mtm = MainThreadMarker::new()
            .expect("`SimpleApplication::init` must be called on the main thread");

        unsafe {
            // Initialize mac application instance.
            // SAFETY: mtm ensure that here is the main thread.
            let _: Retained<AnyObject> = msg_send![SimpleApplication::class(), sharedApplication];
        }

        // If there was an invocation to NSApp prior to here,
        // then the NSApp will not be a SimpleApplication.
        // objc2's downcast ensure that this doesn't happen.
        let app = NSApp(mtm);

        if let Ok(app) = app.downcast() {
            Ok(app)
        } else {
            Err(())
        }
    }
}
