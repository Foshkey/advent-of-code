use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Map {
    grid: Vec<Vec<u8>>,
    trailheads: Vec<Coord>,
}

impl Map {
    pub fn get_trailhead_scores(&self) -> usize {
        let mut trails: Trails = HashMap::new();
        for &trailhead in &self.trailheads {
            trails.insert(trailhead, HashSet::new());
        }
        self.map_trails(&mut trails);
        trails.values().map(|peaks| peaks.len()).sum()
    }

    pub fn count_possible_trails(&self) -> usize {
        self.trailheads
            .iter()
            .map(|&trailhead| self.map_trail(trailhead, 0, &mut None))
            .sum()
    }

    fn map_trails(&self, trails: &mut Trails) {
        let trailheads: Vec<_> = trails.keys().cloned().collect();
        for trailhead in trailheads {
            self.map_trail(trailhead, 0, &mut Some((trailhead, trails)));
        }
    }

    fn map_trail(
        &self,
        position: Coord,
        height: u8,
        tracking: &mut Option<(Coord, &mut Trails)>,
    ) -> usize {
        if height == 9 {
            if let Some((trailhead, trails)) = tracking.as_mut() {
                trails
                    .get_mut(trailhead)
                    .map(|peaks| peaks.insert(position));
            }
            return 1;
        }

        self.get_neighbors(position)
            .iter()
            .map(|&(neighbor_position, neighbor_value)| {
                if neighbor_value == height + 1 {
                    self.map_trail(neighbor_position, neighbor_value, tracking)
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
        let mut trailheads = Vec::new();
        let grid = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.char_indices()
                    .map(|(col, c)| {
                        let n = c.to_string().parse::<u8>().unwrap_or(20);
                        if n == 0 {
                            trailheads.push(Coord { row, col });
                        }
                        n
                    })
                    .collect()
            })
            .collect();

        Map { grid, trailheads }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    row: usize,
    col: usize,
}

type Trails = HashMap<Coord, HashSet<Coord>>;

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
