use std::{cmp::max, collections::HashSet, str::FromStr};

use anyhow::{bail, Error, Ok, Result};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn reflect(&self, c: char) -> Self {
        match (self, c) {
            (Direction::Up, '\\') => Direction::Left,
            (Direction::Down, '\\') => Direction::Right,
            (Direction::Left, '\\') => Direction::Up,
            (Direction::Right, '\\') => Direction::Down,
            (Direction::Up, '/') => Direction::Right,
            (Direction::Down, '/') => Direction::Left,
            (Direction::Left, '/') => Direction::Down,
            (Direction::Right, '/') => Direction::Up,
            _ => panic!("Unexpected character {c}"),
        }
    }

    fn split(&self, c: char) -> Option<[Self; 2]> {
        match (self, c) {
            (Direction::Up | Direction::Down, '-') => Some([Direction::Left, Direction::Right]),
            (Direction::Left | Direction::Right, '|') => Some([Direction::Up, Direction::Down]),
            (_, '-' | '|') => None,
            _ => panic!("Unexpected character {c}"),
        }
    }
}

#[derive(Debug, Clone)]
enum TileType {
    Empty,
    Mirror(char),
    Splitter(char),
}

#[derive(Debug, Clone)]
struct Tile {
    tile_type: TileType,
    energized_directions: HashSet<Direction>,
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        let tile_type = match value {
            '.' => TileType::Empty,
            '|' | '-' => TileType::Splitter(value),
            '\\' | '/' => TileType::Mirror(value),
            _ => bail!("Unknown character: {value}"),
        };

        Ok(Tile {
            tile_type,
            energized_directions: HashSet::new(),
        })
    }
}

impl Tile {
    fn is_energized(&self) -> bool {
        !self.energized_directions.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    row: usize,
    col: usize,
}

type Beam = (Coord, Direction);

#[derive(Debug, Clone)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
}

impl FromStr for Grid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let tiles = s
            .lines()
            .map(|l| l.chars().map(Tile::try_from).collect::<Result<Vec<Tile>>>())
            .collect::<Result<Vec<Vec<Tile>>>>()?;

        Ok(Grid { tiles })
    }
}

impl Grid {
    fn energize(&mut self, start: Beam) {
        let mut beam_queue: Vec<Beam> = vec![start];

        while let Some((coord, direction)) = beam_queue.pop() {
            let Some(tile) = self.get_tile(coord) else {
                continue;
            };

            if !tile.energized_directions.insert(direction) {
                // Already energized from this direction, break out
                continue;
            }

            let new_directions = match tile.tile_type {
                TileType::Mirror(c) => vec![direction.reflect(c)],
                TileType::Splitter(c) => {
                    if let Some(directions) = direction.split(c) {
                        directions.to_vec()
                    } else {
                        vec![direction]
                    }
                }
                TileType::Empty => vec![direction],
            };

            for new_direction in new_directions {
                let Some(new_coord) = self.get_next_coord(coord, new_direction) else {
                    continue;
                };
                beam_queue.push((new_coord, new_direction));
            }
        }
    }

    fn get_tile(&mut self, coord: Coord) -> Option<&mut Tile> {
        let row = self.tiles.get_mut(coord.row)?;
        row.get_mut(coord.col)
    }

    fn get_next_coord(&self, current: Coord, direction: Direction) -> Option<Coord> {
        let (dr, dc) = direction.get_delta();
        let new_row = usize::try_from(current.row as isize + dr).ok()?;
        let new_col = usize::try_from(current.col as isize + dc).ok()?;
        Some(Coord {
            row: new_row,
            col: new_col,
        })
    }

    fn count_energized(&self) -> usize {
        self.tiles
            .iter()
            .flatten()
            .filter(|t| t.is_energized())
            .count()
    }
}

fn part_1(input: &str) -> Result<usize> {
    let mut grid: Grid = input.parse()?;
    let start_coord = Coord { row: 0, col: 0 };
    let start_direction = Direction::Right;
    grid.energize((start_coord, start_direction));
    Ok(grid.count_energized())
}

fn part_2(input: &str) -> Result<usize> {
    let original_grid: Grid = input.parse()?;
    let mut highest_energy = 0;
    let row_len = original_grid.tiles.len();
    if row_len == 0 {
        bail!("No rows");
    }
    let col_len = original_grid.tiles[0].len();
    if col_len == 0 {
        bail!("No columns");
    }

    for row in 0..row_len {
        // Right
        let mut grid = original_grid.clone();
        grid.energize((Coord { row, col: 0 }, Direction::Right));
        highest_energy = max(highest_energy, grid.count_energized());

        // Left
        let mut grid = original_grid.clone();
        grid.energize((
            Coord {
                row,
                col: col_len - 1,
            },
            Direction::Left,
        ));
        highest_energy = max(highest_energy, grid.count_energized());
    }

    for col in 0..col_len {
        // Down
        let mut grid = original_grid.clone();
        grid.energize((Coord { row: 0, col }, Direction::Down));
        highest_energy = max(highest_energy, grid.count_energized());

        // Up
        let mut grid = original_grid.clone();
        grid.energize((
            Coord {
                row: row_len - 1,
                col,
            },
            Direction::Up,
        ));
        highest_energy = max(highest_energy, grid.count_energized());
    }

    Ok(highest_energy)
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {:?}", part_1(input));
    println!("Part 2: {:?}", part_2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("example.txt");
        let result = part_1(input);
        assert_eq!(46, result.unwrap());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("example.txt");
        let result = part_2(input);
        assert_eq!(51, result.unwrap());
    }
}
