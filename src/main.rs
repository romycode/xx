use std::io;

use crossterm::event;

use term::Term;

use crate::buffer::{Buffer, Move};

mod buffer;
mod term;

fn main() {
    let stdout = io::stdout();
    let mut term = Term::new(&stdout);
    term.enable_raw();

    let mut max_lines = crossterm::terminal::size().unwrap().0;
    if max_lines == 0 {
        max_lines = 100;
    }

    let mut buffer = Buffer::new();

    loop {
        term.clear();
        term.move_cursor(0, max_lines - 1);
        term.print(&buffer.test());
        term.move_cursor(0, 0);
        term.print(&buffer.content.iter().collect::<String>().split('\n').collect::<Vec<_>>().join("\r\n"));
        term.move_cursor(buffer.column as u16, buffer.line as u16);
        term.flush();

        if let Ok(event) = term.event() {
            match event {
                event::Event::Key(key) => match key {
                    event::KeyEvent {
                        code: event::KeyCode::Char('q'),
                        modifiers: event::KeyModifiers::ALT,
                        ..
                    } => break,
                    event::KeyEvent {
                        code: event::KeyCode::Up,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => {
                        buffer.mv(Move::Up, false, false);
                    }
                    event::KeyEvent {
                        code: event::KeyCode::Down,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => {
                        buffer.mv(Move::Down, false, false);
                    }
                    event::KeyEvent {
                        code: event::KeyCode::Left,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => {
                        buffer.mv(Move::Left, false, true);
                    }
                    event::KeyEvent {
                        code: event::KeyCode::Right,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => {
                        buffer.mv(Move::Right, false, true);
                    }
                    event::KeyEvent {
                        code: event::KeyCode::Enter,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => {
                        buffer.insert('\n');
                    }
                    event::KeyEvent {
                        code: event::KeyCode::Backspace,
                        modifiers: event::KeyModifiers::NONE,
                        ..
                    } => {
                        buffer.remove();
                    }
                    event::KeyEvent {
                        code: event::KeyCode::Char(c),
                        modifiers: event::KeyModifiers::NONE | event::KeyModifiers::SHIFT,
                        ..
                    } => {
                        buffer.insert(c);
                    }
                    _ => {}
                },
                event::Event::FocusGained => {}
                event::Event::FocusLost => {}
                event::Event::Mouse(_) => {}
                event::Event::Paste(_) => {}
                event::Event::Resize(_, _) => {}
            }
        }
    }
}