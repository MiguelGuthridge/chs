mod fen;
mod moves;
mod turns;

use arr_macro::arr;
pub use fen::FenError;
use std::fmt::{Debug, Display};

use super::{
    game_state::{DrawReason, GameState, WinReason},
    piece::{Piece},
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
    whose_turn: Color,

    /// Vector of moves
    moves: Vec<Turn>,

    /// Number of half moves since pawn push or capture
    half_move_clock: Vec<i8>,

    /// Number of full moves
    num_moves: i32,

    /// Position to target for en passant
    en_passant_target: Option<Position>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            captures: Default::default(),
            squares: arr![None; 64],
            whose_turn: Color::White,
            moves: Default::default(),
            half_move_clock: vec![0],
            en_passant_target: None,
            num_moves: 1,
        }
    }
}

impl Board {
    /// Create a board in the starting position
    pub fn from_start() -> Self {
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

    /// Return a reference to the piece in a particular position
    pub fn at_position(&self, position: Position) -> Option<&Piece> {
        self.squares[position.pos()].as_ref()
    }

    /// Return whose turn it is
    pub fn whose_turn(&self) -> Color {
        self.whose_turn
    }

    /// Returns a reference to the previous turn
    pub fn get_prev_turn(&self) -> Option<&Turn> {
        if self.moves.is_empty() {
            None
        } else {
            Some(&self.moves[self.moves.len() - 1])
        }
    }

    /// Returns whether position is check
    pub fn is_check(&self) -> bool {
        self.is_king_attacked(self.whose_turn)
    }

    /// Returns whether position is checkmate
    pub fn is_checkmate(&mut self) -> bool {
        self.is_check() && self.do_get_moves().is_empty()
    }

    /// Returns whether the position is stalemate
    pub fn is_stalemate(&mut self) -> bool {
        !self.is_check() && self.do_get_moves().is_empty()
    }

    /// Returns whether the position is a draw by threefold repetition
    pub fn is_threefold_repetition(&self) -> bool {
        // todo!()
        false
    }

    /// Returns whether its a draw by the 50 move rule
    pub fn is_50_move_rule(&self) -> bool {
        *self.half_move_clock.last().unwrap() >= 100
    }

    /// Returns whether it's a draw by insufficient repetition
    pub fn is_insufficient_material(&self) -> bool {
        // todo!()
        false
    }

    /// Returns whether the game is a draw
    pub fn is_draw(&mut self) -> bool {
        !self.is_checkmate()
            && (self.is_stalemate()
                || self.is_threefold_repetition()
                || self.is_50_move_rule()
                || self.is_insufficient_material())
    }

    /// Returns whether the game is over
    pub fn is_game_over(&mut self) -> bool {
        self.is_draw() || self.is_checkmate()
    }

    /// Returns the state of the game
    pub fn get_game_state(&mut self) -> GameState {
        if self.is_checkmate() {
            GameState::Win(!self.whose_turn, WinReason::Checkmate)
        } else if self.is_stalemate() {
            GameState::Draw(DrawReason::Stalemate)
        } else if self.is_50_move_rule() {
            GameState::Draw(DrawReason::FiftyMoveRule)
        } else if self.is_threefold_repetition() {
            GameState::Draw(DrawReason::ThreefoldRepetition)
        } else if self.is_insufficient_material() {
            GameState::Draw(DrawReason::InsufficientMaterial)
        } else {
            GameState::Playing
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "To move: {}", self.whose_turn)?;
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
