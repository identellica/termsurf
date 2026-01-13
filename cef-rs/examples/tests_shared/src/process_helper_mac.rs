use crate::{
    common::{client_app::*, client_app_other::*},
    renderer::client_app_renderer::*,
};
use cef::{args::Args, *};

pub fn run_main(
    args: Args,
    custom_schemes: Vec<ClientAppCustomScheme>,
    app_renderer_delegates: Vec<Box<dyn Delegate>>,
) -> Result<(), i32> {
    #[cfg(feature = "sandbox")]
    let _sandbox = {
        let mut sandbox = cef::sandbox::Sandbox::new();
        sandbox.initialize(args.as_main_args());
        sandbox
    };

    let _loader = {
        let loader = library_loader::LibraryLoader::new(&std::env::current_exe().unwrap(), true);
        assert!(loader.load());
        loader
    };

    let _ = api_hash(sys::CEF_API_VERSION_LAST, 0);

    let app = ClientApp::new(custom_schemes);
    let mut app = match args.as_cmd_line().map(|cmd| ProcessType::from(&cmd)) {
        Some(ProcessType::Renderer) => {
            let app_renderer = ClientAppRenderer::new(app_renderer_delegates);
            ClientAppRendererApp::new(app, app_renderer)
        }
        _ => ClientAppOther::new(app),
    };

    match execute_process(
        Some(args.as_main_args()),
        Some(&mut app),
        std::ptr::null_mut(),
    ) {
        0 => Ok(()),
        err => Err(err),
    }
}
