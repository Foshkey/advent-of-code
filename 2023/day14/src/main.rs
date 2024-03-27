use std::{collections::HashMap, fmt::Display, str::FromStr, time::Instant};

use anyhow::{anyhow, bail, Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn get_delta(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }

    fn reverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    row: usize,
    col: usize,
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Tile {
    RoundRock,
    CubeRock,
    Empty,
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::RoundRock => 'O',
            Tile::CubeRock => '#',
            Tile::Empty => '.',
        }
    }
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'O' => Tile::RoundRock,
            '#' => Tile::CubeRock,
            '.' => Tile::Empty,
            _ => bail!("Unknown tile: {}", value),
        })
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Platform {
    grid: Vec<Vec<Tile>>,
}

impl FromStr for Platform {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Platform {
            grid: s
                .lines()
                .map(|l| l.chars().map(Tile::try_from).collect::<Result<Vec<Tile>>>())
                .collect::<Result<Vec<Vec<Tile>>>>()?,
        })
    }
}

impl ToString for Platform {
    fn to_string(&self) -> String {
        self.grid
            .iter()
            .map(|row| row.iter().map(|&t| char::from(t)).collect())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Platform {
    fn tilt(&mut self, direction: Direction) -> Result<()> {
        let row_len = self.grid.len();
        if row_len == 0 {
            return Ok(());
        }
        let col_len = self.grid[0].len();

        // Determine how far to loop based on direction
        let end = if direction == Direction::North || direction == Direction::South {
            col_len
        } else {
            row_len
        };

        for i in 0..end {
            // Determine starting position
            let mut prev_rock_coord = match direction {
                Direction::North => Coord { row: 0, col: i },
                Direction::East => Coord {
                    row: i,
                    col: col_len - 1,
                },
                Direction::South => Coord {
                    row: row_len - 1,
                    col: i,
                },
                Direction::West => Coord { row: i, col: 0 },
            };

            // While we can find rocks, move them to the furthest empty spot
            while let Some(rock_coord) =
                self.find_next(Tile::RoundRock, prev_rock_coord, direction.reverse())
            {
                if let Some(empty_spot) = self.find_furthest(Tile::Empty, rock_coord, direction) {
                    self.move_rock(rock_coord, empty_spot)?;
                }
                prev_rock_coord = rock_coord;
            }
        }

        Ok(())
    }

    fn spin(&mut self) -> Result<()> {
        self.tilt(Direction::North)?;
        self.tilt(Direction::West)?;
        self.tilt(Direction::South)?;
        self.tilt(Direction::East)?;
        Ok(())
    }

    fn move_rock(&mut self, from: Coord, to: Coord) -> Result<()> {
        if self
            .grid
            .get(from.row)
            .ok_or(anyhow!("From ({from}) outside bounds of platform"))?
            .get(from.col)
            .ok_or(anyhow!("From ({from}) outside bounds of platform"))?
            == &Tile::Empty
        {
            bail!("Attempted to move from {from} which is empty")
        }

        if self
            .grid
            .get(to.row)
            .ok_or(anyhow!("To ({to}) outside bounds of platform"))?
            .get(to.col)
            .ok_or(anyhow!("To ({to}) outside bounds of platform"))?
            != &Tile::Empty
        {
            bail!("Attempted to move to {to} which is non-empty");
        }

        self.grid[to.row][to.col] = self.grid[from.row][from.col];
        self.grid[from.row][from.col] = Tile::Empty;

        Ok(())
    }

    fn calculate_load_north(&self) -> usize {
        let row_len = self.grid.len();
        self.grid
            .iter()
            .enumerate()
            .map(|(r, row)| (row_len - r) * row.iter().filter(|&&t| t == Tile::RoundRock).count())
            .sum()
    }

    fn find_next(&self, tile_to_find: Tile, start: Coord, direction: Direction) -> Option<Coord> {
        let mut current = start;
        while let Some((tile, coord)) = self.get_next_tile(current, direction) {
            if tile == tile_to_find {
                return Some(coord);
            }
            current = coord;
        }
        None
    }

    fn find_furthest(
        &self,
        tile_to_find: Tile,
        start: Coord,
        direction: Direction,
    ) -> Option<Coord> {
        let mut current = start;
        let mut empty_tile = None;
        while let Some((tile, coord)) = self.get_next_tile(current, direction) {
            if tile == tile_to_find {
                empty_tile = Some(coord);
            } else {
                return empty_tile;
            }
            current = coord;
        }
        empty_tile
    }

    fn get_next_tile(&self, current: Coord, direction: Direction) -> Option<(Tile, Coord)> {
        let (dr, dc) = direction.get_delta();
        let new_row = usize::try_from(current.row as isize + dr).ok()?;
        let new_col = usize::try_from(current.col as isize + dc).ok()?;

        let Some(tile) = self.grid.get(new_row)?.get(new_col) else {
            return None;
        };

        Some((
            *tile,
            Coord {
                row: new_row,
                col: new_col,
            },
        ))
    }
}

fn part_1(input: &str) -> Result<usize> {
    let mut platform = input.parse::<Platform>()?;
    platform.tilt(Direction::North)?;
    Ok(platform.calculate_load_north())
}

fn part_2(input: &str) -> Result<usize> {
    const SPINS: i32 = 1000000000;
    let mut prev_platforms = HashMap::new();
    let mut platform = input.parse::<Platform>()?;
    prev_platforms.insert(platform.clone(), 0);
    for index in 1..=SPINS {
        platform.spin()?;
        if let Some(prev_index) = prev_platforms.get(&platform) {
            // Cycle is happening, find what would be the final cycle
            let end_i = prev_index + (SPINS - prev_index) % (index - prev_index);
            if let Some((end_platform, _)) = prev_platforms.iter().find(|&(_, &i)| i == end_i) {
                platform = end_platform.clone();
                break;
            }
        }
        prev_platforms.insert(platform.clone(), index);
    }
    Ok(platform.calculate_load_north())
}

fn main() {
    let now = Instant::now();
    let input = include_str!("input.txt");
    println!("Part 1: {:?}", part_1(input));
    println!("Part 2: {:?}", part_2(input));
    println!("Done in {}ms", now.elapsed().as_millis())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let result = part_1(input);
        assert_eq!(136, result.unwrap());
    }

    #[test]
    fn test_example_roll() {
        let input = include_str!("example.txt");
        let expected = include_str!("example_rolled_north.txt")
            .parse::<Platform>()
            .unwrap();

        let mut platform = input.parse::<Platform>().unwrap();
        platform.tilt(Direction::North).unwrap();

        assert_eq!(expected, platform);
    }

    #[test]
    fn test_move_rock() {
        let mut platform = "..O\n#..".parse::<Platform>().unwrap();
        platform
            .move_rock(Coord { row: 0, col: 2 }, Coord { row: 1, col: 1 })
            .unwrap();

        assert_eq!("...\n#O.", &platform.to_string())
    }

    #[test]
    fn test_spin_example() {
        let expected1 = include_str!("example_spin1.txt")
            .parse::<Platform>()
            .unwrap();
        let expected2 = include_str!("example_spin2.txt")
            .parse::<Platform>()
            .unwrap();
        let expected3 = include_str!("example_spin3.txt")
            .parse::<Platform>()
            .unwrap();
        let mut platform = include_str!("example.txt").parse::<Platform>().unwrap();

        platform.spin().unwrap();
        assert_eq!(expected1, platform);
        platform.spin().unwrap();
        assert_eq!(expected2, platform);
        platform.spin().unwrap();
        assert_eq!(expected3, platform);
    }
}
