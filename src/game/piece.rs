use std::fmt::Display;

use super::{Color, Position, Board, turn::Turn};

/// Enum representing all possible kinds of pieces
#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PieceType::King => "K",
                PieceType::Queen => "Q",
                PieceType::Rook => "R",
                PieceType::Bishop => "B",
                PieceType::Knight => "N",
                PieceType::Pawn => "P",
            }
        )
    }
}

/// Represents a piece on the board
#[derive(Debug)]
pub struct  Piece {
    pub kind: PieceType,
    color: Color,
    has_moved: bool,
}

impl Piece {
    pub fn new(kind: PieceType, color: Color) -> Self {
        Self { kind, color, has_moved: false }
    }

    pub fn get_moves(&self, pos: Position, board: &Board) -> Vec<Turn> {
        todo!()
    }
}
