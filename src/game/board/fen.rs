use std::num::ParseIntError;

use crate::game::{piece::Piece, Color, PieceType, Position};

use super::Board;

/// Error with FEN parsing
#[derive(Debug)]
pub enum FenError {
    /// FEN strings must be ASCII
    NotAscii,

    /// Wrong number of sections in FEN string. Expect:
    /// * positions
    /// * color
    /// * castling rights
    /// * en passant target
    /// * half-move clock
    /// * number of turns
    ///
    /// Includes actual number of sections given
    IncorrectSections(usize),

    /// Wrong number of rows (expected 8)
    IncorrectRows(i8),

    /// Wrong number of cols (expected 8)
    /// Includes row number and column number
    IncorrectCols(i8, i8),

    /// Invalid piece type
    /// Includes character for that piece type
    InvalidPiece(char),

    /// Invalid color for piece
    /// Includes the given color
    InvalidColor(String),

    /// Invalid position for en passant target
    InvalidPosition(String),

    /// Invalid string for castling
    InvalidCastling(String),

    /// Castles aren't on home row, but castling is enabled
    IllegalCastling(String),

    /// Failed to parse number
    InvalidNumber(ParseIntError),
}

impl From<ParseIntError> for FenError {
    fn from(e: ParseIntError) -> Self {
        FenError::InvalidNumber(e)
    }
}

impl Board {
    /// Create a new board from a FEN string
    pub fn from_fen(fen: &str) -> Result<Self, FenError> {
        if !fen.is_ascii() {
            return Err(FenError::NotAscii);
        }

        let mut board = Self::default();

        let mut row: i8 = 7;
        let mut col: i8 = 0;

        let fen_split: Vec<&str> = fen.split_ascii_whitespace().collect();

        if fen_split.len() != 6 {
            // Invalid FEN, wrong number of sections
            return Err(FenError::IncorrectSections(fen_split.len()));
        }

        let positions = fen_split[0];
        let to_move = fen_split[1];
        let castling = fen_split[2];
        let en_passant_target = fen_split[3];
        board.half_move_clock = vec![fen_split[4].parse()?];
        board.num_moves = fen_split[5].parse()?;

        // Piece positions
        for c in positions.chars() {
            // Numbers represent spaces
            if c.is_ascii_digit() {
                let spaces: i8 = String::from(c).parse().unwrap();
                col += spaces;
                if col > 8 {
                    // Too many spaces, invalid FEN
                    return Err(FenError::IncorrectCols(row, col));
                }
            } else if c == '/' {
                // Column should be complete
                if col != 8 {
                    return Err(FenError::IncorrectCols(row, col));
                }
                row += 1;
                col = 0;
                // Too many rows, invalid FEN
                if row == 8 {
                    return Err(FenError::IncorrectRows(row));
                }
            } else {
                // If we're >= col 8, there were too many columns
                if col >= 8 {
                    return Err(FenError::IncorrectCols(row, col));
                }
                let color = if c.is_ascii_uppercase() {
                    Color::White
                } else {
                    Color::Black
                };
                let kind = match c.to_ascii_lowercase() {
                    'k' => PieceType::King,
                    'q' => PieceType::Queen,
                    'b' => PieceType::Bishop,
                    'n' => PieceType::Knight,
                    'r' => PieceType::Rook,
                    _ => return Err(FenError::InvalidPiece(c)),
                };
                // Add piece to the board
                board.squares[Position::new(row, col).pos()] = Some(Piece::new(kind, color));
            }
        }
        // Afterwards, we should have completed 7 rows
        if row != 7 {
            return Err(FenError::IncorrectRows(row));
        }

        // Castling logic

        // Disable castling by default, then enable it if required
        for (pos, color) in [
            (Position::new(0, 0), Color::White),
            (Position::new(0, 7), Color::White),
            (Position::new(7, 0), Color::Black),
            (Position::new(7, 7), Color::Black),
        ] {
            if let Some(piece) = &mut board.squares[pos.pos()] {
                if piece.kind == PieceType::Rook && piece.color == color {
                    piece.move_count = 1;
                }
            }
        }
        // If some squares can castle
        if castling != "-" {
            for c in castling.chars() {
                let (pos, color) = match c {
                    'Q' => (Position::new(0, 0), Color::White),
                    'K' => (Position::new(0, 7), Color::White),
                    'q' => (Position::new(7, 0), Color::Black),
                    'k' => (Position::new(7, 7), Color::Black),
                    _ => return Err(FenError::IllegalCastling(castling.to_string())),
                };
                // If the correct rook is there
                if let Some(piece) = &mut board.squares[pos.pos()] {
                    if piece.kind == PieceType::Rook && piece.color == color {
                        // Let it castle
                        piece.move_count = 0;
                    }
                }
            }
        }

        // Parse other info
        board.whose_turn = Color::from_fen(to_move)?;
        board.en_passant_target = Position::from_fen(en_passant_target)?;

        Ok(board)
    }
}
