use super::{PieceType, Position};

/// Represents a move that can be made
#[derive(Debug, Clone)]
pub struct Turn {
    /// Kind of piece being moved
    pub kind: PieceType,
    /// Position to move from
    pub from: Position,
    /// Position to move to
    pub to: Position,
    /// Position of any piece being captured in the move
    pub capture: Option<Position>,
    /// From/to positions of any other piece that needs to be moved
    pub additional_move: Option<(Position, Position)>,
}

impl Turn {
    /// Create a new basic move
    pub fn new_basic(kind: PieceType, from: Position, to: Position) -> Self {
        Self {
            kind,
            from,
            to,
            capture: None,
            additional_move: None,
        }
    }

    /// Create a new move that captures a piece
    pub fn new_capture(kind: PieceType, from: Position, to: Position) -> Self {
        Self {
            kind,
            from,
            to,
            capture: Some(to),
            additional_move: None,
        }
    }

    /// Create a new move that involves an additional move
    pub fn new_additional(
        kind: PieceType,
        main: (Position, Position),
        other: (Position, Position),
    ) -> Self {
        Self {
            kind,
            from: main.0,
            to: main.1,
            capture: None,
            additional_move: Some(other),
        }
    }

    /// Create a new move that involves a capture on a different square
    pub fn new_capture_complex(
        kind: PieceType,
        from: Position,
        to: Position,
        capture: Position,
    ) -> Self {
        Self {
            kind,
            from,
            to,
            capture: Some(capture),
            additional_move: None,
        }
    }
}
