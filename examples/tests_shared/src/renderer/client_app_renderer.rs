use cef::*;
use std::sync::Arc;

pub trait Delegate: Send {
    fn on_web_kit_initialized(&self, _app: &Arc<ClientAppRenderer>) {}

    fn on_browser_created(
        &self,
        _app: &Arc<ClientAppRenderer>,
        _browser: Option<&Browser>,
        _extra_info: Option<&DictionaryValue>,
    ) {
    }

    fn on_browser_destroyed(&self, _app: &Arc<ClientAppRenderer>, _browser: Option<&Browser>) {}

    fn load_handler(&self, _app: &Arc<ClientAppRenderer>) -> Option<LoadHandler> {
        None
    }

    fn on_context_created(
        &self,
        _app: &Arc<ClientAppRenderer>,
        _browser: Option<&Browser>,
        _frame: Option<&Frame>,
        _context: Option<&V8Context>,
    ) {
    }

    fn on_context_released(
        &self,
        _app: &Arc<ClientAppRenderer>,
        _browser: Option<&Browser>,
        _frame: Option<&Frame>,
        _context: Option<&V8Context>,
    ) {
    }

    fn on_uncaught_exception(
        &self,
        _app: &Arc<ClientAppRenderer>,
        _browser: Option<&Browser>,
        _frame: Option<&Frame>,
        _context: Option<&V8Context>,
        _exception: Option<&V8Exception>,
        _stack_trace: Option<&V8StackTrace>,
    ) {
    }

    fn on_focused_node_changed(
        &self,
        _app: &Arc<ClientAppRenderer>,
        _browser: Option<&Browser>,
        _frame: Option<&Frame>,
        _node: Option<&Domnode>,
    ) {
    }

    fn on_process_message_received(
        &self,
        _app: &Arc<ClientAppRenderer>,
        _browser: Option<&Browser>,
        _frame: Option<&Frame>,
        _source_process: ProcessId,
        _message: Option<&ProcessMessage>,
    ) -> i32 {
        0
    }
}

pub struct ClientAppRenderer {
    delegates: Vec<Box<dyn Delegate>>,
}

impl ClientAppRenderer {
    pub fn new(delegates: Vec<Box<dyn Delegate>>) -> Arc<Self> {
        Arc::new(Self { delegates })
    }

    pub fn delegates(&self) -> &[Box<dyn Delegate>] {
        &self.delegates
    }
}

wrap_render_process_handler! {
    struct ClientAppRendererRenderProcessHandler {
        client_app_renderer: Arc<ClientAppRenderer>,
    }

    impl RenderProcessHandler {
        fn on_web_kit_initialized(&self) {
            for delegate in self.client_app_renderer.delegates() {
                delegate.on_web_kit_initialized(&self.client_app_renderer);
            }
        }

        fn on_browser_created(
            &self,
            browser: Option<&mut Browser>,
            extra_info: Option<&mut DictionaryValue>,
        ) {
            let browser = browser.cloned();
            let extra_info = extra_info.cloned();
            for delegate in self.client_app_renderer.delegates() {
                delegate.on_browser_created(
                    &self.client_app_renderer,
                    browser.as_ref(),
                    extra_info.as_ref(),
                );
            }
        }

        fn on_browser_destroyed(&self, browser: Option<&mut Browser>) {
            let browser = browser.cloned();
            for delegate in self.client_app_renderer.delegates() {
                delegate.on_browser_destroyed(&self.client_app_renderer, browser.as_ref());
            }
        }

        fn load_handler(&self) -> Option<LoadHandler> {
            for delegate in self.client_app_renderer.delegates() {
                if let Some(load_handler) = delegate.load_handler(&self.client_app_renderer) {
                    return Some(load_handler);
                }
            }
            None
        }

        fn on_context_created(
            &self,
            browser: Option<&mut Browser>,
            frame: Option<&mut Frame>,
            context: Option<&mut V8Context>,
        ) {
            let browser = browser.cloned();
            let frame = frame.cloned();
            let context = context.cloned();
            for delegate in self.client_app_renderer.delegates() {
                delegate.on_context_created(
                    &self.client_app_renderer,
                    browser.as_ref(),
                    frame.as_ref(),
                    context.as_ref(),
                );
            }
        }

        fn on_context_released(
            &self,
            browser: Option<&mut Browser>,
            frame: Option<&mut Frame>,
            context: Option<&mut V8Context>,
        ) {
            let browser = browser.cloned();
            let frame = frame.cloned();
            let context = context.cloned();
            for delegate in self.client_app_renderer.delegates() {
                delegate.on_context_released(
                    &self.client_app_renderer,
                    browser.as_ref(),
                    frame.as_ref(),
                    context.as_ref(),
                );
            }
        }

        fn on_uncaught_exception(
            &self,
            browser: Option<&mut Browser>,
            frame: Option<&mut Frame>,
            context: Option<&mut V8Context>,
            exception: Option<&mut V8Exception>,
            stack_trace: Option<&mut V8StackTrace>,
        ) {
            let browser = browser.cloned();
            let frame = frame.cloned();
            let context = context.cloned();
            let exception = exception.cloned();
            let stack_trace = stack_trace.cloned();
            for delegate in self.client_app_renderer.delegates() {
                delegate.on_uncaught_exception(
                    &self.client_app_renderer,
                    browser.as_ref(),
                    frame.as_ref(),
                    context.as_ref(),
                    exception.as_ref(),
                    stack_trace.as_ref(),
                );
            }
        }

        fn on_focused_node_changed(
            &self,
            browser: Option<&mut Browser>,
            frame: Option<&mut Frame>,
            node: Option<&mut Domnode>,
        ) {
            let browser = browser.cloned();
            let frame = frame.cloned();
            let node = node.cloned();
            for delegate in self.client_app_renderer.delegates() {
                delegate.on_focused_node_changed(
                    &self.client_app_renderer,
                    browser.as_ref(),
                    frame.as_ref(),
                    node.as_ref(),
                );
            }
        }

        fn on_process_message_received(
            &self,
            browser: Option<&mut Browser>,
            frame: Option<&mut Frame>,
            source_process: ProcessId,
            message: Option<&mut ProcessMessage>,
        ) -> i32 {
            let browser = browser.cloned();
            let frame = frame.cloned();
            let message = message.cloned();
            for delegate in self.client_app_renderer.delegates() {
                let handled = delegate.on_process_message_received(
                    &self.client_app_renderer,
                    browser.as_ref(),
                    frame.as_ref(),
                    source_process,
                    message.as_ref(),
                );
                if handled != 0 {
                    return handled;
                }
            }
            0
        }
    }
}

wrap_app! {
    pub struct ClientAppRendererApp {
        base: App,
        client_app_renderer: Arc<ClientAppRenderer>,
    }

    impl App {
        fn on_register_custom_schemes(&self, registrar: Option<&mut SchemeRegistrar>) {
            self.base.on_register_custom_schemes(registrar);
        }

        fn render_process_handler(&self) -> Option<RenderProcessHandler> {
            Some(ClientAppRendererRenderProcessHandler::new(
                self.client_app_renderer.clone(),
            ))
        }
    }
}
