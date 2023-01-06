use super::{position::Position, board::Board};


pub trait Piece {
    fn get_moves(self, position: Position, board: Board) -> Vec<Position>;
}
