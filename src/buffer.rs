pub enum Move {
    Up,
    Left,
    Down,
    Right,
}

pub struct Buffer {
    pub content: Vec<char>,
    pub line: usize,
    pub column: usize,
    pub cursor: usize,
    pub lines: Vec<usize>,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            content: vec![],
            line: 0,
            column: 0,
            cursor: 0,
            lines: vec![0],
        }
    }
    pub fn test(&self) -> String {
        format!(
            "buffer_pos:{} line:{} column:{} lines:{:?}",
            self.cursor, self.line, self.column, self.lines
        )
    }
    fn move_cursor(&mut self, cursor: usize) {
        self.cursor = cursor;
        self.line = 0;
        for (i, line) in self.lines.iter().enumerate() {
            if *line > self.cursor {
                self.line = i;
                break;
            }
        }

        if self.line == 0 {
            self.column = self.cursor;
            return;
        }
        self.column = match self.cursor.abs_diff(self.lines[self.line - 1]) {
            0 => 0,
            diff => diff
        };
    }
    fn move_chars_end(&mut self) {
        for i in (self.cursor..self.content.len()).rev() {
            if i == 0 {
                continue;
            }
            self.content[i] = self.content[i - 1];
        }
    }
    fn current_line_cols(&mut self) -> usize {
        if self.line == 0 {
            return self.lines[self.line];
        }
        match self.lines[self.line] - self.lines[self.line - 1] {
            0 => 0,
            _ => self.lines[self.line] - self.lines[self.line - 1],
        }
    }
    pub fn insert_at(&mut self, cursor: usize, c: char) {
        self.move_cursor(cursor);
        self.insert(c);
    }
    pub fn insert(&mut self, c: char) {
        self.content.push(' ');
        self.move_chars_end(); // leave cursor position blank

        self.content[self.cursor] = c;
        self.cursor += 1;
        self.column += 1;
        self.lines[self.line] += 1;
        for i in self.line + 1..self.lines.len() { self.lines[i] += 1; }

        if '\n' == c {
            let prev_end = self.lines[self.line];
            self.lines[self.line] = self.cursor;
            self.column = 0;
            self.line += 1;
            self.lines.insert(self.line, prev_end);
        }
    }
    pub fn remove_at(&mut self, cursor: usize) {
        self.move_cursor(cursor + 1);
        self.remove();
    }
    pub fn remove(&mut self) {
        if self.cursor == 0 { return; }
        let removed = self.content.remove(self.cursor - 1);
        self.cursor -= 1;
        if 0 < self.column { self.column -= 1; }
        self.lines[self.line] -= 1;
        for i in self.line + 1..self.lines.len() { self.lines[i] -= 1; }

        if '\n' == removed {
            self.line -= 1;
            self.cursor = self.lines[self.line] - 1;
            self.column = self.current_line_cols() - 1;
            self.lines[self.line] = self.lines.remove(self.line + 1);
        }
    }
    pub fn mv(&mut self, mv: Move, start: bool, end: bool) {
        if self.content.len() == 0 { return; }
        match mv {
            Move::Up => {
                if 0 == self.line { return; }
                self.line -= 1;

                let curr_line_columns = self.current_line_cols();
                if curr_line_columns < self.column + 1 {
                    self.column = curr_line_columns - 1;
                }
                self.cursor = match self.line {
                    0 => self.column,
                    v => self.lines[v - 1] + self.column
                };

                if end {
                    self.cursor = self.lines[self.line] - 1;
                    self.column = self.current_line_cols() - 1;
                    return;
                }
                if start {
                    if 0 == self.line {
                        self.cursor = 0;
                        self.column = 0;
                        return;
                    }
                    self.cursor = self.lines[self.line - 1];
                    self.column = 0;
                    return;
                }
            }
            Move::Down => {
                if self.line == self.lines.len() - 1 { return; }
                self.line += 1;

                let curr_line_columns = self.current_line_cols();
                if curr_line_columns < self.column + 1 {
                    self.column = curr_line_columns - 1;
                }
                self.cursor = match self.line {
                    0 => self.column,
                    v => self.lines[v - 1] + self.column
                };

                if end {
                    if self.line + 1 == self.lines.len() {
                        self.column += 1;
                    }
                    self.cursor = self.lines[self.line];
                    self.column = curr_line_columns;
                }
                if start {
                    self.cursor = self.lines[self.line - 1];
                    self.column = 0;
                }
            }
            Move::Left => {
                if 0 < self.column {
                    self.column -= 1;
                    self.cursor -= 1;
                    return;
                }
                self.mv(Move::Up, false, true);
            }
            Move::Right => {
                if self.column < self.current_line_cols() {
                    let prev_column = self.column;
                    self.column += 1;
                    self.cursor += 1;
                    if prev_column == self.current_line_cols() - 1 {
                        self.mv(Move::Down, true, false)
                    }
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_char() {
        let mut buffer = Buffer::new();
        buffer.insert('a');
        assert_eq!("a", buffer.content.iter().collect::<String>());
        assert_eq!("buffer_pos:1 line:0 column:1 lines:[1]", buffer.test());
    }

    #[test]
    fn test_remove_char() {
        let mut buffer = Buffer::new();
        buffer.insert('a');
        assert_eq!("a", buffer.content.iter().collect::<String>());
        assert_eq!("buffer_pos:1 line:0 column:1 lines:[1]", buffer.test());

        buffer.remove();
        assert_eq!("", buffer.content.iter().collect::<String>());
        assert_eq!("buffer_pos:0 line:0 column:0 lines:[0]", buffer.test());
    }

    #[test]
    fn test_insert_new_line() {
        let mut buffer = Buffer::new();
        buffer.insert('\n');
        assert_eq!("\n", buffer.content.iter().collect::<String>());
        assert_eq!("buffer_pos:1 line:1 column:0 lines:[1, 1]", buffer.test());
    }

    #[test]
    fn test_remove_new_line() {
        let mut buffer = Buffer::new();
        buffer.insert('\n');
        assert_eq!("\n", buffer.content.iter().collect::<String>());
        assert_eq!("buffer_pos:1 line:1 column:0 lines:[1, 1]", buffer.test());

        buffer.remove();
        assert_eq!("", buffer.content.iter().collect::<String>());
        assert_eq!("buffer_pos:0 line:0 column:0 lines:[0]", buffer.test());
    }

    #[test]
    fn test_insert_new_line_should_split_line() {
        let mut buffer = Buffer::new();
        buffer.insert('a');
        buffer.insert('a');
        buffer.insert('a');
        buffer.insert('a');
        assert_eq!("aaaa", buffer.content.iter().collect::<String>());
        assert_eq!("buffer_pos:4 line:0 column:4 lines:[4]", buffer.test());

        buffer.insert_at(2, '\n');
        assert_eq!("aa\naa", buffer.content.iter().collect::<String>());
        assert_eq!("buffer_pos:3 line:1 column:0 lines:[3, 5]", buffer.test());
    }

    #[test]
    fn test_remove_new_line_should_join_lines() {
        let mut buffer = Buffer::new();
        buffer.insert('a');
        buffer.insert('a');
        buffer.insert('\n');
        buffer.insert('a');
        buffer.insert('a');
        assert_eq!("aa\naa", buffer.content.iter().collect::<String>());
        assert_eq!("buffer_pos:5 line:1 column:2 lines:[3, 5]", buffer.test());

        buffer.remove_at(2);
        assert_eq!("aaaa", buffer.content.iter().collect::<String>());
        assert_eq!("buffer_pos:2 line:0 column:2 lines:[4]", buffer.test());
    }

    #[test]
    fn test_move_cursor_update_line_and_column() {
        let mut buffer = Buffer::new();
        buffer.insert('a');
        buffer.insert('\n');
        buffer.insert('a');
        buffer.insert('a');
        buffer.insert('\n');
        buffer.insert('a');
        buffer.insert('a');
        buffer.insert('a');
        buffer.insert('\n');
        buffer.insert('a');
        buffer.insert('a');
        buffer.insert('a');
        buffer.insert('a');
        assert_eq!("a\naa\naaa\naaaa", buffer.content.iter().collect::<String>());
        assert_eq!("buffer_pos:13 line:3 column:4 lines:[2, 5, 9, 13]", buffer.test());

        buffer.move_cursor(1);
        assert_eq!(2, buffer.current_line_cols());
        buffer.move_cursor(2);
        assert_eq!(3, buffer.current_line_cols());
        buffer.move_cursor(5);
        assert_eq!(4, buffer.current_line_cols());
    }

    #[test]
    fn test_current_line_cols_works_ok() {
        let mut buffer = Buffer::new();
        buffer.insert('a');
        buffer.insert('\n');
        buffer.insert('a');
        buffer.insert('a');
        buffer.insert('\n');
        buffer.insert('a');
        buffer.insert('a');
        buffer.insert('a');
        buffer.insert('\n');
        buffer.insert('a');
        buffer.insert('a');
        buffer.insert('a');
        buffer.insert('a');
        assert_eq!("a\naa\naaa\naaaa", buffer.content.iter().collect::<String>());
        assert_eq!("buffer_pos:13 line:3 column:4 lines:[2, 5, 9, 13]", buffer.test());

        buffer.move_cursor(1);
        assert_eq!(2, buffer.current_line_cols());
        buffer.move_cursor(2);
        assert_eq!(3, buffer.current_line_cols());
        buffer.move_cursor(5);
        assert_eq!(4, buffer.current_line_cols());
    }

    #[test]
    fn test_mv_right_works_ok() {
        let mut buffer = build_test_buffer("aa\naaaa");
        assert_eq!("aa\naaaa", buffer.content.iter().collect::<String>());
        assert_eq!("buffer_pos:7 line:1 column:4 lines:[3, 7]", buffer.test());

        buffer.move_cursor(0);
        assert_eq!("buffer_pos:0 line:0 column:0 lines:[3, 7]", buffer.test());

        buffer.mv(Move::Right, false, false);
        assert_eq!("buffer_pos:1 line:0 column:1 lines:[3, 7]", buffer.test());
        buffer.mv(Move::Right, false, false);
        assert_eq!("buffer_pos:2 line:0 column:2 lines:[3, 7]", buffer.test());
        buffer.mv(Move::Right, false, false);
        assert_eq!("buffer_pos:3 line:1 column:0 lines:[3, 7]", buffer.test());
        buffer.mv(Move::Right, false, false);
        assert_eq!("buffer_pos:4 line:1 column:1 lines:[3, 7]", buffer.test());
    }

    #[test]
    fn test_mv_left_works_ok() {
        let mut buffer = build_test_buffer("aaaa\naa");
        assert_eq!("aaaa\naa", buffer.content.iter().collect::<String>());
        assert_eq!("buffer_pos:7 line:1 column:2 lines:[5, 7]", buffer.test());

        buffer.mv(Move::Left, false, false);
        assert_eq!("buffer_pos:6 line:1 column:1 lines:[5, 7]", buffer.test());
        buffer.mv(Move::Left, false, false);
        assert_eq!("buffer_pos:5 line:1 column:0 lines:[5, 7]", buffer.test());
        buffer.mv(Move::Left, false, false);
        assert_eq!("buffer_pos:4 line:0 column:4 lines:[5, 7]", buffer.test());
        buffer.mv(Move::Left, false, false);
        assert_eq!("buffer_pos:3 line:0 column:3 lines:[5, 7]", buffer.test());
    }

    #[test]
    fn test_mv_down_works_ok() {
        let mut buffer = build_test_buffer("aaaa\naa\naaa");
        assert_eq!("aaaa\naa\naaa", buffer.content.iter().collect::<String>());
        assert_eq!("buffer_pos:11 line:2 column:3 lines:[5, 8, 11]", buffer.test());

        buffer.move_cursor(3);

        buffer.mv(Move::Down, false, false);
        assert_eq!("buffer_pos:8 line:1 column:3 lines:[5, 8, 11]", buffer.test());
        buffer.mv(Move::Down, false, false);
        assert_eq!("buffer_pos:11 line:2 column:3 lines:[5, 8, 11]", buffer.test());
    }

    #[test]
    fn test_mv_up_works_ok() {
        let mut buffer = build_test_buffer("aaaa\naa\naaa");
        assert_eq!("aaaa\naa\naaa", buffer.content.iter().collect::<String>());
        assert_eq!("buffer_pos:11 line:2 column:3 lines:[5, 8, 11]", buffer.test());

        buffer.mv(Move::Up, false, false);
        assert_eq!("buffer_pos:8 line:1 column:3 lines:[5, 8, 11]", buffer.test());
        buffer.mv(Move::Up, false, false);
        assert_eq!("buffer_pos:3 line:0 column:3 lines:[5, 8, 11]", buffer.test());

        buffer.move_cursor(6);
        assert_eq!("buffer_pos:6 line:1 column:1 lines:[5, 8, 11]", buffer.test());
        buffer.mv(Move::Up, false, true);
        assert_eq!("buffer_pos:4 line:0 column:4 lines:[5, 8, 11]", buffer.test());

        buffer.move_cursor(6);
        assert_eq!("buffer_pos:6 line:1 column:1 lines:[5, 8, 11]", buffer.test());
        buffer.mv(Move::Up, true, false);
        assert_eq!("buffer_pos:0 line:0 column:0 lines:[5, 8, 11]", buffer.test());
    }

    fn build_test_buffer(content: &str) -> Buffer {
        let mut buffer = Buffer::new();
        for i in content.chars() {
            buffer.insert(i);
        }
        buffer
    }
}
