use anyhow::{bail, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Direction {
    pub x: i32,
    pub y: i32,
}

impl Direction {
    pub fn new(c: char) -> Result<Self> {
        Ok(match c {
            'U' => Self { x: 0, y: -1 },
            'D' => Self { x: 0, y: 1 },
            'L' => Self { x: -1, y: 0 },
            'R' => Self { x: 1, y: 0 },
            _ => bail!("Invalid direction: {}", c),
        })
    }
}
