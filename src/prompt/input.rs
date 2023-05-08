use ansi::{FORE_BLUE, FORE_GREEN, FORE_LIGHTMAGENTA, FORE_RESET};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::Write;
use std::time::Duration;
use std::{io, thread};

fn modify(choice: String, to_add: &str, position: usize) -> String {
    let first = &choice[..position];
    let second = &choice[position..];
    format!("{first}{to_add}{second}")
}

pub fn input(prompt: Option<String>, default: Option<String>) -> crate::Result<String> {
    let real_prompt = prompt.unwrap_or_else(|| "> ".to_string());
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
                code: KeyCode::Down,
                ..
            }) => {
                cursor_position = choice.len();
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => cursor_position = 0,
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => {
                cursor_position = cursor_position.saturating_sub(1);
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
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
                if !choice.is_empty() {
                    choice = format!(
                        "{}{}",
                        &choice[..cursor_position - 1],
                        &choice[cursor_position..]
                    );
                    cursor_position -= 1;
                }
            }
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
            _ => {}
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
