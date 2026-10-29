//! Windows only: embed the app icon and version info (VERSIONINFO) into `wryte.exe`, so it
//! shows in Explorer, the taskbar, the Start menu and Alt-Tab.
//!
//! On every other target this does nothing. A missing resource compiler is a warning, so a
//! cross-compile from macOS or Linux still links, unless `WRYTE_REQUIRE_WINRES=1` turns it
//! into an error (for release builds).

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/wryte.ico");
    println!("cargo:rerun-if-env-changed=WRYTE_REQUIRE_WINRES");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/app-icon/wryte.ico")
        .set("ProductName", "Wryte")
        .set("FileDescription", "Wryte word processor")
        .set("CompanyName", "Wryte")
        .set("LegalCopyright", "Copyright (c) the Wryte contributors and the WordCraft authors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "Wryte.exe")
        .set("InternalName", "wryte");
    if let Err(e) = res.compile() {
        if std::env::var_os("WRYTE_REQUIRE_WINRES").is_some() {
            eprintln!("embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=wryte.exe built without icon/version resources: {e}");
    }
}
