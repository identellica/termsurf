#[cfg(target_os = "windows")]
include!("src/shared/resources.rs");

#[cfg(target_os = "windows")]
fn main() {
    winres::WindowsResource::new()
        .set_icon_with_id("resources/win/cefsimple.ico", &IDI_CEFSIMPLE.to_string())
        .set_icon_with_id("resources/win/small.ico", &IDI_SMALL.to_string())
        .compile()
        .unwrap();
}

#[cfg(not(target_os = "windows"))]
fn main() {}
