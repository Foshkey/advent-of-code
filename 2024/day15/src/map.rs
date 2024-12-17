use std::fmt::{Display, Formatter};

use crate::tile::Tile;

#[derive(Debug)]
pub struct Map {
    robot_position: (usize, usize),
    grid: Vec<Vec<Tile>>,
}

impl Map {
    pub fn move_robot(&mut self, delta: (isize, isize)) {
        if !self.has_room(self.robot_position, delta) {
            return;
        }

        let mut new_position = add(self.robot_position, delta);
        self.robot_position = new_position;

        let mut tile = Tile::Empty;
        loop {
            tile = self.replace_tile(new_position, tile);
            if tile == Tile::Empty {
                break;
            }
            new_position = add(new_position, delta);
        }
    }

    pub fn widen(&mut self) {
        let mut new_grid = Vec::new();

        for row in &self.grid {
            let mut new_row = Vec::new();
            for &tile in row {
                if tile == Tile::Box {
                    new_row.push(Tile::LeftBox);
                    new_row.push(Tile::RightBox);
                    continue;
                }

                new_row.push(tile);
                new_row.push(tile);
            }
            new_grid.push(new_row);
        }

        self.grid = new_grid;
        self.robot_position.1 *= 2
    }

    pub fn get_gps_sum(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .map(|(col, &tile)| {
                        if tile == Tile::Box || tile == Tile::LeftBox {
                            row * 100 + col
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn has_room(&self, position: (usize, usize), delta: (isize, isize)) -> bool {
        let (next_tile_position, next_tile) = self.get_tile_relative(position, delta);
        if next_tile == Tile::Obstacle {
            return false;
        }

        if next_tile == Tile::Empty {
            return true;
        }

        self.has_room(next_tile_position, delta)
    }

    fn get_tile_relative(
        &self,
        position: (usize, usize),
        delta: (isize, isize),
    ) -> ((usize, usize), Tile) {
        let row = (position.0 as isize + delta.0) as usize;
        let col = (position.1 as isize + delta.1) as usize;
        ((row, col), self.grid[row][col])
    }

    fn replace_tile(&mut self, position: (usize, usize), tile: Tile) -> Tile {
        let old_tile = self.grid[position.0][position.1];
        self.grid[position.0][position.1] = tile;
        old_tile
    }
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let mut robot_position = (0, 0);
        let grid = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.char_indices()
                    .map(|(col, c)| {
                        if c == '@' {
                            robot_position = (row, col);
                        }

                        c.into()
                    })
                    .collect()
            })
            .collect();

        Map {
            robot_position,
            grid,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (robot_row, robot_col) = self.robot_position;

        for (row, line) in self.grid.iter().enumerate() {
            for (col, &tile) in line.iter().enumerate() {
                if row == robot_row && col == robot_col {
                    write!(f, "@")?;
                    continue;
                }

                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn add(position: (usize, usize), delta: (isize, isize)) -> (usize, usize) {
    let row = (position.0 as isize + delta.0) as usize;
    let col = (position.1 as isize + delta.1) as usize;
    (row, col)
}
