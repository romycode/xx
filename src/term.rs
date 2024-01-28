use std::io::{Stdout, Write};

use crossterm::{Command, cursor, event, QueueableCommand, style};
use crossterm::terminal::{Clear, ClearType};

pub struct Term<'t> {
    stdout: &'t Stdout,
}

impl<'t> Drop for Term<'t> {
    fn drop(&mut self) { self.disable_raw(); }
}

impl<'t> Term<'t> {
    pub fn new(stdout: &'t Stdout) -> Self {
        Self { stdout }
    }
    pub fn enable_raw(&self) {
        crossterm::terminal::enable_raw_mode().ok();
    }

    pub fn disable_raw(&self) {
        crossterm::terminal::disable_raw_mode().ok();
    }

    pub fn queue(&mut self, command: impl Command) {
        self.stdout.queue(command).ok();
    }

    pub fn clear(&mut self) {
        self.queue(Clear(ClearType::All));
    }

    pub fn move_cursor(&mut self, column: u16, line: u16) {
        self.queue(cursor::MoveTo(column, line));
    }

    pub fn print(&mut self, content: &str) {
        self.queue(style::Print(content));
    }

    pub fn event(&self) -> std::io::Result<event::Event> {
        event::read()
    }

    pub fn flush(&mut self) {
        self.stdout.flush().ok();
    }
}