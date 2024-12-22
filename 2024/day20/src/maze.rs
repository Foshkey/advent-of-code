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
    pub fn find_path(&self) -> Option<usize> {
        // This is your typical A* search algorithm
        let mut set = BTreeSet::from([(0, self.start)]);
        let mut steps = HashMap::from([(self.start, 0)]);

        while let Some((_, current)) = set.pop_first() {
            if current == self.end {
                return Some(steps[&self.end]);
            }

            for neighbor in self.get_neighbors(&current) {
                let tentative_steps = steps[&current] + 1;
                let is_better_path = steps
                    .get(&neighbor)
                    .map_or(true, |&steps| tentative_steps < steps);

                if is_better_path {
                    steps.insert(neighbor, tentative_steps);
                    set.insert((tentative_steps + neighbor.distance(&self.end), neighbor));
                }
            }
        }

        None
    }

    /// Finds all cheats that saves time
    /// Returns:
    ///     A list of time savings
    pub fn find_cheats(&self) -> Vec<usize> {
        let mut maze = self.clone();
        let mut time_savings = Vec::new();
        let Some(base_path) = self.find_path() else {
            return time_savings;
        };

        for (row, line) in self.grid.iter().enumerate() {
            for (col, &is_empty) in line.iter().enumerate() {
                // Skip if this isn't a wall
                if is_empty {
                    continue;
                }

                // Determine if this is a wall worth jumping through (empty on either side)
                let position = Coord { row, col };
                let neighbors = self.get_neighbors(&position);
                let empty_neighbors = neighbors.len() == 2
                    && (neighbors[0].row == neighbors[1].row
                        || neighbors[0].col == neighbors[1].col);
                if !empty_neighbors {
                    continue;
                }

                // Remove the wall and test
                *maze.get_mut(&position).unwrap() = true;
                let path = maze.find_path().unwrap();
                if path < base_path {
                    time_savings.push(base_path - path);
                }
                *maze.get_mut(&position).unwrap() = false;
            }
        }

        time_savings
    }

    fn get_neighbors(&self, position: &Coord) -> Vec<Coord> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(|(d_row, d_col)| self.get_relative(position, d_row, d_col))
            .collect()
    }

    fn get_relative(&self, position: &Coord, d_row: isize, d_col: isize) -> Option<Coord> {
        let row = position.row.checked_add_signed(d_row)?;
        let col = position.col.checked_add_signed(d_col)?;
        let &empty = self.grid.get(row)?.get(col)?;
        if empty {
            Some(Coord { col, row })
        } else {
            None
        }
    }

    fn get_mut(&mut self, position: &Coord) -> Option<&mut bool> {
        self.grid.get_mut(position.row)?.get_mut(position.col)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_item_ordering() {
        let position1 = Coord { col: 1, row: 1 };
        let position2 = Coord { col: 1, row: 2 };
        let position3 = Coord { col: 0, row: 0 };

        let mut queue = BTreeSet::new();
        queue.insert((3, position1));
        queue.insert((1, position1));
        queue.insert((2, position2));
        queue.insert((0, position3));

        assert_eq!(position3, queue.pop_first().unwrap().1);
        assert_eq!(position1, queue.pop_first().unwrap().1);
        assert_eq!(position2, queue.pop_first().unwrap().1);
        assert_eq!(position1, queue.pop_first().unwrap().1);
    }
}
