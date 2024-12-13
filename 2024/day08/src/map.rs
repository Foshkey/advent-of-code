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

    pub fn get_antinodes_with_resonance(&self) -> HashSet<Coord> {
        let mut antinodes = HashSet::new();

        for (_, positions) in self.antennas.iter() {
            for antenna in positions {
                for other in positions {
                    for antinode in self.get_antinode_line(antenna, other) {
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

        let d_row = (b.row as isize - a.row as isize) * 2;
        let d_col = (b.col as isize - a.col as isize) * 2;
        self.check_add(a, d_row, d_col)
    }

    fn get_antinode_line(&self, a: &Coord, b: &Coord) -> Vec<Coord> {
        if a == b {
            return vec![];
        }

        let d_row = b.row as isize - a.row as isize;
        let d_col = b.col as isize - a.col as isize;
        let mut mult = 1;
        let mut antinodes = Vec::new();

        while let Some(antinode) = self.check_add(a, d_row * mult, d_col * mult) {
            mult += 1;
            antinodes.push(antinode)
        }

        antinodes
    }

    fn check_add(&self, position: &Coord, d_row: isize, d_col: isize) -> Option<Coord> {
        let row = position.row.checked_add_signed(d_row)?;
        if row >= self.height {
            return None;
        }

        let col = position.col.checked_add_signed(d_col)?;
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
