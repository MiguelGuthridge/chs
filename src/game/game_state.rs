use crate::game::Color;

/// Reasons for a draw
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrawReason {
    /// Same position 3 times
    ThreefoldRepetition,

    /// 50 moves without a capture or pawn push
    FiftyMoveRule,

    /// No moves available, but not checkmate
    Stalemate,

    /// Not enough material for checkmate
    InsufficientMaterial,

    /// Both players agreed to it
    /// Not tracked
    MutualAgreement,

    /// Time out, with remaining player having insufficient mating material
    /// Not tracked
    TimeOut,
}

/// Reasons for a win
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WinReason {
    /// Win by checkmate
    Checkmate,

    /// Opponent timed out
    /// Not tracked
    TimeOut,

    /// Opponent resigned
    /// Not tracked
    Resigned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameState {
    Playing,
    Win(Color, WinReason),
    Draw(DrawReason),
}
