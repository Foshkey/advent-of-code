use std::{cell::RefCell, collections::HashSet, fmt::Debug, str::FromStr};

use anyhow::{bail, Error, Result};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn reverse(&self) -> Direction {
        match &self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    fn step(&self) -> (i8, i8) {
        match &self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

#[derive(Clone, Eq)]
struct Tile {
    x: usize,
    y: usize,
    connecting: HashSet<Direction>,
    is_on_loop: RefCell<bool>,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tile")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl Tile {
    fn new(x: usize, y: usize, c: char) -> Self {
        let connecting = match c {
            '|' => vec![Direction::North, Direction::South],
            '-' => vec![Direction::East, Direction::West],
            'L' => vec![Direction::North, Direction::East],
            'J' => vec![Direction::North, Direction::West],
            '7' => vec![Direction::South, Direction::West],
            'F' => vec![Direction::South, Direction::East],
            '.' => vec![],
            'S' => vec![
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ],
            _ => panic!("Invalid character {c} at position {x}, {y}"),
        };

        Tile {
            x,
            y,
            connecting: connecting.into_iter().collect(),
            is_on_loop: RefCell::new(false),
        }
    }

    fn next_direction(&self, prev_direction: Direction) -> Result<Direction> {
        Ok(*self
            .connecting
            .iter()
            .find(|&&d| d != prev_direction)
            .ok_or(Error::msg(format!(
                "Could not find valid connection for {:?}",
                self
            )))?)
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Map {
    grid: Vec<Vec<Tile>>,
    starting_tile: Tile,
    loop_traced: RefCell<bool>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::<Vec<Tile>>::new();
        let mut starting = None;
        for (x, line) in s.lines().enumerate() {
            let mut row = Vec::<Tile>::new();
            for (y, c) in line.char_indices() {
                let tile = Tile::new(x, y, c);
                if c == 'S' {
                    starting = Some(tile.clone());
                }
                row.push(tile);
            }
            grid.push(row);
        }

        if let Some(starting_tile) = starting {
            Ok(Map {
                grid,
                starting_tile,
                loop_traced: RefCell::from(false),
            })
        } else {
            bail!("Could not find starting tile.")
        }
    }
}

impl Map {
    fn trace_loop(&self) -> Result<()> {
        // This will walk the loop and mark all tiles with whether they're on the loop.
        // If we already traced the loop, then return.
        if *self.loop_traced.borrow() {
            return Ok(());
        }

        // Start with the starting tile's first valid connection
        let mut direction = *self
            .get_valid_connections(&self.starting_tile)
            .iter()
            .next()
            .ok_or(Error::msg(format!(
                "Could not find valid connection for tile {:?}",
                self.starting_tile
            )))?;

        // Get the adjacent tile
        let mut current_tile = self
            .get_adjacent_tile(&self.starting_tile, direction)
            .ok_or(Error::msg(format!(
                "Could not find adjacent for tile {:?}",
                self.starting_tile
            )))?;

        // Mark the current tile as on the loop
        *current_tile.is_on_loop.borrow_mut() = true;

        // Repeat until we're back at the starting tile
        while *current_tile != self.starting_tile {
            direction = current_tile.next_direction(direction.reverse())?;
            current_tile = self
                .get_adjacent_tile(current_tile, direction)
                .ok_or(Error::msg(format!(
                    "Could not find adjacent tile for tile {:?}",
                    current_tile
                )))?;
            *current_tile.is_on_loop.borrow_mut() = true;
        }

        // Finally, indicate that the loop has been traced.
        *self.loop_traced.borrow_mut() = true;

        Ok(())
    }

    fn get_adjacent_tile<'a>(&'a self, tile: &'a Tile, direction: Direction) -> Option<&Tile> {
        let (dx, dy) = direction.step();

        let new_x = tile.x as i32 + dx as i32;
        if new_x < 0 || new_x >= self.grid.len() as i32 {
            return None;
        }

        let new_y = tile.y as i32 + dy as i32;
        if new_y < 0 || new_y >= self.grid[0].len() as i32 {
            return None;
        }

        Some(&self.grid[new_x as usize][new_y as usize])
    }

    fn get_valid_connections(&self, tile: &Tile) -> HashSet<Direction> {
        let mut valid_connections = HashSet::<Direction>::new();
        for &connection in &tile.connecting {
            if let Some(connecting_tile) = self.get_adjacent_tile(tile, connection) {
                if connecting_tile.connecting.contains(&connection.reverse()) {
                    valid_connections.insert(connection);
                }
            }
        }
        valid_connections
    }

    fn get_farthest_point(&self) -> Result<usize> {
        self.trace_loop()?;
        Ok(self.count(|tile| *tile.is_on_loop.borrow()) / 2)
    }

    fn count<F>(&self, predicate: F) -> usize
    where
        F: Fn(&Tile) -> bool,
    {
        self.grid.iter().fold(0, |acc, row| {
            acc + row.iter().filter(|&tile| predicate(tile)).count()
        })
    }

    fn count_inside_tiles(&self) -> Result<usize> {
        self.trace_loop()?;
        Ok(self.count(|tile| self.is_tile_inside(tile)))
    }

    fn is_tile_inside(&self, tile: &Tile) -> bool {
        // If tile is on the loop, return false
        if *tile.is_on_loop.borrow() {
            return false;
        }

        // Use the ray algorithm to count how many intersections are on a ray to the edge
        let row = &self.grid[tile.x][..tile.y];
        let intersections = row
            .iter()
            .filter(|&t| {
                // We just need to count number of tiles on the loop that has a northern connection.
                *t.is_on_loop.borrow() && self.get_valid_connections(t).contains(&Direction::North)
            })
            .count();

        // If odd, the tile is inside
        intersections % 2 == 1
    }
}

fn solution_part1(input: &str) -> Result<usize> {
    let map = input.parse::<Map>()?;
    map.get_farthest_point()
}

fn solution_part2(input: &str) -> Result<usize> {
    let map = input.parse::<Map>()?;
    map.count_inside_tiles()
}

fn main() {
    let input = include_str!("input.txt");
    let result_pt1 = solution_part1(input);
    println!("Part 1: {result_pt1:?}");
    let result_pt2 = solution_part2(input);
    println!("Part 1: {result_pt2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = include_str!("example1.txt");
        let result = solution_part1(input).unwrap();
        assert_eq!(4, result);
    }

    #[test]
    fn test_example2() {
        let input = include_str!("example2.txt");
        let result = solution_part1(input).unwrap();
        assert_eq!(8, result);
    }

    #[test]
    fn test_example3() {
        let input = include_str!("example3.txt");
        let result = solution_part2(input).unwrap();
        assert_eq!(4, result);
    }

    #[test]
    fn test_example4() {
        let input = include_str!("example4.txt");
        let result = solution_part2(input).unwrap();
        assert_eq!(10, result);
    }
}
