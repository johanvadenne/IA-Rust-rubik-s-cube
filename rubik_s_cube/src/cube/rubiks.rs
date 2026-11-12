use rand::Rng;

use super::{Color, Move, ALL_MOVES};

/// Rubik's cube state: 6 faces × 3×3 stickers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RubiksCube {
    /// Face order: White, Blue, Yellow, Green, Red, Orange.
    pub faces: [[[Color; 3]; 3]; 6],
}

impl RubiksCube {
    pub fn new() -> Self {
        let faces = [
            [[Color::White; 3]; 3],
            [[Color::Blue; 3]; 3],
            [[Color::Yellow; 3]; 3],
            [[Color::Green; 3]; 3],
            [[Color::Red; 3]; 3],
            [[Color::Orange; 3]; 3],
        ];
        Self { faces }
    }

    pub fn is_solved(&self) -> bool {
        for face in &self.faces {
            let center = face[1][1];
            for row in face {
                for &color in row {
                    if color != center {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Rotation around the Blue–Green axis for the given layer (0 or 2 for outer faces).
    pub fn rotate_x(&mut self, layer: usize) {
        let (front, back, top, bottom) = (0, 2, 4, 5);
        let temp = self.faces[top][layer];

        for i in 0..3 {
            self.faces[top][layer][i] = self.faces[front][2 - i][layer];
            self.faces[front][2 - i][layer] = self.faces[bottom][2 - layer][2 - i];
            self.faces[bottom][2 - layer][2 - i] = self.faces[back][i][2 - layer];
            self.faces[back][i][2 - layer] = temp[i];
        }

        if layer == 0 {
            Self::rotate_face(&mut self.faces[3], false);
        } else if layer == 2 {
            Self::rotate_face(&mut self.faces[1], true);
        }
    }

    /// Rotation around the White–Yellow axis for the given layer.
    pub fn rotate_y(&mut self, layer: usize) {
        let (front, back, top, bottom) = (1, 3, 4, 5);
        let temp = self.faces[top];

        for i in 0..3 {
            self.faces[top][i][layer] = self.faces[front][i][layer];
            self.faces[front][i][layer] = self.faces[bottom][i][layer];
            self.faces[bottom][i][layer] = self.faces[back][2 - i][2 - layer];
            self.faces[back][2 - i][2 - layer] = temp[i][layer];
        }

        if layer == 0 {
            Self::rotate_face(&mut self.faces[0], false);
        } else if layer == 2 {
            Self::rotate_face(&mut self.faces[2], true);
        }
    }

    /// Rotation around the Red–Orange axis for the given layer.
    pub fn rotate_z(&mut self, layer: usize) {
        let (front, back, top, bottom) = (0, 2, 1, 3);
        let temp = self.faces[top][layer];

        for i in 0..3 {
            self.faces[top][layer][i] = self.faces[front][layer][i];
            self.faces[front][layer][i] = self.faces[bottom][layer][i];
            self.faces[bottom][layer][i] = self.faces[back][layer][i];
            self.faces[back][layer][i] = temp[i];
        }

        if layer == 0 {
            Self::rotate_face(&mut self.faces[4], false);
        } else if layer == 2 {
            Self::rotate_face(&mut self.faces[5], true);
        }
    }

    fn rotate_face(face: &mut [[Color; 3]; 3], clockwise: bool) {
        let temp = *face;
        if clockwise {
            for i in 0..3 {
                for j in 0..3 {
                    face[j][2 - i] = temp[i][j];
                }
            }
        } else {
            for i in 0..3 {
                for j in 0..3 {
                    face[j][i] = temp[i][2 - j];
                }
            }
        }
    }

    /// Neighbors after each of the 18 moves (cube, move).
    pub fn neighbors(&self) -> Vec<(Self, Move)> {
        ALL_MOVES
            .iter()
            .map(|&mv| {
                let mut next = self.clone();
                next.apply(mv);
                (next, mv)
            })
            .collect()
    }

    /// Scramble with `depth` random moves, avoiding immediate inverse / same-face repeats.
    pub fn scramble<R: Rng>(&mut self, depth: usize, rng: &mut R) -> Vec<Move> {
        let mut moves = Vec::with_capacity(depth);
        let mut last_face: Option<u8> = None;

        for _ in 0..depth {
            let candidates: Vec<Move> = ALL_MOVES
                .iter()
                .copied()
                .filter(|mv| last_face.map(|f| mv.face != f).unwrap_or(true))
                .collect();
            let mv = candidates[rng.gen_range(0..candidates.len())];
            self.apply(mv);
            last_face = Some(mv.face);
            moves.push(mv);
        }
        moves
    }

    /// Print the cube net with emoji colors.
    pub fn display(&self) {
        for row in &self.faces[4] {
            print!("⬛⬛⬛");
            for &color in row {
                print!("{}", color.emoji());
            }
            println!("⬛⬛⬛⬛⬛⬛");
        }

        for i in 0..3 {
            for face in 0..4 {
                for k in 0..3 {
                    print!("{}", self.faces[face][i][k].emoji());
                }
            }
            println!();
        }

        for row in &self.faces[5] {
            print!("⬛⬛⬛");
            for &color in row {
                print!("{}", color.emoji());
            }
            println!("⬛⬛⬛⬛⬛⬛");
        }
    }
}

impl Default for RubiksCube {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::Move;

    #[test]
    fn new_cube_is_solved() {
        assert!(RubiksCube::new().is_solved());
    }

    #[test]
    fn four_quarter_turns_are_identity() {
        for face in 0..6u8 {
            let mut cube = RubiksCube::new();
            let mv = Move { face, turns: 1 };
            for _ in 0..4 {
                cube.apply(mv);
            }
            assert!(cube.is_solved(), "face {face} failed identity");
        }
    }

    #[test]
    fn double_turn_twice_is_identity() {
        for face in 0..6u8 {
            let mut cube = RubiksCube::new();
            let mv = Move { face, turns: 2 };
            cube.apply(mv);
            cube.apply(mv);
            assert!(cube.is_solved(), "face {face} double turn failed");
        }
    }

    #[test]
    fn move_then_inverse_is_identity() {
        for &mv in &ALL_MOVES {
            let mut cube = RubiksCube::new();
            cube.apply(mv);
            cube.apply(mv.inverse());
            assert!(cube.is_solved(), "move {:?} inverse failed", mv);
        }
    }

    #[test]
    fn scramble_then_inverse_sequence_solves() {
        let mut rng = rand::thread_rng();
        let mut cube = RubiksCube::new();
        let moves = cube.scramble(12, &mut rng);
        assert!(!cube.is_solved());
        for mv in moves.iter().rev() {
            cube.apply(mv.inverse());
        }
        assert!(cube.is_solved());
    }
}
