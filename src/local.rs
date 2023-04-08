pub mod cache;
pub mod settings;
pub mod weather_file;
#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios"))))]
mod xdg_user_dirs;
pub(crate) mod dirs;
