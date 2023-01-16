use std::fmt::Display;

use super::{turn::Turn, Board, Color, Position};

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

const PROMOTABLE_TYPES: [PieceType; 4] = [
    PieceType::Queen,
    PieceType::Rook,
    PieceType::Bishop,
    PieceType::Knight,
];

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

    /// Return the moves that can be legally made by this piece
    ///
    /// pos: current position of the piece
    /// board: reference to the board
    pub fn get_moves(&self, pos: Position, board: &Board) -> Vec<Turn> {
        let moves = match self.kind {
            PieceType::King => self.king_moves(pos, board),
            PieceType::Queen => self.queen_moves(pos, board),
            PieceType::Rook => self.rook_moves(pos, board),
            PieceType::Bishop => self.bishop_moves(pos, board),
            PieceType::Knight => self.knight_moves(pos, board),
            PieceType::Pawn => self.pawn_moves(pos, board),
        };

        // TODO: filter out illegal moves

        moves
    }

    /// Returns whether it's possible to move this piece into the given square,
    /// as well as a reference to the piece there
    ///
    /// This returns true if the square is empty, or if it has a piece of the
    /// opposite color
    ///
    /// It does not account for the kind of piece this is
    fn get_turn_simple(&self, from: Position, to: Position, board: &Board) -> Option<Turn> {
        if let Some(piece) = board.at_position(to) {
            if piece.color != self.color {
                Some(Turn::new_capture(self.kind, from, to))
            } else {
                None
            }
        } else {
            Some(Turn::new_basic(self.kind, from, to))
        }
    }

    /// Get moves in a line from the given directions
    fn line_moves(&self, pos: Position, board: &Board, directions: &[(i8, i8)]) -> Vec<Turn> {
        let mut moves = vec![];

        for (r_off, c_off) in directions {
            let mut new_pos = pos;
            while let Some(off_pos) = new_pos.offset(*r_off, *c_off) {
                new_pos = off_pos;
                if let Some(turn) = self.get_turn_simple(pos, new_pos, board) {
                    let was_capture = turn.capture.is_some();
                    moves.push(turn);
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

    fn rook_moves(&self, pos: Position, board: &Board) -> Vec<Turn> {
        self.line_moves(pos, board, &[(1, 0), (0, 1), (-1, 0), (0, -1)])
    }

    fn bishop_moves(&self, pos: Position, board: &Board) -> Vec<Turn> {
        self.line_moves(pos, board, &[(1, 1), (1, -1), (-1, -1), (-1, 1)])
    }

    fn queen_moves(&self, pos: Position, board: &Board) -> Vec<Turn> {
        self.line_moves(
            pos,
            board,
            &[
                (1, 0),
                (0, 1),
                (-1, 0),
                (0, -1),
                (1, 0),
                (0, 1),
                (-1, 0),
                (0, -1),
            ],
        )
    }

    fn king_moves(&self, from_pos: Position, board: &Board) -> Vec<Turn> {
        let mut moves = vec![];
        for r in [-1, 0, 1] {
            for c in [-1, 0, 1] {
                if r != 0 || c != 0 {
                    if let Some(to_pos) = from_pos.offset(r, c) {
                        if let Some(turn) = self.get_turn_simple(from_pos, to_pos, board) {
                            moves.push(turn);
                        }
                    }
                }
            }
        }
        // Castling
        // Can't have moved, and must be on the first rank
        if self.move_count == 0 && from_pos.row() == if self.color == Color::White { 0 } else { 7 }
        {
            self.castling_moves(from_pos, board, &mut moves);
        }
        moves
    }

    fn castling_moves(&self, from_pos: Position, board: &Board, moves: &mut Vec<Turn>) {
        // Find the rooks
        for (row, col, res_col) in [(0, 1, 6), (0, -1, 2)] {
            // Check each square for pieces
            let mut new_pos = from_pos;
            while let Some(pos) = new_pos.offset(row, col) {
                new_pos = pos;
                // If it contains a piece
                if let Some(piece) = board.at_position(new_pos) {
                    // If it's our rook
                    if piece.kind == PieceType::Rook
                        && piece.color == self.color
                        && piece.move_count == 0
                    {
                        // We might be able to castle
                        // Check up to the resultant square that nothing is
                        // under attack
                        // TODO

                        // For now just add it to the moves regardless
                        moves.push(Turn::new_additional(
                            self.kind,
                            (from_pos, Position::new(from_pos.row(), res_col)),
                            (new_pos, Position::new(from_pos.row(), res_col - col)),
                        ));
                    }
                }
            }
        }
    }

    fn knight_moves(&self, pos: Position, board: &Board) -> Vec<Turn> {
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
                if let Some(turn) = self.get_turn_simple(pos, to, board) {
                    moves.push(turn);
                }
            }
        }

        moves
    }

    fn pawn_moves(&self, pos: Position, board: &Board) -> Vec<Turn> {
        let mut moves = vec![];

        self.pawn_advance(pos, board, &mut moves);
        self.pawn_capture(pos, -1, board, &mut moves);
        self.pawn_capture(pos, 1, board, &mut moves);
        self.pawn_en_passant(pos, board, &mut moves);

        // 6th row, promotions
        if pos.row() == self.color.get_home() + self.color.get_direction() * 6 {}

        moves
    }

    fn pawn_advance(&self, pos: Position, board: &Board, moves: &mut Vec<Turn>) {
        if let Some(pos_offset) = pos.offset(self.color.get_direction(), 0) {
            if board.at_position(pos_offset).is_none() {
                // Promotion
                if pos_offset.row() == (!self.color).get_home() {
                    for promo in PROMOTABLE_TYPES {
                        moves.push(Turn::new_promotion(
                            self.kind, pos, pos_offset, promo, false,
                        ));
                    }
                } else {
                    moves.push(Turn::new_basic(self.kind, pos, pos_offset));
                }
            }
            // First move can be two spaces
            if pos.row() == self.color.get_home() + self.color.get_direction() {
                let pos_offset = pos_offset
                    .offset(self.color.get_direction(), 0)
                    .expect("Since they're at row 2, we should never leave the board");
                if board.at_position(pos_offset).is_none() {
                    moves.push(Turn::new_basic(self.kind, pos, pos_offset));
                }
            }
        }
    }

    fn pawn_capture(&self, pos: Position, c_off: i8, board: &Board, moves: &mut Vec<Turn>) {
        if let Some(pos_offset) = pos.offset(self.color.get_direction(), c_off) {
            if let Some(piece) = board.at_position(pos_offset) {
                if piece.color == !self.color {
                    // Promotion
                    if pos_offset.row() == (!self.color).get_home() {
                        for promo in PROMOTABLE_TYPES {
                            moves.push(Turn::new_promotion(
                                self.kind, pos, pos_offset, promo, true,
                            ));
                        }
                    } else {
                        moves.push(Turn::new_capture(self.kind, pos, pos_offset));
                    }
                }
            }
        }
    }

    fn pawn_en_passant(&self, pos: Position, board: &Board, moves: &mut Vec<Turn>) {
        // Only if the pawn is at the right position
        if pos.rank() == self.color.get_home() + self.color.get_direction() * 4 {
            // If the last move was a two-space pawn push adjacent to this
            // pawn
            if let Some(turn) = board.get_prev_turn() {
                if turn.kind == PieceType::Pawn
                    && turn.from.col() == (!self.color).get_home() - self.color.get_direction()
                    && turn.to.col() == pos.col()
                    && (-1..=1).contains(&(turn.to.row() - pos.col()))
                {
                    // Holy hell
                    moves.push(Turn::new_capture_complex(
                        self.kind,
                        pos,
                        Position::new(pos.row() + self.color.get_direction(), turn.to.col()),
                        turn.to,
                    ))
                }
            }
        }
    }
}
