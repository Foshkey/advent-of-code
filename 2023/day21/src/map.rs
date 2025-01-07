use std::{collections::HashSet, str::FromStr};

#[derive(Clone, Debug)]
pub struct Map {
    start: Coord,
    grid: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn get_num_spaces(&self, distance: usize) -> usize {
        let mut set = HashSet::from([self.start]);

        for _ in 0..distance {
            set = set.iter().fold(HashSet::new(), |set, position| {
                set.union(&self.get_neighbors(position)).copied().collect()
            });
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
        let row_vec = self.grid.get(get_index(row, self.height))?;
        let col = position.col + d_col;
        let is_empty = row_vec.get(get_index(col, self.width))?;
        if *is_empty {
            Some(Coord { col, row })
        } else {
            None
        }
    }
}

fn get_index(index: isize, size: usize) -> usize {
    let result = index % size as isize;
    (if result < 0 {
        result + size as isize
    } else {
        result
    }) as usize
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let grid = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, ch)| match ch {
                        'S' => {
                            start = Some(Coord {
                                row: row as isize,
                                col: col as isize,
                            });
                            Ok(true)
                        }
                        '.' => Ok(true),
                        '#' => Ok(false),
                        _ => Err(format!("Invalid character at row {row}, col {col}: {ch}")),
                    })
                    .collect::<Result<Vec<bool>, Self::Err>>()
            })
            .collect::<Result<Vec<Vec<bool>>, Self::Err>>()?;

        let start = start.ok_or("Start position not found")?;

        Ok(Map {
            start,
            width: grid.len(),
            height: grid.first().unwrap().len(),
            grid,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Coord {
    row: isize,
    col: isize,
}
