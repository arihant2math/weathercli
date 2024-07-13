pub use input::input;
pub use multiselect::multiselect;
pub use radio::radio;
pub use textarea::textarea;

pub mod input;
pub mod multiselect;
pub mod radio;
pub mod textarea;

// TODO: Accept prompt
pub fn yes_no<Number>(default: Number, multiline: Option<bool>) -> std::io::Result<bool>
where
    usize: From<Number>,
{
    Ok([false, true][radio(&["no", "yes"], usize::from(default), multiline)?])
}
