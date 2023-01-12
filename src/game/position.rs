use std::fmt::Display;

use super::Color;

/// Represents a position on the chess board
#[derive(Debug, Clone, Copy)]
pub struct Position(u8);

impl Position {
    pub fn new(row: u8, col: u8) -> Position {
        Position(row * 8 + col)
    }

    /// Position, for indexing into a board
    pub fn pos(&self) -> usize {
        self.0 as usize
    }

    /// Rank (row), as 1 - 8
    pub fn rank(&self) -> u8 {
        self.0 / 8 + 1
    }

    /// File (column), as 'A' - 'H'
    pub fn file(&self) -> char {
        (self.0 % 8 + b'A') as char
    }

    // Color of the square
    pub fn color(&self) -> Color {
        match self.0 % 2 {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Bruh"),
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}
