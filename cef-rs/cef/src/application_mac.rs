use objc2::{extern_protocol, runtime::Bool};

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
