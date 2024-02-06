use std::io::stdout;

use crate::editor::Editor;

mod buffer;
mod term;
mod editor;

fn main() {
    let stdout = stdout();
    Editor::new(&stdout).run();
}