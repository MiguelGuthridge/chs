use crate::game::{Position, PieceType, Turn, Color};

use super::Board;

impl Board {
    /// Make a turn
    /// It is assumed that the move is legal
    pub fn make_turn(&mut self, turn: Turn) {
        // If a piece is captured, remove it
        if let Some(capture) = turn.capture {
            let captured = std::mem::replace(&mut self.squares[capture.pos()], None)
                .expect("Capture non-existent piece");
            self.captures.push(captured);
            self.squares[capture.pos()] = None;
            self.half_move_clock.push(-1);
        }
        // If it's a pawn push, but not a capture, record that
        if turn.kind == PieceType::Pawn && turn.capture.is_none() {
            // If it's a two-move push, set the en passant target
            if (turn.to.row() - turn.from.row()).abs() == 2 {
                self.en_passant_target = Some(Position::new(
                    (turn.to.row() + turn.from.row()) / 2,
                    turn.from.col(),
                ));
            } else {
                self.en_passant_target = None;
            }
            self.half_move_clock.push(-1);
        } else {
            self.en_passant_target = None;
        }
        // Lift the main piece
        let mut piece = std::mem::replace(&mut self.squares[turn.from.pos()], None)
            .expect("Move non-existent piece");
        // Lift and place the second piece
        if let Some((from, to)) = turn.additional_move {
            let secondary_piece = std::mem::replace(&mut self.squares[from.pos()], None)
                .expect("Non-existent additional piece");
            assert!(self.squares[to.pos()].is_none());
            self.squares[to.pos()] = Some(secondary_piece);
        }

        // If the piece is promoting, make that adjustment
        if let Some(promo_kind) = turn.promote_to {
            piece.kind = promo_kind;
        }

        // Increment that piece's move count
        piece.move_count += 1;

        // Now place the main piece into the correct square
        assert!(self.squares[turn.to.pos()].is_none(), "{}\n{}", self, turn);
        self.squares[turn.to.pos()] = Some(piece);

        // And store the turn into the turn history and change whose turn it is
        *self.half_move_clock.last_mut().unwrap() += 1;
        self.moves.push(turn);
        self.whose_turn = !self.whose_turn;
        if self.whose_turn == Color::White {
            self.num_moves += 1;
        }
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
        self.whose_turn = !self.whose_turn;

        // Check the move before this to handle the en passant target
        if let Some(prev_turn) = self.moves.last() {
            if prev_turn.kind == PieceType::Pawn
                && (prev_turn.to.row() - prev_turn.from.row()).abs() == 2
            {
                self.en_passant_target = Some(Position::new(
                    (prev_turn.to.row() + prev_turn.from.row()) / 2,
                    prev_turn.from.col(),
                ));
            } else {
                self.en_passant_target = None;
            }
        } else {
            self.en_passant_target = None;
        }

        if self.half_move_clock.last() == Some(&0) {
            self.half_move_clock.pop();
        } else {
            *self.half_move_clock.last_mut().unwrap() -= 1;
        }
        if self.whose_turn == Color::Black {
            self.num_moves -= 1;
        }

        Some(turn)
    }
}
