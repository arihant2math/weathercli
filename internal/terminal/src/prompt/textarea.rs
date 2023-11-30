use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::io::stdout;
use std::io::Result;
use std::io::Write;
use std::time::Duration;
use std::{io, thread};

pub fn textarea() -> Result<String> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    execute!(stdout(), cursor::MoveTo(0, 0))?;
    io::stdout().flush()?;
    let mut text: Vec<String> = Vec::new();
    text.push(String::new());
    let mut cursor_char: usize = 0;
    let mut cursor_line: usize = 0;
    let mut lagged_cursor_char: usize = 0;
    loop {
        read()?;
        // matching the key
        match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Home,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                cursor_char = 0;
            }

            Event::Key(KeyEvent {
                code: KeyCode::End,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                cursor_char = text[cursor_line].len() - 1;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                if 0 < cursor_line {
                    cursor_line = cursor_line.saturating_sub(1);
                    cursor_char = lagged_cursor_char.min(text[cursor_line].len());
                } else {
                    cursor_char = 0;
                }
            }

            Event::Key(KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                if text.len() - 1 > cursor_line {
                    cursor_line = cursor_line.saturating_add(1);
                    cursor_char = lagged_cursor_char.min(text[cursor_line].len());
                } else {
                    // Move to the end of the line
                    cursor_char = text[cursor_line].len();
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                if cursor_char > 0 {
                    cursor_char = cursor_char.saturating_sub(1);
                } else if cursor_line > 0 {
                    cursor_line = cursor_line.saturating_sub(1);
                    cursor_char = text[cursor_line].len();
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                if text[cursor_line].len() > cursor_char {
                    cursor_char = cursor_char.saturating_add(1);
                } else if text.len() - 1 > cursor_line {
                    cursor_line = cursor_line.saturating_add(1);
                    cursor_char = 0;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                if cursor_char > 0 {
                    if text[cursor_line].chars().nth(cursor_char - 1).unwrap().is_whitespace() {
                        cursor_char = cursor_char.saturating_sub(1);
                    }
                    while cursor_char > 0 && !text[cursor_line].chars().nth(cursor_char - 1).unwrap().is_whitespace() {
                        cursor_char = cursor_char.saturating_sub(1);
                    }
                } else if cursor_line > 0 {
                    cursor_line = cursor_line.saturating_sub(1);
                    cursor_char = text[cursor_line].len();
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                if text[cursor_line].len() > cursor_char {
                    if text[cursor_line].chars().nth(cursor_char).unwrap().is_whitespace() {
                        cursor_char = cursor_char.saturating_add(1);
                    }
                    while text[cursor_line].len() > cursor_char && !text[cursor_line].chars().nth(cursor_char).unwrap().is_whitespace() {
                        cursor_char = cursor_char.saturating_add(1);
                    }
                } else if text.len() - 1 > cursor_line {
                    cursor_line = cursor_line.saturating_add(1);
                    cursor_char = 0;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => break,
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                execute!(stdout(), LeaveAlternateScreen)?;
                disable_raw_mode()?;
                panic!("Ctrl+C pressed");
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {
                break;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char(ch),
                modifiers: KeyModifiers::NONE | KeyModifiers::SHIFT,
                ..
            }) => {
                text[cursor_line].insert(cursor_char, ch);
                execute!(stdout(), Clear(ClearType::CurrentLine), cursor::MoveToColumn(0))?;
                cursor_char += 1;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::CONTROL,
                ..
            }) => {}
            Event::Key(KeyEvent {
                code: KeyCode::Delete | KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                if cursor_char > 0 {
                    text[cursor_line].remove(cursor_char - 1);
                    cursor_char -= 1;
                } else if cursor_line > 0 {
                    let old_text = text.remove(cursor_line);
                    cursor_line -= 1;
                    cursor_char = text[cursor_line].len();
                    text[cursor_line].push_str(&old_text);
                }
                execute!(stdout(), Clear(ClearType::UntilNewLine))?;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            }) => {
                text.insert(
                    cursor_line + 1,
                    text[cursor_line][cursor_char..].to_string(),
                );
                text[cursor_line].truncate(cursor_char);
                cursor_line += 1;
                cursor_char = 0;
                execute!(stdout(), Clear(ClearType::UntilNewLine))?;
            }
            _ => (),
        }
        execute!(stdout(), cursor::MoveTo(0, 0))?;
        print!("{}", text.join("\n"));
        #[allow(clippy::cast_possible_truncation)]
        execute!(stdout(), Clear(ClearType::FromCursorDown), cursor::MoveTo(cursor_char as u16, cursor_line as u16))?;
        if text[cursor_line].len() > cursor_char || text[cursor_line].len() > lagged_cursor_char {
            lagged_cursor_char = cursor_char;
        }
        thread::sleep(Duration::from_millis(5));
        stdout().flush()?;
    }
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(String::new())
}
