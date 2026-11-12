use super::RubiksCube;

/// 18 face turns: 6 faces × {90°, 180°, 270°}.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    /// Face index 0..6 (same order as [`super::Color`]).
    pub face: u8,
    /// Number of 90° clockwise turns when looking at the face: 1, 2, or 3.
    pub turns: u8,
}

pub const NUM_MOVES: usize = 18;

pub const ALL_MOVES: [Move; NUM_MOVES] = [
    Move { face: 0, turns: 1 },
    Move { face: 0, turns: 2 },
    Move { face: 0, turns: 3 },
    Move { face: 1, turns: 1 },
    Move { face: 1, turns: 2 },
    Move { face: 1, turns: 3 },
    Move { face: 2, turns: 1 },
    Move { face: 2, turns: 2 },
    Move { face: 2, turns: 3 },
    Move { face: 3, turns: 1 },
    Move { face: 3, turns: 2 },
    Move { face: 3, turns: 3 },
    Move { face: 4, turns: 1 },
    Move { face: 4, turns: 2 },
    Move { face: 4, turns: 3 },
    Move { face: 5, turns: 1 },
    Move { face: 5, turns: 2 },
    Move { face: 5, turns: 3 },
];

impl Move {
    pub fn from_index(index: usize) -> Self {
        ALL_MOVES[index]
    }

    pub fn index(self) -> usize {
        (self.face as usize) * 3 + (self.turns as usize - 1)
    }

    pub fn inverse(self) -> Self {
        Self {
            face: self.face,
            turns: match self.turns {
                1 => 3,
                2 => 2,
                3 => 1,
                _ => unreachable!(),
            },
        }
    }

    /// True if both moves twist the same face (useful to prune A*).
    pub fn same_face(self, other: Self) -> bool {
        self.face == other.face
    }

    pub fn label(self) -> String {
        let face = match self.face {
            0 => "W",
            1 => "B",
            2 => "Y",
            3 => "G",
            4 => "R",
            5 => "O",
            _ => "?",
        };
        match self.turns {
            1 => format!("{face}"),
            2 => format!("{face}2"),
            3 => format!("{face}'"),
            _ => format!("{face}?"),
        }
    }
}

impl RubiksCube {
    /// Apply one of the 18 face moves.
    ///
    /// Mapping to the existing X/Y/Z layer rotations:
    /// - White (0)  → Y layer 0
    /// - Yellow (2) → Y layer 2
    /// - Blue (1)   → X layer 2
    /// - Green (3)  → X layer 0
    /// - Red (4)    → Z layer 0
    /// - Orange (5) → Z layer 2
    pub fn apply(&mut self, mv: Move) {
        for _ in 0..mv.turns {
            match mv.face {
                0 => self.rotate_y(0),
                1 => self.rotate_x(2),
                2 => self.rotate_y(2),
                3 => self.rotate_x(0),
                4 => self.rotate_z(0),
                5 => self.rotate_z(2),
                _ => unreachable!("invalid face {}", mv.face),
            }
        }
    }

    pub fn apply_sequence(&mut self, moves: &[Move]) {
        for &mv in moves {
            self.apply(mv);
        }
    }
}
