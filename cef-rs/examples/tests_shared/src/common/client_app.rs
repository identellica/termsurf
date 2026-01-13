use cef::*;

pub const PROCESS_TYPE: &str = "type";
pub const RENDERER_PROCESS: &str = "renderer";
#[cfg(target_os = "linux")]
pub const ZYGOTE_PROCESS: &str = "zygote";

pub enum ProcessType {
    Browser,
    Renderer,
    #[cfg(target_os = "linux")]
    Zygote,
    Other,
}

impl From<&CommandLine> for ProcessType {
    fn from(value: &CommandLine) -> Self {
        let process_type = CefString::from(PROCESS_TYPE);
        if value.has_switch(Some(&process_type)) == 0 {
            return Self::Browser;
        }

        let value = CefString::from(&value.switch_value(Some(&process_type))).to_string();
        match value.as_str() {
            RENDERER_PROCESS => Self::Renderer,
            #[cfg(target_os = "linux")]
            ZYGOTE_PROCESS => Self::Zygote,
            _ => Self::Other,
        }
    }
}

#[derive(Clone)]
pub struct ClientAppCustomScheme {
    name: String,
    options: i32,
}

impl ClientAppCustomScheme {
    pub fn new(name: &str, options: &[SchemeOptions]) -> Self {
        let options = options.iter().fold(0, |acc, opt| acc | opt.get_raw());
        #[cfg(not(target_os = "windows"))]
        let options = options as i32;
        Self {
            name: name.to_string(),
            options,
        }
    }
}

wrap_app! {
    pub struct ClientApp {
        custom_schemes: Vec<ClientAppCustomScheme>,
    }

    impl App {
        fn on_register_custom_schemes(&self, registrar: Option<&mut SchemeRegistrar>) {
            let Some(registrar) = registrar else {
                return;
            };

            for scheme in &self.custom_schemes {
                let name = CefString::from(scheme.name.as_str());
                registrar.add_custom_scheme(Some(&name), scheme.options);
            }
        }
    }
}
