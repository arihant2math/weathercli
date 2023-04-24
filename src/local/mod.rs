pub mod cache;
pub mod dirs;
pub mod settings;
#[cfg(feature = "gui")]
pub mod settings_app;
pub mod weather_file;
#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios"))))]
mod xdg_user_dirs;
