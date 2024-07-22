use anyhow::{anyhow, Error};
use std::str::FromStr;

use crate::coord::Coord;

pub struct Map {
    pub data: Vec<Vec<u8>>,
}

impl Map {
    pub fn get_value(&self, coord: &Coord) -> Option<u8> {
        self.data
            .get(coord.y as usize)?
            .get(coord.x as usize)
            .copied()
    }

    pub fn get_size(&self) -> Coord {
        Coord {
            x: self.data[0].len() as i32,
            y: self.data.len() as i32,
        }
    }

    pub fn is_within(&self, coord: &Coord) -> bool {
        let size = self.get_size();
        coord.x >= 0 && coord.x < size.x && coord.y >= 0 && coord.y < size.y
    }

    pub fn distance(&self, start: &Coord, end: &Coord) -> u32 {
        (start.x - end.x).unsigned_abs() + (start.y - end.y).unsigned_abs()
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let data: Result<Vec<Vec<u8>>, Self::Err> = input
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| {
                        c.to_digit(10)
                            .map(|c| c as u8)
                            .ok_or(anyhow!("Invalid digit: {}", c))
                    })
                    .collect()
            })
            .collect();

        Ok(Map { data: data? })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_from_str_valid_input() {
        let input = "123\n456\n789";
        let expected_data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let map: Map = input.parse().unwrap();
        assert_eq!(map.data, expected_data);
    }

    #[test]
    fn test_map_from_str_invalid_input() {
        let input = "123\n45a\n789";
        let map_result: Result<Map, _> = input.parse();
        assert!(map_result.is_err());
    }
}
