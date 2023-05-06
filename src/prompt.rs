use std::io::Write;
use std::time::Duration;
use std::{io, thread};

use crate::error;
use crate::color::*;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

// TODO: Fix no multiline drawing

fn draw_multiselect<S: AsRef<str>>(options: &[S], selected: &[bool], highlighted: usize, multiline: bool) -> String {
    let mut result = String::new();
    if multiline {
        for (count, option) in options.iter().enumerate() {
            if count == highlighted {
                result += FORE_MAGENTA;
                result += "> ";
            }
            else {
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

fn draw_radio<S: AsRef<str>>(options: &[S], highlighted: usize, multiline: bool) -> String {
    let mut result = String::new();
    if multiline {
        for (count, option) in options.iter().enumerate() {
            if count == highlighted {
                result += FORE_MAGENTA;
                result += "> ";
            }
            else {
                result += "  ";
            }
            if count == highlighted {
                result += FORE_GREEN;
            } else {
                result += FORE_BLUE;
            }
            result += option.as_ref();
            result += "\n";
        }
    } else {
        for (count, option) in options.iter().enumerate() {
            if count == highlighted {
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
pub fn radio<S: AsRef<str>>(
    options: &[S],
    default: usize,
    multiline: Option<bool>,
) -> crate::Result<usize> {
    let multiline_standard = multiline.unwrap_or(true); // TODO: Fix occasional no display bug
    thread::sleep(Duration::from_millis(100));
    read()?;
    let start_msg = draw_radio(options, default, multiline_standard);
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
        print!("{}", draw_radio(options,  choice, multiline_standard));
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

pub fn multiselect<S: AsRef<str>>(
    options: &[S],
    defaults: &[bool],
    multiline: Option<bool>,
) -> crate::Result<Vec<bool>> {
    println!("Press ctrl-enter when finished");
    let multiline_standard = multiline.unwrap_or(true); // TODO: Fix occasional no display bug
    thread::sleep(Duration::from_millis(100));
    read()?;
    let mut selected = defaults.to_vec();
    let mut highlighted: usize = 0;
    let start_msg = draw_multiselect(options, &*selected, highlighted, multiline_standard);
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
        print!("{}", draw_multiselect(options, &selected, highlighted, multiline_standard));
        io::stdout().flush()?;
        read()?;
        // matching the key
        match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Up | KeyCode::Left,
                ..
            }) => highlighted = highlighted.saturating_sub(1),
            Event::Key(KeyEvent {
                code: KeyCode::Down | KeyCode::Right,
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
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                break;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                selected[highlighted] = !selected[highlighted];
            },
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

fn modify(choice: String, to_add: &str, position: usize) -> String {
    let first = &choice[..position];
    let second = &choice[position..];
    format!("{first}{to_add}{second}")
}

pub fn input(prompt: Option<String>, default: Option<String>) -> crate::Result<String> {
    let real_prompt = prompt.unwrap_or("> ".to_string());
    read()?;
    print!("{real_prompt}");
    io::stdout().flush()?;
    // entering raw mode
    enable_raw_mode()?;
    let mut choice = default.unwrap_or_default();
    // key detection
    let mut cursor_position: usize = choice.len();
    loop {
        print!("\x1b[2K");
        print!("\x1b[1000D");
        print!("{FORE_LIGHTMAGENTA}{real_prompt}{FORE_BLUE}{choice}");
        print!("\x1b[1000D");
        print!("\x1b[{}C", cursor_position + real_prompt.len());
        io::stdout().flush()?;
        read()?;
        let r = read()?;
        // matching the key
        match r {
            Event::Key(KeyEvent {
                code: KeyCode::Down, ..
            }) => {
                cursor_position = choice.len();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => {
                cursor_position = 0
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left, ..
            }) => {
                if cursor_position > 0 {
                    cursor_position -= 1;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right, ..
            }) => {
                if cursor_position < choice.len() {
                    cursor_position += 1;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                disable_raw_mode()?;
                println!("{FORE_RESET}");
                panic!("Control-C pressed");
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Delete | KeyCode::Backspace,
                ..
            }) => {
                if choice.len() != 0  {
                    choice = format!("{}{}", &choice[..cursor_position-1], &choice[cursor_position..]);
                    cursor_position -= 1;
                }
            },
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => {
                choice = modify(choice, &c.to_string(), cursor_position);
                cursor_position += 1;
            }
            Event::Paste(s) => {
                choice = modify(choice, &s, cursor_position);
                cursor_position += s.len();
            }
            _ => {},
        }
        thread::sleep(Duration::from_millis(10));
    }
    print!("\x1b[2K");
    print!("\x1b[1000D");
    println!("{FORE_LIGHTMAGENTA}{real_prompt}{FORE_GREEN}{choice}{FORE_RESET}");
    // disabling raw mode
    disable_raw_mode()?;
    Ok(choice)
}
