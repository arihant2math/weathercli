pub mod input;
pub mod multiselect;
pub mod radio;
pub mod textarea;

pub use input::input;
pub use multiselect::multiselect;
pub use radio::radio;
pub use textarea::textarea;

pub fn yes_no<B>(default: B, multiline: Option<bool>) -> std::io::Result<bool> where usize: From<B> {
    Ok([false, true][radio(&["no", "yes"], usize::from(default), multiline)?])
}
