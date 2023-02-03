use std::{ops::Not, fmt::Display};

use super::board::FenError;

/// Which player needs to make their move next
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// Get a color from the to move component of a FEN string
    pub fn from_fen(fen_color: &str) -> Result<Self, FenError> {
        match fen_color {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            &_ => Err(FenError::InvalidColor(fen_color.to_string())),
        }
    }

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

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Color::White => "White",
            Color::Black => "Black",
        })?;
        Ok(())
    }
}
