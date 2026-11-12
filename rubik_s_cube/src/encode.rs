use crate::cube::{Color, RubiksCube};

/// One-hot encoding size: 54 stickers × 6 colors.
pub const STATE_SIZE: usize = 54 * Color::COUNT;

/// Flatten cube state to a one-hot `f32` vector of length [`STATE_SIZE`].
pub fn encode_state(cube: &RubiksCube) -> Vec<f32> {
    let mut out = vec![0.0f32; STATE_SIZE];
    let mut sticker = 0usize;
    for face in &cube.faces {
        for row in face {
            for &color in row {
                out[sticker * Color::COUNT + color.as_index()] = 1.0;
                sticker += 1;
            }
        }
    }
    debug_assert_eq!(sticker, 54);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solved_encoding_has_54_ones() {
        let encoded = encode_state(&RubiksCube::new());
        assert_eq!(encoded.len(), STATE_SIZE);
        let sum: f32 = encoded.iter().sum();
        assert!((sum - 54.0).abs() < 1e-5);
    }
}
