#[derive(Debug)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

impl Coord {
    pub fn to_tuple(&self) -> (isize, isize) {
        (self.x, self.y)
    }
}

impl From<&str> for Coord {
    fn from(s: &str) -> Self {
        let (x, y) = s.split_once(',').unwrap();
        Coord {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}
