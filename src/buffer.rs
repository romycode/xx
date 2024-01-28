pub struct Buffer {
    content: Vec<char>,
    line: usize,
    column: usize,
    cursor: usize,
    lines: Vec<usize>,
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
    fn test(&self) -> String {
        format!("c:{} l:{} c:{} ls:{:?}", self.cursor, self.line, self.column, self.lines)
    }
    fn move_cursor(&mut self, cursor: usize) {
        self.cursor = cursor;
        self.line = 0;
        for (i, line) in self.lines.iter().enumerate() {
            if *line > self.line {
                break;
            }
            self.line = i;
        }
    }
    fn move_chars_end(&mut self) {
        for i in (self.cursor..self.content.len()).rev() {
            if i == 0 {
                continue;
            }
            self.content[i] = self.content[i - 1];
        }
    }
    fn move_lines_end(&mut self) {
        for i in (self.line..self.lines.len()).rev() {
            if i == 0 {
                continue;
            }
            self.lines[i] = self.lines[i - 1];
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
        for i in self.line + 1..self.lines.len() {
            self.lines[i] += 1;
        }

        if '\n' == c {
            self.lines.push(0);
            self.move_lines_end();
            let line_end = self.lines[self.line];
            self.lines[self.line] = self.cursor;
            if self.cursor < line_end {
                self.lines[self.line + 1] = line_end;
                self.lines[self.line] = self.cursor;
            }
            self.line += 1;
            self.column = 0;
        }
    }
    pub fn remove(&mut self) {
        let removed = self.content.remove(self.cursor - 1);
        self.cursor -= 1;
        if 0 < self.column {
            self.column -= 1;
        }
        self.lines[self.line] -= 1;
        for i in self.line + 1..self.lines.len() {
            self.lines[i] -= 1;
        }

        if '\n' == removed {
            self.line -= 1;
            if self.line == 0 {
                self.column = self.cursor;
            } else {
                self.column = self.lines[self.line] - self.cursor;
            }
            self.lines[self.line] = self.lines.remove(self.line + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_move_cursor() {
        let mut buff = Buffer::new();
        add_string_to_buffer(&mut buff, "albert\nromera");
        assert_eq!("albert\nromera", buff.content.iter().collect::<String>());
        assert_eq!(buff.cursor, 13);
        assert_eq!(buff.line, 1);
        buff.move_cursor(0);
        assert_eq!("albert\nromera", buff.content.iter().collect::<String>());
        assert_eq!(buff.cursor, 0);
        assert_eq!(buff.line, 0);
    }

    #[test]
    fn should_insert_char_at_cursor() {
        let mut buff = Buffer::new();
        assert_eq!("", buff.content.iter().collect::<String>());
        buff.insert('a');
        assert_eq!("a", buff.content.iter().collect::<String>());
    }

    #[test]
    fn should_insert_new_line() {
        let mut buff = Buffer::new();
        add_string_to_buffer(&mut buff, "albert");
        assert_eq!("albert", buff.content.iter().collect::<String>());
        buff.insert('\n');
        assert_eq!("albert\n", buff.content.iter().collect::<String>());
    }

    #[test]
    fn should_split_line_in_two() {
        let mut buff = Buffer::new();
        add_string_to_buffer(&mut buff, "albert");
        assert_eq!("c:6 l:0 c:6 ls:[6]", buff.test());
        assert_eq!("albert", buff.content.iter().collect::<String>());
        buff.insert_at(3, '\n');
        assert_eq!("c:4 l:1 c:0 ls:[4, 7]", buff.test());
        assert_eq!("alb\nert", buff.content.iter().collect::<String>());
    }

    #[test]
    fn should_remove_char_at_cursor() {
        let mut buff = Buffer::new();
        add_string_to_buffer(&mut buff, "albert");
        assert_eq!("c:6 l:0 c:6 ls:[6]", buff.test());
        assert_eq!("albert", buff.content.iter().collect::<String>());
        buff.remove();
        assert_eq!("c:5 l:0 c:5 ls:[5]", buff.test());
        assert_eq!("alber", buff.content.iter().collect::<String>());
    }

    #[test]
    fn should_remove_new_line() {
        let mut buff = Buffer::new();
        add_string_to_buffer(&mut buff, "albert\n");
        assert_eq!("c:7 l:1 c:0 ls:[7, 7]", buff.test());
        assert_eq!("albert\n", buff.content.iter().collect::<String>());
        buff.remove();
        assert_eq!("albert", buff.content.iter().collect::<String>());
    }

    fn add_string_to_buffer(buffer: &mut Buffer, content: &str) {
        for char in content.chars() {
            buffer.insert(char);
        }
    }
}
