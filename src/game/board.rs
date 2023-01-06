use super::piece::Piece;

/// Which player needs to make their move next
pub enum ToMove {
    White,
    Black,
}

pub struct Board {
    /// 8x8 board
    board: [[Option<Box<dyn Piece>>; 8]; 8],

    /// Whose turn it is to move
    turn: ToMove,
}
