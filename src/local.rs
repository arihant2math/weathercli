pub mod cache;
pub mod settings;
pub mod weather_file;
#[cfg(target_os = "unix")]
mod xdg_user_dirs;
mod dirs;
