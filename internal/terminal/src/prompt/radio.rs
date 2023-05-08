use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::prompt::multiselect;
use std::io::Write;
use std::time::Duration;
use std::{io, thread};

fn draw<S: AsRef<str>>(options: &[S], highlighted: usize, multiline: bool) -> String {
    let mut falses = vec![false; options.len()];
    falses[highlighted] = true;
    multiselect::draw(options, &falses, highlighted, multiline)
}

pub fn radio<S: AsRef<str>>(
    options: &[S],
    default: usize,
    multiline: Option<bool>,
) -> io::Result<usize> {
    let multiline_standard = multiline.unwrap_or(true);
    thread::sleep(Duration::from_millis(100));
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
            }) => {
                disable_raw_mode()?;
                panic!("Control-C pressed");
            }
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
