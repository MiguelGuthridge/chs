use std::fmt::{Debug, Display};

use super::{board::FenError, Color};

/// Represents a position on the chess board
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position(i8);

impl Position {
    pub fn new(row: i8, col: i8) -> Self {
        assert!((0..8).contains(&row));
        assert!((0..8).contains(&col));
        Position(row * 8 + col)
    }

    /// Create a position from a FEN string
    pub fn from_fen(fen_pos: &str) -> Result<Option<Self>, FenError> {
        if fen_pos == "-" {
            return Ok(None);
        }
        let chars: Vec<char> = fen_pos.chars().collect();
        if chars.len() != 2 {
            return Err(FenError::InvalidPosition(fen_pos.to_string()));
        }
        let col_char = chars[0].to_ascii_lowercase();
        let row_char = chars[1];

        if !('a'..='h').contains(&col_char) || !('1'..='8').contains(&row_char) {
            return Err(FenError::InvalidPosition(fen_pos.to_string()));
        }

        let row = row_char as u8 - b'1';
        let col = col_char as u8 - b'a';

        Ok(Some(Self::new(row as i8, col as i8)))
    }

    /// Position from 0..64, for indexing into a board
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
        let y = self.row() + row;
        let x = self.col() + col;

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

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position(row={}, col={})", self.row(), self.col())
    }
}
