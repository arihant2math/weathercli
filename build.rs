use std::{env, io};
use winresource::WindowsResource;

fn main() -> io::Result<()> {
    let version = rustc_version::version().unwrap();
    println!("cargo:rustc-env=RUSTC_VERSION={}", version);
    println!("cargo:rerun-if-changed=build.rs");
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        WindowsResource::new()
            // This path can be absolute, or relative to your crate root.
            .set_icon("icon/updater.ico")
            .compile()?;
    }
    Ok(())
}
