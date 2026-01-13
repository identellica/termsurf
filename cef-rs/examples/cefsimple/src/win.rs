use crate::shared;
use cef::*;

#[unsafe(no_mangle)]
unsafe extern "C" fn RunWinMain(
    instance: sys::HINSTANCE,
    _command_line: *const u8,
    _command_show: i32,
    sandbox_info: *mut u8,
) -> i32 {
    let _library = shared::load_cef();

    let main_args = MainArgs { instance };
    let args = args::Args::from(main_args);
    let Some(cmd_line) = args.as_cmd_line() else {
        return 1;
    };

    shared::run_main(args.as_main_args(), &cmd_line, sandbox_info);
    0
}
