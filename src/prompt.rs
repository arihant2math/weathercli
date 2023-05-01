use std::{io, thread};
use std::io::Write;
use std::time::Duration;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crate::{color, error};

fn draw<S: AsRef<str>>(options: &[S], choice: usize, multiline: bool) -> String {
    assert!(options.len() > choice);
    let mut result = String::new();
    if multiline {
        for (count, option) in options.iter().enumerate() {
            if count == choice {
                result += color::FORE_MAGENTA;
                result += "> \x1b[32m";
            } else {
                result += "\x1b[34m  ";
            }
            result += option.as_ref();
            result += "\x1b[39m";
            result += "\n";
        }
    } else {
        for (count, option) in options.iter().enumerate() {
            if count == choice {
                result += color::FORE_GREEN;
            } else {
                result += color::FORE_BLUE;
            }
            result += option.as_ref();
            result += "\x1b[39m";
            result += " ";
        }
    }
    result
}

pub fn choice<S: AsRef<str>>(options: &[S], default: usize, multiline: Option<bool>) -> crate::Result<usize> {
    let multiline_standard = multiline.unwrap_or(true); // TODO: Fix occasional no display bug
    thread::sleep(Duration::from_millis(100));
    read()?;
    let start_msg = draw(options, default, multiline_standard);
    print!("{start_msg}");
    io::stdout().flush()?;
    // entering raw mode
    enable_raw_mode()?;
    let mut choice = default;
    // key detection
    loop {
        print!("\x1b[1000D");
        if multiline_standard {
            print!("\x1b[{}A", options.len());
        } else {
            print!("\x1b[1A");
        }
        print!("{}", draw(options, choice, multiline_standard));
        io::stdout().flush()?;
        read()?;
        // matching the key
        match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Up | KeyCode::Left,
                ..
            }) => choice = choice.saturating_sub(1),
            Event::Key(KeyEvent {
                code: KeyCode::Down | KeyCode::Right,
                ..
            }) => choice = choice.saturating_add(1),
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {disable_raw_mode()?; return Err(error::Error::IoError("Control-C pressed".to_string()))?},
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => break,
            _ => (),
        }
        if choice >= options.len() {
            choice = options.len() - 1;
        }
        thread::sleep(Duration::from_millis(10));
    }
    // disabling raw mode
    disable_raw_mode()?;
    Ok(choice)
}
