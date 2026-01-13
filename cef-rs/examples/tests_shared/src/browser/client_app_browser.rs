use crate::{
    browser::main_message_loop_external_pump::*,
    common::{client_app::*, client_switches::*},
};
use cef::*;
use std::{
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

pub trait Delegate: Send {
    fn on_before_command_line_processing(
        &self,
        _app: &Arc<ClientAppBrowser>,
        _command_line: &mut CommandLine,
    ) {
    }

    fn on_register_custom_preferences(
        &self,
        _app: &Arc<ClientAppBrowser>,
        _type_: PreferencesType,
        _registrar: &mut PreferenceRegistrar,
    ) {
    }

    fn on_context_initialized(&self, _app: &Arc<ClientAppBrowser>) {}

    fn on_already_running_app_relaunch(
        &self,
        _app: &Arc<ClientAppBrowser>,
        _command_line: &mut CommandLine,
        _current_directory: &Path,
    ) -> bool {
        false
    }

    fn default_client(&self, _app: &Arc<ClientAppBrowser>) -> Option<Client> {
        None
    }
}

pub struct ClientAppBrowser {
    delegates: Vec<Box<dyn Delegate>>,
}

impl ClientAppBrowser {
    pub fn new(delegates: Vec<Box<dyn Delegate>>) -> Rc<Self> {
        Rc::new(Self { delegates })
    }

    pub fn populate_settings(
        command_line: Option<CommandLine>,
        cookieable_schemes: Vec<String>,
        settings: Settings,
    ) -> Settings {
        #[cfg(any(target_os = "windows", target_os = "linux"))]
        let settings = {
            Settings {
                multi_threaded_message_loop: command_line.as_ref().map_or(0, |command_line| {
                    command_line.has_switch(Some(&CefString::from(MULTI_THREADED_MESSAGE_LOOP)))
                }),
                ..settings
            }
        };

        let settings = if settings.multi_threaded_message_loop == 0 {
            Settings {
                external_message_pump: command_line.as_ref().map_or(0, |command_line| {
                    command_line.has_switch(Some(&CefString::from(EXTERNAL_MESSAGE_PUMP)))
                }),
                ..settings
            }
        } else {
            settings
        };

        let cookieable_schemes_list = CefString::from(cookieable_schemes.join(",").as_str());

        Settings {
            cookieable_schemes_list,
            ..settings
        }
    }

    pub fn delegates(&self) -> &[Box<dyn Delegate>] {
        &self.delegates
    }
}

wrap_app! {
    pub struct ClientAppBrowserApp {
        base: ClientApp,
        client_app_browser: Arc<ClientAppBrowser>,
    }

    impl App {
        fn on_before_command_line_processing(
            &self,
            process_type: Option<&CefString>,
            command_line: Option<&mut CommandLine>,
        ) {
            let (Some(process_type), Some(command_line)) = (process_type, command_line) else {
                return;
            };

            // Pass additional command-line flags to the browser process.
            if process_type.to_string().is_empty() {
                // Pass additional command-line flags when off-screen rendering is enabled.
                if command_line.has_switch(Some(&CefString::from(OFF_SCREEN_RENDERING_ENABLED)))
                    != 0
                    && command_line.has_switch(Some(&CefString::from(SHARED_TEXTURE_ENABLED))) == 0
                {
                    // Use software rendering and compositing (disable GPU) for increased FPS
                    // and decreased CPU usage. This will also disable WebGL so remove these
                    // switches if you need that capability.
                    // See https://github.com/chromiumembedded/cef/issues/1257 for details.
                    if command_line.has_switch(Some(&CefString::from(ENABLE_GPU))) == 0 {
                        command_line.append_switch(Some(&CefString::from("disable-gpu")));
                        command_line
                            .append_switch(Some(&CefString::from("disable-gpu-compositing")));
                    }
                }

                if command_line.has_switch(Some(&CefString::from(USE_VIEWS))) != 0
                    && command_line.has_switch(Some(&CefString::from("top-chrome-md"))) == 0
                {
                    // Use non-material mode on all platforms by default. Among other things
                    // this causes menu buttons to show hover state. See usage of
                    // MaterialDesignController::IsModeMaterial() in Chromium code.
                    command_line.append_switch_with_value(
                        Some(&CefString::from("top-chrome-md")),
                        Some(&CefString::from("non-material")),
                    );
                }

                // Disable the toolchain prompt on macOS.
                #[cfg(target_os = "macos")]
                command_line.append_switch(Some(&CefString::from("use-mock-keychain")));

                // On Linux, in off screen rendering (OSR) shared texture mode, we must
                // ensure that ANGLE uses the EGL backend. Without this, DMABUF based
                // rendering will fail. The Chromium fallback path uses X11 pixmaps,
                // which are only supported by Mesa drivers (e.g., AMD and Intel).
                //
                // While Mesa supports DMABUFs via both EGL and pixmaps, the EGL based
                // DMA BUF import path is more robust and required for compatibility with
                // drivers like NVIDIA that do not support pixmaps.
                //
                // We also append the kOzonePlatform switch with value x11 to ensure
                // that X11 semantics are preserved, which is necessary for compatibility
                // with some GDK/X11 integrations (e.g. Wayland with AMD).
                #[cfg(target_os = "linux")]
                if command_line.has_switch(Some(&CefString::from(OFF_SCREEN_RENDERING_ENABLED)))
                    != 0
                    && command_line.has_switch(Some(&CefString::from(SHARED_TEXTURE_ENABLED))) != 0
                {
                    if command_line.has_switch(Some(&CefString::from(USE_ANGLE))) == 0 {
                        command_line.append_switch_with_value(
                            Some(&CefString::from(USE_ANGLE)),
                            Some(&CefString::from("gl-egl")),
                        );
                    }
                    if command_line.has_switch(Some(&CefString::from(OZONE_PLATFORM))) == 0 {
                        command_line.append_switch_with_value(
                            Some(&CefString::from(OZONE_PLATFORM)),
                            Some(&CefString::from("X11")),
                        );
                    }
                }
            }

            for delegate in self.client_app_browser.delegates() {
                delegate.on_before_command_line_processing(&self.client_app_browser, command_line);
            }
        }

        fn on_register_custom_schemes(&self, registrar: Option<&mut SchemeRegistrar>) {
            self.base.on_register_custom_schemes(registrar);
        }

        fn browser_process_handler(&self) -> Option<BrowserProcessHandler> {
            Some(ClientAppBrowserProcessHandler::new(
                self.client_app_browser.clone(),
            ))
        }
    }
}

wrap_browser_process_handler! {
    pub struct ClientAppBrowserProcessHandler {
        client_app_browser: Arc<ClientAppBrowser>,
    }

    impl BrowserProcessHandler {
        fn on_register_custom_preferences(
            &self,
            type_: PreferencesType,
            registrar: Option<&mut PreferenceRegistrar>,
        ) {
            let Some(registrar) = registrar else {
                return;
            };

            for delegate in self.client_app_browser.delegates() {
                delegate.on_register_custom_preferences(&self.client_app_browser, type_, registrar);
            }
        }

        fn on_context_initialized(&self) {
            for delegate in self.client_app_browser.delegates() {
                delegate.on_context_initialized(&self.client_app_browser);
            }
        }

        fn on_already_running_app_relaunch(
            &self,
            command_line: Option<&mut CommandLine>,
            current_directory: Option<&CefString>,
        ) -> i32 {
            let (Some(command_line), Some(current_directory)) = (command_line, current_directory)
            else {
                return 0;
            };

            let delegates = self.client_app_browser.delegates();
            if !delegates.is_empty() {
                let current_directory = PathBuf::from(current_directory.to_string().as_str());

                for delegate in delegates {
                    if delegate.on_already_running_app_relaunch(
                        &self.client_app_browser,
                        command_line,
                        current_directory.as_path(),
                    ) {
                        return 1;
                    }
                }
            }

            0
        }

        fn on_schedule_message_pump_work(&self, delay_ms: i64) {
            if let Some(message_loop) = get_main_message_loop() {
                if let Ok(mut message_loop) = message_loop.lock() {
                    message_loop.on_schedule_message_pump_work(delay_ms);
                }
            }
        }

        fn default_client(&self) -> Option<Client> {
            for delegate in self.client_app_browser.delegates() {
                let client = delegate.default_client(&self.client_app_browser);
                if client.is_some() {
                    return client;
                }
            }
            None
        }
    }
}
