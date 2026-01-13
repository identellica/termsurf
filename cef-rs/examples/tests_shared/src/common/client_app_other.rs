use cef::*;

wrap_app! {
    pub struct ClientAppOther {
        base: App,
    }

    impl App {
        fn on_register_custom_schemes(&self, registrar: Option<&mut SchemeRegistrar>) {
            self.base.on_register_custom_schemes(registrar);
        }
    }
}
