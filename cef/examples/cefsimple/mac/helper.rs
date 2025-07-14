use cef::{args::Args, *};

fn main() {
    let _ = api_hash(sys::CEF_API_VERSION_LAST, 0);
    let args = Args::new();

    #[cfg(target_os = "macos")]
    let mut sandbox = cef::sandbox::Sandbox::new();
    sandbox.initialize(args.as_main_args());

    #[cfg(target_os = "macos")]
    let _loader = {
        let loader = library_loader::LibraryLoader::new(&std::env::current_exe().unwrap(), true);
        assert!(loader.load());
        loader
    };

    execute_process(
        Some(args.as_main_args()),
        None::<&mut App>,
        std::ptr::null_mut(),
    );
}
