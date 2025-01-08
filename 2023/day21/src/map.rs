use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct Map {
    start: Coord,
    grid: Vec<Vec<bool>>,
    size: usize,
}

impl Map {
    pub fn get_num_spaces(&self, distance: usize) -> usize {
        let mut set = HashSet::from([self.start]);

        for _ in 0..distance {
            let mut new_set = HashSet::new();
            for position in set {
                new_set.extend(self.get_neighbors(&position));
            }
            set = new_set;
        }

        set.len()
    }

    fn get_neighbors(&self, position: &Coord) -> HashSet<Coord> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(|(d_row, d_col)| self.get_relative(position, d_row, d_col))
            .collect()
    }

    fn get_relative(&self, position: &Coord, d_row: isize, d_col: isize) -> Option<Coord> {
        let row = position.row + d_row;
        let row_vec = self.grid.get(self.get_index(row))?;
        let col = position.col + d_col;
        let is_empty = row_vec.get(self.get_index(col))?;
        if *is_empty {
            Some(Coord { col, row })
        } else {
            None
        }
    }

    fn get_index(&self, index: isize) -> usize {
        let result = index % self.size as isize;
        (if result < 0 {
            result + self.size as isize
        } else {
            result
        }) as usize
    }
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let mut start = Coord { row: 0, col: 0 };
        let grid: Vec<Vec<bool>> = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, ch)| match ch {
                        'S' => {
                            start = Coord {
                                row: row as isize,
                                col: col as isize,
                            };
                            true
                        }
                        '.' => true,
                        _ => false,
                    })
                    .collect()
            })
            .collect();

        Map {
            start,
            size: grid.len(),
            grid,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Coord {
    row: isize,
    col: isize,
}
