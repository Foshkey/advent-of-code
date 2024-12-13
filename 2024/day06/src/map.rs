use crate::tracker;

#[derive(PartialEq)]
pub enum Tile {
    Empty,
    Obstacle,
    Path,
}

pub struct Map {
    pub grid: Vec<Vec<Tile>>,
    pub guard_coord: (usize, usize),
    guard_direction: (isize, isize),
    original_guard_coord: (usize, usize),
    tracker: tracker::Tracker,
}

impl Map {
    pub fn reset(&mut self) {
        self.guard_coord = self.original_guard_coord;
        self.guard_direction = (-1, 0);
        self.tracker = tracker::Tracker::new()
    }

    pub fn get_guard_path_count(&mut self) -> usize {
        while self.move_to_next_obstacle(true) {
            self.turn_guard_right();
        }

        self.grid
            .iter()
            .map(|row| row.iter().filter(|&tile| *tile == Tile::Path).count())
            .sum()
    }

    pub fn is_guard_path_loop(&mut self) -> bool {
        while self.move_to_next_obstacle(false) {
            self.turn_guard_right();

            if !self.tracker.mark(self.guard_coord, self.guard_direction) {
                return true;
            }
        }

        false
    }

    fn move_to_next_obstacle(&mut self, mark_path: bool) -> bool {
        let mut is_obstacle = false;

        loop {
            if mark_path {
                let (row, col) = self.guard_coord;
                self.grid[row][col] = Tile::Path;
            }

            let Some((next_tile, next_coord)) = self.get_next() else {
                break;
            };

            if *next_tile == Tile::Obstacle {
                is_obstacle = true;
                break;
            }

            self.guard_coord = next_coord;
        }

        is_obstacle
    }

    fn get_next(&self) -> Option<(&Tile, (usize, usize))> {
        let (row, col) = self.guard_coord;
        let (d_row, d_col) = self.guard_direction;
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
            original_guard_coord: guard_coord,
            tracker: tracker::Tracker::new(),
        }
    }
}
