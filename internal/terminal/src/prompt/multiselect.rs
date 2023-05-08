use crate::color::{FORE_BLUE, FORE_GREEN, FORE_LIGHTMAGENTA};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::Write;
use std::time::Duration;
use std::{io, thread};

// TODO: Fix no multiline drawing
pub fn draw<S: AsRef<str>>(
    options: &[S],
    selected: &[bool],
    highlighted: usize,
    multiline: bool,
) -> String {
    let mut result = String::new();
    if multiline {
        for (count, option) in options.iter().enumerate() {
            if count == highlighted {
                result += FORE_LIGHTMAGENTA;
                result += "> ";
            } else {
                result += "  ";
            }
            if selected[count] {
                result += FORE_GREEN;
            } else {
                result += FORE_BLUE;
            }
            result += option.as_ref();
            result += "\n";
        }
    } else {
        for (count, option) in options.iter().enumerate() {
            if selected[count] {
                result += FORE_GREEN;
            } else {
                result += FORE_BLUE;
            }
            result += option.as_ref();
            result += " ";
        }
    }
    result
}

pub fn multiselect<S: AsRef<str>>(
    options: &[S],
    defaults: &[bool],
    multiline: Option<bool>,
) -> io::Result<Vec<bool>> {
    println!("Press ctrl-q when finished, red is not selected and green is selected");
    let multiline_standard = multiline.unwrap_or(true);
    thread::sleep(Duration::from_millis(100));
    read()?;
    let mut selected = defaults.to_vec();
    let mut highlighted: usize = 0;
    let start_msg = draw(options, &selected, highlighted, multiline_standard);
    print!("{start_msg}");
    io::stdout().flush()?;
    // entering raw mode
    enable_raw_mode()?;
    // key detection
    loop {
        print!("\x1b[1000D");
        if multiline_standard {
            print!("\x1b[{}A", options.len());
        } else {
            print!("\x1b[1A");
        }
        print!(
            "{}",
            draw(options, &selected, highlighted, multiline_standard)
        );
        io::stdout().flush()?;
        read()?;
        // matching the key
        match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                break;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up | KeyCode::Char('w'),
                ..
            }) => highlighted = highlighted.saturating_sub(1),
            Event::Key(KeyEvent {
                code: KeyCode::Down | KeyCode::Char('s'),
                ..
            }) => highlighted = highlighted.saturating_add(1),
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                disable_raw_mode()?;
                panic!("Control-C pressed");
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                selected[highlighted] = !selected[highlighted];
            }
            _ => (),
        }
        if highlighted >= options.len() {
            highlighted = options.len() - 1;
        }
        thread::sleep(Duration::from_millis(10));
    }
    // disabling raw mode
    disable_raw_mode()?;
    Ok(selected)
}
