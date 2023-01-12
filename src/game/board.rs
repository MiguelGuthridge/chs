use arr_macro::arr;

use super::{piece::Piece, turn::Turn, Color, PieceType};

#[derive(Debug)]
pub struct Board {
    /// Pieces that have been captured
    captures: Vec<Piece>,

    /// 8x8 board
    squares: [Option<Piece>; 8 * 8],

    /// Whose turn it is to move
    turn: Color,

    /// Vector of moves
    moves: Vec<Turn>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            captures: Default::default(),
            squares: arr![None; 64],
            turn: Color::White,
            moves: Default::default(),
        }
    }
}

impl Board {
    /// Create a board in the starting position
    fn new_from_start() -> Self {
        let mut board = Self::default();

        let piece_order = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        // Pieces
        for (piece, i) in piece_order.iter().zip(0..8) {
            board.squares[i] = Some(Piece::new(*piece, Color::White));
        }
        for (piece, i) in piece_order.iter().zip(56..64) {
            board.squares[i] = Some(Piece::new(*piece, Color::Black));
        }
        // Pawns
        for i in 8..16 {
            board.squares[i] = Some(Piece::new(PieceType::Pawn, Color::White));
        }
        for i in 48..56 {
            board.squares[i] = Some(Piece::new(PieceType::Pawn, Color::Black));
        }

        board
    }

    /// Make a turn
    /// It is assumed that the move is legal
    fn make_turn(&mut self, turn: Turn) {
        // If a piece is captured, remove it
        if let Some(capture) = turn.capture {
            let captured = std::mem::replace(&mut self.squares[capture.pos()], None)
                .expect("Capture non-existent piece");
            self.captures.push(captured);
            self.squares[capture.pos()] = None;
        }
        // Lift the main piece
        let piece = std::mem::replace(&mut self.squares[turn.from.pos()], None)
            .expect("Move non-existent piece");
        // Lift and place the second piece
        if let Some((from, to)) = turn.additional_move {
            let secondary_piece = std::mem::replace(&mut self.squares[from.pos()], None)
                .expect("Non-existent additional piece");
            self.squares[to.pos()] = Some(secondary_piece);
        }

        // Now place the main piece into the correct square
        self.squares[turn.to.pos()] = Some(piece);

        // And store the turn into the turn history
        self.moves.push(turn);
    }

    /// Undo the last turn
    /// Return it, or None if there is nothing to undo
    fn undo_turn(&mut self) -> Option<Turn> {
        let turn = self.moves.pop()?;
        // Lift piece from the expected place
        let piece = std::mem::replace(&mut self.squares[turn.to.pos()], None)
            .expect("Undo move non-existent piece");
        // Lift and place the second piece
        if let Some((from, to)) = turn.additional_move {
            let secondary_piece = std::mem::replace(&mut self.squares[to.pos()], None)
                .expect("Non-existent additional piece");
            self.squares[from.pos()] = Some(secondary_piece);
        }

        // Add back any captured piece
        if let Some(capture) = turn.capture {
            self.squares[capture.pos()] = self.captures.pop();
        }

        Some(turn)
    }
}
