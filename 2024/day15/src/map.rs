use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use crate::tile::Tile;

#[derive(Debug)]
pub struct Map {
    robot_position: (usize, usize),
    grid: Vec<Vec<Tile>>,
}

impl Map {
    pub fn move_robot(&mut self, delta: (isize, isize)) {
        // First check if there's room
        if !self.has_room(self.robot_position, delta) {
            return;
        }

        // Move the robot
        let new_position = add(self.robot_position, delta);
        self.robot_position = new_position;

        // Tiles is a hashmap where the key is the position that the tile (value) needs to go
        let mut tiles = HashMap::new();

        // The robot needs an empty spot to move to, so start with that
        tiles.insert(new_position, Tile::Empty);

        // Loop while we have tiles in hand
        while !tiles.is_empty() {
            let mut new_tiles = HashMap::new();
            let mut other_big_box_tiles = Vec::new();

            // Go through each tile that we need to place
            for (position, tile) in tiles {
                // Replace the tile and deal with it
                let tile = self.replace_tile(position, tile);

                // If empty, cool we're done
                if tile != Tile::Empty {
                    continue;
                }

                // If it wasn't empty, then we need to add it to the next set of tiles that we need to place
                new_tiles.insert(add(position, delta), tile);

                // If we're moving up/down and this tile is part of a big box, then the other tile needs to move as well
                if delta.0 != 0 && tile.is_big_box() {
                    let other_position = self.get_other_big_box_position(position, tile);
                    other_big_box_tiles.push(other_position);
                }
            }

            // Deal with the other big box tiles that we need to move
            for other in other_big_box_tiles {
                let new_position = add(other, delta);
                // If it hasn't already been added, then add it
                new_tiles
                    .entry(new_position)
                    .or_insert_with(|| self.replace_tile(other, Tile::Empty));
            }

            // Continue on with the next set of tiles
            tiles = new_tiles;
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

        // If we're moving up/down, then we need to find other halves of big boxes
        if delta.0 != 0 && next_tile.is_big_box() {
            let other_position = self.get_other_big_box_position(next_tile_position, next_tile);
            return self.has_room(next_tile_position, delta)
                && self.has_room(other_position, delta);
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

    fn get_other_big_box_position(&self, position: (usize, usize), tile: Tile) -> (usize, usize) {
        if tile == Tile::LeftBox {
            (position.0, position.1 + 1)
        } else {
            (position.0, position.1 - 1)
        }
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
