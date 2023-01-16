use std::ops::Not;

/// Which player needs to make their move next
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// Returns the index of the row that is home for this color
    pub fn get_home(self) -> i8 {
        match self {
            Color::White => 0,
            Color::Black => 7,
        }
    }

    /// Returns a row offset representing forwards for this
    pub fn get_direction(self) -> i8 {
        match self {
            Color::White => 1,
            Color::Black => -1,
        }
    }
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
