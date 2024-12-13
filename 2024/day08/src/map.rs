use std::collections::{HashMap, HashSet};

pub struct Map {
    antennas: HashMap<char, HashSet<Coord>>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Map {
            antennas: HashMap::new(),
            width,
            height,
        }
    }

    pub fn insert(&mut self, frequency: char, position: Coord) {
        if let Some(positions) = self.antennas.get_mut(&frequency) {
            positions.insert(position);
        } else {
            self.antennas.insert(frequency, HashSet::from([position]));
        }
    }

    pub fn get_antinodes(&self) -> HashSet<Coord> {
        let mut antinodes = HashSet::new();

        for (_, positions) in self.antennas.iter() {
            for antenna in positions {
                for other in positions {
                    if let Some(antinode) = self.get_antinode(antenna, other) {
                        antinodes.insert(antinode);
                    }
                }
            }
        }

        antinodes
    }

    fn get_antinode(&self, a: &Coord, b: &Coord) -> Option<Coord> {
        if a == b {
            return None;
        }

        let d_row = b.row as isize - a.row as isize;
        let row = a.row.checked_add_signed(d_row * 2)?;
        if row >= self.height {
            return None;
        }

        let d_col = b.col as isize - a.col as isize;
        let col = a.col.checked_add_signed(d_col * 2)?;
        if col >= self.width {
            return None;
        }

        Some(Coord { row, col })
    }
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let width = s.lines().next().unwrap().chars().count();
        let height = s.lines().count();
        let mut map = Map::new(width, height);

        for (row, line) in s.lines().enumerate() {
            for (col, c) in line.char_indices() {
                if c != '.' {
                    map.insert(c, Coord { row, col });
                }
            }
        }

        map
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    row: usize,
    col: usize,
}
