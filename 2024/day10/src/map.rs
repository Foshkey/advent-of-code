use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Map {
    grid: Vec<Vec<u8>>,
    trails: HashMap<Coord, HashSet<Coord>>,
}

impl Map {
    pub fn get_trailhead_scores(&self) -> usize {
        let mut map = Map {
            grid: self.grid.clone(),
            trails: self.trails.clone(),
        };
        map.map_trails();
        map.trails.values().map(|peaks| peaks.len()).sum()
    }

    pub fn count_possible_trails(&self) -> usize {
        self.trails
            .keys()
            .map(|&trailhead| self.count(trailhead, 0))
            .sum()
    }

    fn map_trails(&mut self) {
        let trailheads: Vec<_> = self.trails.keys().cloned().collect();
        for trailhead in trailheads {
            self.map_trail(trailhead, trailhead, 0);
        }
    }

    fn map_trail(&mut self, trailhead: Coord, position: Coord, height: u8) {
        if height == 9 {
            self.trails
                .get_mut(&trailhead)
                .map(|peaks| peaks.insert(position));
            return;
        }

        for (neighbor_position, neighbor_height) in self.get_neighbors(position) {
            if neighbor_height == height + 1 {
                self.map_trail(trailhead, neighbor_position, neighbor_height);
            }
        }
    }

    fn count(&self, position: Coord, height: u8) -> usize {
        if height == 9 {
            return 1;
        }

        self.get_neighbors(position)
            .iter()
            .map(|&(neighbor_position, neighbor_value)| {
                if neighbor_value == height + 1 {
                    self.count(neighbor_position, neighbor_value)
                } else {
                    0
                }
            })
            .sum()
    }

    fn get_neighbors(&self, position: Coord) -> Vec<(Coord, u8)> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .filter_map(|&(d_row, d_col)| self.get_relative(position, d_row, d_col))
            .collect()
    }

    fn get_relative(&self, position: Coord, d_row: isize, d_col: isize) -> Option<(Coord, u8)> {
        let row = position.row.checked_add_signed(d_row)?;
        let col = position.col.checked_add_signed(d_col)?;
        let num = self.grid.get(row)?.get(col)?;
        Some((Coord { row, col }, *num))
    }
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let mut trails = HashMap::new();
        let grid = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.char_indices()
                    .map(|(col, c)| {
                        let n = c.to_string().parse::<u8>().unwrap_or(20);
                        if n == 0 {
                            trails.insert(Coord { row, col }, HashSet::new());
                        }
                        n
                    })
                    .collect()
            })
            .collect();

        Map { grid, trails }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    row: usize,
    col: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_example_1() {
        let input = "\
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";
        let map: Map = input.into();
        assert_eq!(map.get_trailhead_scores(), 2);
    }

    #[test]
    fn test_simple_example_2() {
        let input = "\
..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
        let map: Map = input.into();
        assert_eq!(map.get_trailhead_scores(), 4);
    }

    #[test]
    fn test_simple_example_3() {
        let input = "\
10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01";
        let map: Map = input.into();
        assert_eq!(map.get_trailhead_scores(), 3);
    }
}
