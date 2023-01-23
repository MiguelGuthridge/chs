use std::fmt::Display;

use super::{Board, Color, Position};

/// Enum representing all possible kinds of pieces
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

pub const PROMOTABLE_TYPES: [PieceType; 4] = [
    PieceType::Queen,
    PieceType::Rook,
    PieceType::Bishop,
    PieceType::Knight,
];

pub const KNIGHT_MOVES: [(i8, i8); 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
];

impl Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PieceType::King => "King",
                PieceType::Queen => "Queen",
                PieceType::Rook => "Rook",
                PieceType::Bishop => "Bishop",
                PieceType::Knight => "Knight",
                PieceType::Pawn => "Pawn",
            }
        )
    }
}

/// Represents a piece on the board
#[derive(Debug, Clone)]
pub struct Piece {
    pub kind: PieceType,
    pub color: Color,
    pub move_count: i32,
}

impl Piece {
    pub fn new(kind: PieceType, color: Color) -> Self {
        Self {
            kind,
            color,
            move_count: 0,
        }
    }

    /// Returns whether the piece could move here on an empty board.
    ///
    /// This ignores checks, captures, and pieces in the way, as they are dealt
    /// with elsewhere; except for pawns due to the complex nature of their
    /// captures
    pub fn could_move_to(&self, from: Position, to: Position, board: &Board) -> bool {
        if from == to {
            false
        } else {
            match self.kind {
                PieceType::King => self.could_king_move_to(from, to),
                PieceType::Queen => self.could_queen_move_to(from, to),
                PieceType::Rook => self.could_rook_move_to(from, to),
                PieceType::Bishop => self.could_bishop_move_to(from, to),
                PieceType::Knight => self.could_knight_move_to(from, to),
                PieceType::Pawn => self.could_pawn_move_to(from, to, board),
            }
        }
    }

    fn could_king_move_to(&self, from: Position, to: Position) -> bool {
        (from.row() - to.row()).abs() <= 1 && (from.col() - to.col()).abs() <= 1
    }

    fn could_rook_move_to(&self, from: Position, to: Position) -> bool {
        from.row() == to.row() || from.col() == to.row()
    }

    fn could_bishop_move_to(&self, from: Position, to: Position) -> bool {
        from.row() - from.col() == to.row() - to.col()
            || from.row() + from.col() == to.row() + to.col()
    }

    fn could_queen_move_to(&self, from: Position, to: Position) -> bool {
        self.could_rook_move_to(from, to) || self.could_bishop_move_to(from, to)
    }

    fn could_knight_move_to(&self, from: Position, to: Position) -> bool {
        let row_diff = (from.row() - to.row()).abs();
        let col_diff = (from.col() - to.col()).abs();
        row_diff == 2 && col_diff == 1 || row_diff == 1 && col_diff == 2
    }

    /// God I hate pawns, why are they so god damn complex
    fn could_pawn_move_to(&self, from: Position, to: Position, board: &Board) -> bool {
        // If the row or col are too far off, don't even bother checking
        let col_diff = (from.col() - to.col()).abs();
        if col_diff >= 2 {
            return false;
        }
        let row_diff = from.row() - to.row();
        // If they're moving in the wrong direction
        if row_diff * self.color.get_direction() <= 0 {
            return false;
        }
        // Or if we're not on the home row and we're not moving one square
        if from.row() != self.color.get_home() + self.color.get_direction() && row_diff.abs() != 1 {
            return false;
        }
        // Or if we're trying to move more than two squares on the home row
        if row_diff.abs() > 2 {
            return false;
        }
        // If there's a piece in front of us and we're moving directly forwards
        if board.at_position(to).is_some() && col_diff == 0 {
            return false;
        }
        // If there's no piece where we're going and we're moving diagonally
        if board.at_position(to).is_none() && col_diff == 1 {
            // But if it was en passant
            if let Some(turn) = board.get_prev_turn() {
                if turn.kind == PieceType::Pawn
                    && to.row() == turn.to.row()
                    && to.row() + self.color.get_direction() * 2 == turn.from.row()
                    && (to.col() - turn.to.col()).abs() == 1
                {
                    return true;
                }
            }
            return false;
        }
        true
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.color, self.kind)?;
        Ok(())
    }
}
