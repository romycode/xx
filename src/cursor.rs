 use std::fmt::{Debug, Formatter};

pub struct Origin {
    pub line: u32,
    pub column: u32,
}

impl Debug for Origin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "O({},{})", self.line, self.column)
    }
}

pub struct Size {
    pub lines: u32,
    pub columns: u32,
}

impl Debug for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "S({},{})", self.lines, self.columns)
    }
}

type Limit = Size;

pub struct Cursor {
    pub line: u32,
    pub column: u32,
    pub origin: Origin,
    pub limit: Limit,
}

impl Debug for Cursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "C(l:{} c:{} {:?} {:?})",
            self.line, self.column, self.origin, self.limit
        )
    }
}

impl Cursor {
    pub fn new(origin: Origin, size: Size) -> Self {
        Cursor {
            line: origin.line,
            column: origin.column,
            limit: Limit {
                lines: origin.line + size.lines,
                columns: origin.column + size.columns,
            },
            origin,
        }
    }

    pub fn down(&mut self) {
        if self.line < self.limit.lines {
            self.line += 1;
        }
    }

    pub fn up(&mut self) {
        if self.line > self.origin.line {
            self.line -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.column < self.limit.columns {
            self.column += 1;
        }
    }

    pub fn left(&mut self) {
        if self.column > self.origin.column {
            self.column -= 1;
        }
    }

    pub fn set_first_column(&mut self) {
        self.column = self.origin.column;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_up_not_below_zero() {
        let mut sut = create_cursor();

        sut.up();

        assert_eq!("C(l:0 c:0 O(0,0) S(1,1))", format!("{:?}", sut));
    }

    #[test]
    fn move_up_decrease_line() {
        let mut sut = create_cursor();

        sut.down();
        sut.up();

        assert_eq!("C(l:0 c:0 O(0,0) S(1,1))", format!("{:?}", sut));
    }

    #[test]
    fn move_down_increase_line() {
        let mut sut = create_cursor();

        sut.down();

        assert_eq!("C(l:1 c:0 O(0,0) S(1,1))", format!("{:?}", sut));
    }

    #[test]
    fn move_down_not_overflow_lines() {
        let mut sut = create_cursor();

        sut.down();
        sut.down();

        assert_eq!("C(l:1 c:0 O(0,0) S(1,1))", format!("{:?}", sut));
    }

    #[test]
    fn move_left_not_below_zero() {
        let mut sut = create_cursor();

        sut.left();

        assert_eq!("C(l:0 c:0 O(0,0) S(1,1))", format!("{:?}", sut));
    }

    #[test]
    fn move_left_decrease_column() {
        let mut sut = create_cursor();

        sut.right();
        sut.left();

        assert_eq!("C(l:0 c:0 O(0,0) S(1,1))", format!("{:?}", sut));
    }

    #[test]
    fn move_right_increase_column() {
        let mut sut = create_cursor();

        sut.right();

        assert_eq!("C(l:0 c:1 O(0,0) S(1,1))", format!("{:?}", sut));
    }

    #[test]
    fn move_right_not_overflow_lines() {
        let mut sut = create_cursor();

        sut.right();
        sut.right();

        assert_eq!("C(l:0 c:1 O(0,0) S(1,1))", format!("{:?}", sut));
    }

    #[test]
    fn move_up_respect_origin_line() {
        let mut sut = create_cursor_with_origin_five();

        sut.up();
        sut.up();

        assert_eq!("C(l:5 c:5 O(5,5) S(6,6))", format!("{:?}", sut));
    }

    #[test]
    fn move_down_respect_origin_line() {
        let mut sut = create_cursor_with_origin_five();

        sut.down();
        sut.down();

        assert_eq!("C(l:6 c:5 O(5,5) S(6,6))", format!("{:?}", sut));
    }

    #[test]
    fn move_left_respect_origin_line() {
        let mut sut = create_cursor_with_origin_five();

        sut.left();
        sut.left();

        assert_eq!("C(l:5 c:5 O(5,5) S(6,6))", format!("{:?}", sut));
    }

    #[test]
    fn move_right_respect_origin_line() {
        let mut sut = create_cursor_with_origin_five();

        sut.right();
        sut.right();

        assert_eq!("C(l:5 c:6 O(5,5) S(6,6))", format!("{:?}", sut));
    }

    #[test]
    fn set_first_colum_move_cursor_to_column_zero() {
        let mut sut = create_cursor_with_origin_five();

        sut.right();
        sut.set_first_column();

        assert_eq!("C(l:5 c:5 O(5,5) S(6,6))", format!("{:?}", sut));
    }

    fn create_cursor() -> Cursor {
        Cursor::new(
            Origin { line: 0, column: 0 },
            Size {
                lines: 1,
                columns: 1,
            },
        )
    }

    fn create_cursor_with_origin_five() -> Cursor {
        Cursor::new(
            Origin { line: 5, column: 5 },
            Size {
                lines: 1,
                columns: 1,
            },
        )
    }
}
