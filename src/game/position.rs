use std::fmt::Display;

/// Represents a position on the chess board
pub struct Position {
    /// = row * 8 + column when zero-indexed
    pos: u8
}

impl Position {
    pub fn new(row: u8, col: u8) -> Position {
        Position { pos: row * 8 + col }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Rank => 1 - 8
        let rank = self.pos / 8 + 1;
        // File => A - H
        let file = (self.pos % 8 + b'A') as char;

        write!(f, "{}{}", file, rank)
    }
}
