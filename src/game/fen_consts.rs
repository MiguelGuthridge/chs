
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
    IllegalCastling(String)
}
