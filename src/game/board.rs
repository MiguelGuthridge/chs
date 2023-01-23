use arr_macro::arr;
use std::fmt::{Debug, Display};

use super::{
    piece::{Piece, KNIGHT_MOVES, PROMOTABLE_TYPES},
    turn::Turn,
    Color, PieceType, Position,
};

#[derive(Debug, Clone)]
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

        println!("{}", self);
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
    pub fn at_position(&self, position: Position) -> Option<&Piece> {
        self.squares[position.pos()].as_ref()
    }

    /// Return whose turn it is
    pub fn whose_turn(&self) -> Color {
        self.turn
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
                        if piece.color == color && piece.could_move_to(pos, position, self) {
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

    /// Returns whether a move is legal - ie whether the other player
    /// is capable of capturing the king after the move is made
    pub fn is_move_legal(&mut self, turn: Turn) -> bool {
        self.make_turn(turn);

        let mut valid = true;

        // Find our king and find if someone can attack it
        // This is pretty inefficient - improve this at some point
        for i in 0..64 {
            let pos = Position::from(i);
            if let Some(piece) = self.at_position(pos) {
                if piece.kind == PieceType::King && piece.color == !self.turn && self.are_pieces_attacking(pos, self.turn) {
                    valid = false;
                }
            }
        }

        self.undo_turn();

        valid
    }

    ///////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////

    /// Return the moves that can be legally made by a piece at the given
    /// square
    ///
    /// pos: current position of the piece
    pub fn get_moves(&mut self, pos: Position) -> Vec<Turn> {
        let kind = self.at_position(pos).expect("Piece not there").kind;
        match kind {
            PieceType::King => self.king_moves(pos),
            PieceType::Queen => self.queen_moves(pos),
            PieceType::Rook => self.rook_moves(pos),
            PieceType::Bishop => self.bishop_moves(pos),
            PieceType::Knight => self.knight_moves(pos),
            PieceType::Pawn => self.pawn_moves(pos),
        }
    }

    /// Returns whether it's possible to move this piece into the given square,
    /// as well as a reference to the piece there
    ///
    /// This returns true if the square is empty, or if it has a piece of the
    /// opposite color
    ///
    /// It does not account for the kind of piece this is
    fn get_turn_simple(&self, from: Position, to: Position) -> Option<Turn> {
        let this_piece = self.at_position(from).unwrap();
        if let Some(other_piece) = self.at_position(to) {
            if other_piece.color != this_piece.color {
                Some(Turn::new_capture(this_piece.kind, from, to))
            } else {
                None
            }
        } else {
            Some(Turn::new_basic(this_piece.kind, from, to))
        }
    }

    fn add_move_if_legal(&mut self, turn: Turn, moves: &mut Vec<Turn>) {
        if self.is_move_legal(turn.clone()) {
            moves.push(turn);
        }
    }

    /// Get moves in a line from the given directions
    fn line_moves(&mut self, pos: Position, directions: &[(i8, i8)]) -> Vec<Turn> {
        let mut moves = vec![];

        for (r_off, c_off) in directions {
            let mut new_pos = pos;
            while let Some(off_pos) = new_pos.offset(*r_off, *c_off) {
                new_pos = off_pos;
                if let Some(turn) = self.get_turn_simple(pos, new_pos) {
                    let was_capture = turn.capture.is_some();
                    self.add_move_if_legal(turn, &mut moves);

                    if was_capture {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        moves
    }

    fn rook_moves(&mut self, pos: Position) -> Vec<Turn> {
        self.line_moves(pos, &[(1, 0), (0, 1), (-1, 0), (0, -1)])
    }

    fn bishop_moves(&mut self, pos: Position) -> Vec<Turn> {
        self.line_moves(pos, &[(1, 1), (1, -1), (-1, -1), (-1, 1)])
    }

    fn queen_moves(&mut self, pos: Position) -> Vec<Turn> {
        self.line_moves(
            pos,
            &[
                (1, 0),
                (0, 1),
                (-1, 0),
                (0, -1),
                (1, 1),
                (1, -1),
                (-1, -1),
                (-1, 1),
            ],
        )
    }

    fn king_moves(&mut self, from_pos: Position) -> Vec<Turn> {
        let mut moves = vec![];
        for r in [-1, 0, 1] {
            for c in [-1, 0, 1] {
                if r != 0 || c != 0 {
                    if let Some(to_pos) = from_pos.offset(r, c) {
                        if let Some(turn) = self.get_turn_simple(from_pos, to_pos) {
                            self.add_move_if_legal(turn, &mut moves);
                        }
                    }
                }
            }
        }
        // Castling
        // Can't have moved, and must be on the first rank
        let piece = self.at_position(from_pos).unwrap();
        if piece.move_count == 0
            && from_pos.row() == if piece.color == Color::White { 0 } else { 7 }
        {
            self.castling_moves(from_pos, &mut moves);
        }
        moves
    }

    fn castling_moves(&mut self, from_pos: Position, moves: &mut Vec<Turn>) {
        // Find the rooks
        for (row, col, res_col) in [(0, 1, 6), (0, -1, 2)] {
            // Check each square for pieces
            let mut new_pos = from_pos;
            while let Some(pos) = new_pos.offset(row, col) {
                new_pos = pos;
                // If it contains a piece
                if let Some(other_piece) = self.at_position(new_pos) {
                    let this_piece = self.at_position(from_pos).unwrap();
                    // If it's our rook
                    if other_piece.kind == PieceType::Rook
                        && other_piece.color == this_piece.color
                        && other_piece.move_count == 0
                    {
                        // We might be able to castle
                        // Check up to the resultant square that nothing is
                        // under attack
                        let from = from_pos.col() + col;
                        let to = res_col - col;
                        let start = i8::min(from, to);
                        let stop = i8::max(from, to);
                        for c in start..stop {
                            let pos = Position::new(row, c);
                            // If a piece is attacking this square, castling
                            // isn't allowed on this side
                            if self.are_pieces_attacking(pos, !this_piece.color) {
                                break;
                            }
                        }

                        self.add_move_if_legal(
                            Turn::new_additional(
                                this_piece.kind,
                                (from_pos, Position::new(from_pos.row(), res_col)),
                                (new_pos, Position::new(from_pos.row(), res_col - col)),
                            ),
                            moves,
                        );
                    }
                }
            }
        }
    }

    fn knight_moves(&mut self, pos: Position) -> Vec<Turn> {
        let mut moves = vec![];

        for (r, c) in [
            // Is there a nicer way to do this?
            (1, 2),
            (2, 1),
            (-1, 2),
            (-2, 1),
            (-1, -2),
            (-2, -1),
            (1, -2),
            (2, -1),
        ] {
            if let Some(to) = pos.offset(r, c) {
                if let Some(turn) = self.get_turn_simple(pos, to) {
                    self.add_move_if_legal(turn, &mut moves);
                }
            }
        }

        moves
    }

    fn pawn_moves(&mut self, pos: Position) -> Vec<Turn> {
        let mut moves = vec![];

        let color = self.at_position(pos).unwrap().color;

        self.pawn_advance(pos, &mut moves);
        self.pawn_capture(pos, -1, &mut moves);
        self.pawn_capture(pos, 1, &mut moves);
        self.pawn_en_passant(pos, &mut moves);

        // 6th row, promotions
        if pos.row() == color.get_home() + color.get_direction() * 6 {}

        moves
    }

    fn pawn_advance(&mut self, pos: Position, moves: &mut Vec<Turn>) {
        let piece = self.at_position(pos).unwrap().clone();
        if let Some(pos_offset) = pos.offset(piece.color.get_direction(), 0) {
            if self.at_position(pos_offset).is_none() {
                // Promotion
                if pos_offset.row() == (!piece.color).get_home() {
                    for promo in PROMOTABLE_TYPES {
                        self.add_move_if_legal(
                            Turn::new_promotion(piece.kind, pos, pos_offset, promo, false),
                            moves,
                        );
                    }
                } else {
                    self.add_move_if_legal(Turn::new_basic(piece.kind, pos, pos_offset), moves);
                }
            }
            // First move can be two spaces
            if pos.row() == piece.color.get_home() + piece.color.get_direction() {
                let pos_offset = pos_offset
                    .offset(piece.color.get_direction(), 0)
                    .expect("Since they're at row 2, we should never leave the board");
                if self.at_position(pos_offset).is_none() {
                    self.add_move_if_legal(Turn::new_basic(piece.kind, pos, pos_offset), moves);
                }
            }
        }
    }

    fn pawn_capture(&mut self, pos: Position, c_off: i8, moves: &mut Vec<Turn>) {
        let this_piece = self.at_position(pos).unwrap();
        if let Some(pos_offset) = pos.offset(this_piece.color.get_direction(), c_off) {
            if let Some(other_piece) = self.at_position(pos_offset) {
                let other_kind = other_piece.kind;
                if this_piece.color == !other_piece.color {
                    // Promotion
                    if pos_offset.row() == other_piece.color.get_home() {
                        for promo in PROMOTABLE_TYPES {
                            self.add_move_if_legal(
                                Turn::new_promotion(other_kind, pos, pos_offset, promo, true),
                                moves,
                            );
                        }
                    } else {
                        self.add_move_if_legal(
                            Turn::new_capture(this_piece.kind, pos, pos_offset),
                            moves,
                        );
                    }
                }
            }
        }
    }

    fn pawn_en_passant(&mut self, pos: Position, moves: &mut Vec<Turn>) {
        let this_piece = self.at_position(pos).unwrap();
        // Only if the pawn is at the right position
        if pos.rank() == this_piece.color.get_home() + this_piece.color.get_direction() * 4 {
            // If the last move was a two-space pawn push adjacent to this
            // pawn
            if let Some(turn) = self.get_prev_turn() {
                if turn.kind == PieceType::Pawn
                    && turn.from.col() == (!this_piece.color).get_home() - this_piece.color.get_direction()
                    && turn.to.col() == pos.col()
                    && (-1..=1).contains(&(turn.to.row() - pos.col()))
                {
                    // Holy hell
                    self.add_move_if_legal(
                        Turn::new_capture_complex(
                            this_piece.kind,
                            pos,
                            Position::new(pos.row() + this_piece.color.get_direction(), turn.to.col()),
                            turn.to,
                        ),
                        moves,
                    )
                }
            }
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "To move: {}", self.turn)?;
        writeln!(f, "Pieces:")?;
        for (i, square) in self.squares.iter().enumerate() {
            if let Some(piece) = square {
                let pos = Position::from(i as i8);
                writeln!(f, "- {}: {}", pos, piece)?;
            }
        }
        writeln!(f, "Captures:")?;
        for cap in self.captures.iter() {
            writeln!(f, "- {}", cap)?;
        }
        writeln!(f, "Turns:")?;
        for turn in self.moves.iter() {
            writeln!(f, "- {}", turn)?;
        }

        Ok(())
    }
}
