use std::io::Stdout;

use crossterm::event;

use crate::buffer::{Buffer, Move};
use crate::term::Term;

pub struct Editor<'e> {
    term: Term<'e>,
    buff: Buffer,
}

impl<'e> Editor<'e> {
    pub fn new(stdout: &'e Stdout) -> Self {
        Self { term: Term::new(stdout), buff: Buffer::new() }
    }
    pub fn run(&mut self) {
        self.term.enable_raw();

        loop {
            self.term.clear();
            self.term.move_cursor(0, self.term.size.1 - 1);
            self.term.print(&self.buff.test());
            self.term.move_cursor(0, 0);
            self.term.print(&self.buff.content.iter().collect::<String>().split('\n').collect::<Vec<_>>().join("\r\n"));
            self.term.move_cursor(self.buff.column as u16, self.buff.line as u16);
            self.term.flush();

            if let Ok(event) = self.term.event() {
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
                            self.buff.mv(Move::Up, false, false);
                        }
                        event::KeyEvent {
                            code: event::KeyCode::Down,
                            modifiers: event::KeyModifiers::NONE,
                            ..
                        } => {
                            self.buff.mv(Move::Down, false, false);
                        }
                        event::KeyEvent {
                            code: event::KeyCode::Left,
                            modifiers: event::KeyModifiers::NONE,
                            ..
                        } => {
                            self.buff.mv(Move::Left, false, true);
                        }
                        event::KeyEvent {
                            code: event::KeyCode::Right,
                            modifiers: event::KeyModifiers::NONE,
                            ..
                        } => {
                            self.buff.mv(Move::Right, false, true);
                        }
                        event::KeyEvent {
                            code: event::KeyCode::Enter,
                            modifiers: event::KeyModifiers::NONE,
                            ..
                        } => {
                            self.buff.insert('\n');
                        }
                        event::KeyEvent {
                            code: event::KeyCode::Backspace,
                            modifiers: event::KeyModifiers::NONE,
                            ..
                        } => {
                            self.buff.remove();
                        }
                        event::KeyEvent {
                            code: event::KeyCode::Char(c),
                            modifiers: event::KeyModifiers::NONE | event::KeyModifiers::SHIFT,
                            ..
                        } => {
                            self.buff.insert(c);
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

        self.term.clear();
    }
}