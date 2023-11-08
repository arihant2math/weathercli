pub mod input;
pub mod multiselect;
pub mod radio;
pub mod textarea;

pub use input::input;
pub use multiselect::multiselect;
pub use radio::radio;
pub use textarea::textarea;

pub fn yes_no(default: bool, multiline: Option<bool>) -> std::io::Result<bool> {
    Ok([true, false][radio(&["yes", "no"], usize::from(default), multiline)?])
}
