use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Obstacle,
    Box,
    LeftBox,
    RightBox,
}

impl Tile {
    pub fn is_big_box(&self) -> bool {
        matches!(self, Tile::LeftBox | Tile::RightBox)
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Obstacle,
            'O' => Tile::Box,
            '[' => Tile::LeftBox,
            ']' => Tile::RightBox,
            _ => Tile::Empty,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Box => write!(f, "O"),
            Tile::LeftBox => write!(f, "["),
            Tile::RightBox => write!(f, "]"),
            Tile::Obstacle => write!(f, "#"),
            Tile::Empty => write!(f, "."),
        }
    }
}
