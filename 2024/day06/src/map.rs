use std::collections::HashSet;

#[derive(Debug)]
pub enum MapError {
    LoopingPath,
}

#[derive(Clone, PartialEq)]
pub enum Tile {
    Empty,
    Obstacle,
    GuardPath(HashSet<(isize, isize)>),
}

#[derive(Clone)]
pub struct Map {
    pub grid: Vec<Vec<Tile>>,
    pub guard_coord: (usize, usize),
    guard_direction: (isize, isize),
}

impl Map {
    pub fn trace_guard_path(&self) -> Result<usize, MapError> {
        let mut map = self.clone();
        map.trace_guard_path_mut()
    }

    fn trace_guard_path_mut(&mut self) -> Result<usize, MapError> {
        while let Some((next_tile, next_coord)) = self.get_next(self.guard_direction) {
            if *next_tile == Tile::Obstacle {
                self.turn_guard_right();
                continue;
            }

            if let Tile::GuardPath(directions) = next_tile {
                if directions.contains(&self.guard_direction) {
                    return Err(MapError::LoopingPath);
                }
            }

            self.mark_guard_position();
            self.guard_coord = next_coord;
        }

        self.mark_guard_position();
        let num_marked_tiles = self
            .grid
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|tile| matches!(tile, Tile::GuardPath(..)))
                    .count()
            })
            .sum();
        Ok(num_marked_tiles)
    }

    fn get_next(&self, direction: (isize, isize)) -> Option<(&Tile, (usize, usize))> {
        let (row, col) = self.guard_coord;
        let (d_row, d_col) = direction;
        let new_row = row.checked_add_signed(d_row)?;
        let new_col = col.checked_add_signed(d_col)?;
        let tile = self.grid.get(new_row)?.get(new_col)?;

        Some((tile, (new_row, new_col)))
    }

    fn turn_guard_right(&mut self) {
        self.guard_direction = match self.guard_direction {
            (-1, 0) => (0, 1),
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            (d_row, d_col) => (d_row, d_col),
        };
    }

    fn mark_guard_position(&mut self) {
        let (row, col) = self.guard_coord;
        let Some(row) = self.grid.get_mut(row) else {
            return;
        };
        let Some(tile) = row.get_mut(col) else {
            return;
        };
        let direction = self.guard_direction;

        match tile {
            Tile::GuardPath(directions) => {
                directions.insert(direction);
            }
            _ => {
                *tile = Tile::GuardPath(HashSet::from([direction]));
            }
        }
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut guard_coord = (0, 0);
        let grid = value
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.char_indices()
                    .map(|(col, c)| {
                        if c == '^' {
                            guard_coord = (row, col)
                        };

                        match c {
                            '.' | '^' => Tile::Empty,
                            _ => Tile::Obstacle,
                        }
                    })
                    .collect()
            })
            .collect();

        Map {
            grid,
            guard_coord,
            guard_direction: (-1, 0),
        }
    }
}
