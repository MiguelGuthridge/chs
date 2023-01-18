use arr_macro::arr;

use super::{
    piece::{Piece, KNIGHT_MOVES},
    turn::Turn,
    Color, PieceType, Position,
};

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
    pub fn new_from_start() -> Self {
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
    pub fn make_turn(&mut self, turn: Turn) {
        // If a piece is captured, remove it
        if let Some(capture) = turn.capture {
            let captured = std::mem::replace(&mut self.squares[capture.pos()], None)
                .expect("Capture non-existent piece");
            self.captures.push(captured);
            self.squares[capture.pos()] = None;
        }
        // Lift the main piece
        let mut piece = std::mem::replace(&mut self.squares[turn.from.pos()], None)
            .expect("Move non-existent piece");
        // Lift and place the second piece
        if let Some((from, to)) = turn.additional_move {
            let secondary_piece = std::mem::replace(&mut self.squares[from.pos()], None)
                .expect("Non-existent additional piece");
            self.squares[to.pos()] = Some(secondary_piece);
        }

        // If the piece is promoting, make that adjustment
        if let Some(promo_kind) = turn.promote_to {
            piece.kind = promo_kind;
        }

        // Increment that piece's move count
        piece.move_count += 1;

        // Now place the main piece into the correct square
        self.squares[turn.to.pos()] = Some(piece);

        // And store the turn into the turn history and change whose turn it is
        self.moves.push(turn);
        self.turn = !self.turn;
    }

    /// Undo the last turn
    /// Return it, or None if there is nothing to undo
    pub fn undo_turn(&mut self) -> Option<Turn> {
        let turn = self.moves.pop()?;
        // Lift piece from the expected place
        let mut piece = std::mem::replace(&mut self.squares[turn.to.pos()], None)
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

        // If the piece promoted, make that adjustment
        if let Some(promo_from) = turn.promote_from {
            piece.kind = promo_from;
        }

        // Decrement that piece's move count
        piece.move_count -= 1;

        // Place the main piece and change whose turn it is
        self.squares[turn.from.pos()] = Some(piece);
        self.turn = !self.turn;

        Some(turn)
    }

    /// Return a reference to the piece in a particular position
    pub fn at_position(&self, position: Position) -> &Option<Piece> {
        &self.squares[position.pos()]
    }

    /// Returns a reference to the previous turn
    pub fn get_prev_turn(&self) -> Option<&Turn> {
        if self.moves.is_empty() {
            None
        } else {
            Some(&self.moves[self.moves.len() - 1])
        }
    }

    /// Returns `true` if a piece of the given color is attacking the given
    /// position
    pub fn are_pieces_attacking(&self, position: Position, color: Color) -> bool {
        // Lines
        for r in [-1, 0, 1] {
            for c in [-1, 0, 1] {
                if r == 0 && c == 0 {
                    continue;
                }
                let mut pos = position;
                while let Some(p) = pos.offset(r, c) {
                    pos = p;
                    if let Some(piece) = self.at_position(pos) {
                        // If that piece is of the correct color and attacks
                        // this square
                        if piece.color == color && piece.could_move_to(pos, position, &self) {
                            return true;
                        }
                        // Otherwise, no other pieces in this line can attack
                        break;
                    }
                }
            }
        }

        // Knight positions
        // This sorta defeats the purpose of the implementation of
        // piece.could_knight_move_to, but at least it makes it more efficient
        for (r, c) in KNIGHT_MOVES {
            if let Some(pos) = position.offset(r, c) {
                if let Some(piece) = self.at_position(pos) {
                    if piece.kind == PieceType::Knight && piece.color == color {
                        return true;
                    }
                }
            }
        }

        false
    }
}
