use super::{position::Position, board::Board};

/// Represents a piece that can be on the chess board
pub trait Piece {
    /// Returns a vector of available moves for the piece
    fn get_moves(self, position: Position, board: Board) -> Vec<Position>;
}
