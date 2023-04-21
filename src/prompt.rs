use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use std::thread;
use std::time::Duration;

fn draw(options: &Vec<String>, choice: usize, multiline: bool) -> String {
    assert!(options.len() > choice);
    let mut result = String::new();
    for (count, option) in options.iter().enumerate() {
        if count != choice {
            result += "\x1b[34m  ";
        } else {
            result += "\x1b[35m> \x1b[32m";
        }
        result += option;
        result += "\x1b[39m";
        if multiline {
            result += "\n"
        } else {
            result += " "
        }
    }
    result
}

pub fn choice(options: Vec<String>, default: usize, multiline: Option<bool>) -> usize {
    read().expect("Input Patching failed");
    let mut multiline_standard = true;
    if let Some(..) = multiline {
        multiline_standard = multiline.unwrap();
    }
    thread::sleep(Duration::from_millis(100));
    // entering raw mode
    enable_raw_mode().unwrap();
    let start_msg = draw(&options, default, multiline_standard);
    print!("{}", start_msg);
    let mut choice = default;
    //key detection
    loop {
        print!("\x1b[1000D");
        if multiline_standard {
            print!("\x1b[{}A", options.len());
        } else {
            print!("\x1b[1A");
        }
        print!("{}", draw(&options, choice, multiline_standard));
        read().expect("Patching failed");
        // matching the key
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => choice = choice.saturating_sub(1),
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            })
            | Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => choice = choice.saturating_add(1),
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => break,
            _ => (),
        }
        if choice >= options.len() {
            choice = options.len() - 1
        }
        thread::sleep(Duration::from_millis(10));
    }
    // disabling raw mode
    disable_raw_mode().unwrap();
    choice
}
