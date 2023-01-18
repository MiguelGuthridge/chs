use std::fmt::Display;

use super::Color;

/// Represents a position on the chess board
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position(i8);

impl Position {
    pub fn new(row: i8, col: i8) -> Position {
        assert!((0..8).contains(&row));
        assert!((0..8).contains(&col));
        Position(row * 8 + col)
    }

    /// Position, for indexing into a board
    pub fn pos(&self) -> usize {
        self.0 as usize
    }

    /// Rank (row), as 1 - 8
    pub fn rank(&self) -> i8 {
        self.row() + 1
    }

    /// Rank (row), as 0 - 8
    pub fn row(&self) -> i8 {
        self.0 / 8
    }

    /// File (column), as 'A' - 'H'
    pub fn file(&self) -> char {
        (self.col() as u8 + b'A') as char
    }

    /// File (column), as 0 - 8
    pub fn col(&self) -> i8 {
        self.0 % 8
    }

    // Color of the square
    pub fn color(&self) -> Color {
        match self.0 % 2 {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Bruh"),
        }
    }

    /// Get a new position as an offset
    pub fn offset(&self, row: i8, col: i8) -> Option<Self> {
        let y = self.row() - row;
        let x = self.col() - col;

        if !(0..8).contains(&x) || !(0..8).contains(&y) {
            None
        } else {
            Some(Self::new(y, x))
        }
    }
}

impl From<i8> for Position {
    fn from(i: i8) -> Self {
        assert!((0..64).contains(&i));
        Self(i)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}
