/// Face colors of the cube.
/// Layout (net):
/// ```text
///       Red(4)
/// White(0) Blue(1) Yellow(2) Green(3)
///       Orange(5)
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Color {
    White = 0,
    Blue = 1,
    Yellow = 2,
    Green = 3,
    Red = 4,
    Orange = 5,
}

impl Color {
    pub const COUNT: usize = 6;

    pub fn as_index(self) -> usize {
        self as u8 as usize
    }

    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Self::White,
            1 => Self::Blue,
            2 => Self::Yellow,
            3 => Self::Green,
            4 => Self::Red,
            5 => Self::Orange,
            _ => panic!("invalid color index: {index}"),
        }
    }

    pub fn emoji(self) -> &'static str {
        match self {
            Self::White => "⬜",
            Self::Blue => "🟦",
            Self::Yellow => "🟨",
            Self::Green => "🟩",
            Self::Red => "🟥",
            Self::Orange => "🟧",
        }
    }
}
