use std::{
    collections::{BTreeSet, HashMap},
    fmt::Display,
    str::FromStr,
};

#[derive(Clone, Debug)]
pub struct Maze {
    start: Coord,
    end: Coord,
    grid: Vec<Vec<bool>>,
}

impl Maze {
    pub fn find_paths(&self, cheat_length: usize, threshold: usize) -> usize {
        // First find the base path to determine the max cost
        let Some(base_path) = self.get_base_path() else {
            return 0;
        };

        let paths = self.find_cheat_paths(base_path, cheat_length, threshold);
        paths.len()
    }

    fn find_cheat_paths(
        &self,
        path: Vec<Coord>,
        cheat_length: usize,
        threshold: usize,
    ) -> Vec<usize> {
        let mut time_savings = Vec::new();

        // Go along the path finding cheat "pairs" where distances <= cheat length but it's worth it to cheat
        for a_i in 0..(path.len() - threshold) {
            let a = path[a_i];
            for (b_i, b) in path.iter().enumerate().skip(threshold + a_i) {
                let d = a.distance(b);
                if d <= cheat_length {
                    let savings = b_i - a_i - d;
                    if savings >= threshold {
                        time_savings.push(savings)
                    }
                }
            }
        }

        time_savings
    }

    fn get_base_path(&self) -> Option<Vec<Coord>> {
        // This is your typical A* search algorithm
        let mut open_set = BTreeSet::from([(0, self.start)]);
        let mut steps = HashMap::from([(self.start, 0)]);
        let mut came_from = HashMap::new();

        while let Some((_, current)) = open_set.pop_first() {
            if current == self.end {
                let mut current = current;
                let mut path = Vec::from([self.end]);
                while let Some(&previous) = came_from.get(&current) {
                    path.push(previous);
                    current = previous;
                }
                path.reverse();
                return Some(path);
            }

            for neighbor in self
                .get_neighbors(&current)
                .into_iter()
                .filter_map(|(n, is_empty)| if is_empty { Some(n) } else { None })
            {
                let tentative_steps = steps[&current] + 1;
                let is_better_path = steps
                    .get(&neighbor)
                    .is_none_or(|&steps| tentative_steps < steps);

                if is_better_path {
                    came_from.insert(neighbor, current);
                    steps.insert(neighbor, tentative_steps);
                    open_set.insert((tentative_steps + neighbor.distance(&self.end), neighbor));
                }
            }
        }

        None
    }

    fn get_neighbors(&self, position: &Coord) -> Vec<(Coord, bool)> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(|(d_row, d_col)| self.get_relative(position, d_row, d_col))
            .collect()
    }

    fn get_relative(&self, position: &Coord, d_row: isize, d_col: isize) -> Option<(Coord, bool)> {
        let row = position.row.checked_add_signed(d_row)?;
        let col = position.col.checked_add_signed(d_col)?;
        let &is_empty = self.grid.get(row)?.get(col)?;
        Some((Coord { col, row }, is_empty))
    }
}

impl FromStr for Maze {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut end = None;
        let grid = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, ch)| match ch {
                        'S' => {
                            start = Some(Coord { row, col });
                            Ok(true)
                        }
                        'E' => {
                            end = Some(Coord { row, col });
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
        let end = end.ok_or("End position not found")?;

        Ok(Maze { start, end, grid })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    pub fn distance(&self, other: &Coord) -> usize {
        (self.col as isize - other.col as isize).unsigned_abs()
            + (self.row as isize - other.row as isize).unsigned_abs()
    }
}

impl From<&str> for Coord {
    fn from(s: &str) -> Self {
        let (x, y) = s.split_once(',').unwrap();
        Coord {
            col: x.parse().unwrap(),
            row: y.parse().unwrap(),
        }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.col, self.row)
    }
}
