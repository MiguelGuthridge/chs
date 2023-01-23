use std::fmt::Display;

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
    /// The kind of piece to promote to
    pub promote_to: Option<PieceType>,
    /// The kind of piece that was promoted from
    /// TODO: figure out why we need this
    pub promote_from: Option<PieceType>,
}

impl Turn {
    /// Create a move where you can specify any properties
    pub fn new(
        kind: PieceType,
        from: Position,
        to: Position,
        capture: Option<Position>,
        additional_move: Option<(Position, Position)>,
        promote_to: Option<PieceType>,
    ) -> Self {
        Self {
            kind,
            from,
            to,
            capture,
            additional_move,
            promote_to,
            promote_from: if promote_to.is_some() {
                Some(kind)
            } else {
                None
            },
        }
    }

    /// Create a new basic move
    pub fn new_basic(kind: PieceType, from: Position, to: Position) -> Self {
        Self {
            kind,
            from,
            to,
            capture: None,
            additional_move: None,
            promote_to: None,
            promote_from: None,
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
            promote_to: None,
            promote_from: None,
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
            promote_to: None,
            promote_from: None,
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
            promote_to: None,
            promote_from: None,
        }
    }

    /// Create a new move that involves a promotion
    pub fn new_promotion(
        kind: PieceType,
        from: Position,
        to: Position,
        promote_to: PieceType,
        capture: bool,
    ) -> Self {
        Self {
            kind,
            from,
            to,
            capture: if capture { Some(to) } else { None },
            additional_move: None,
            promote_to: Some(promote_to),
            promote_from: Some(kind),
        }
    }
}

impl Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} from {} to {}", self.kind, self.from, self.to)?;
        if let Some((add_to, add_from)) = self.additional_move {
            write!(f, ", additionally moving {} to {}", add_from, add_to)?;
        }
        if let Some(cap_pos) = self.capture {
            if cap_pos != self.to {
                write!(f, ", capturing {}", cap_pos)?;
            } else {
                write!(f, ", capturing")?;
            }
        }
        if let Some(promo) = self.promote_to {
            write!(f, ", promoting to {}", promo)?;
        }

        Ok(())
    }
}
