pub mod color;
pub mod prompt;
pub use crossterm;

pub use crossterm::cursor::MoveTo as move_to;
pub use crossterm::execute;
pub use crossterm::terminal::size as terminal_size;
pub use crossterm::terminal::window_size as terminal_window_size;
pub use crossterm::terminal::Clear as clear;
pub use crossterm::terminal::ClearType as clear_type;
pub use crossterm::terminal::EnterAlternateScreen as enter_alternate_screen;
pub use crossterm::terminal::LeaveAlternateScreen as leave_alternate_screen;
