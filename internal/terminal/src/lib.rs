pub mod color;
pub mod prompt;
use crossterm;


/// Gets the terminal size (width, height)
pub fn terminal_size() -> crate::Result<(u16, u16)> {
    let (w, h) = crossterm::terminal::size()?;
    Ok((w, h))
}
