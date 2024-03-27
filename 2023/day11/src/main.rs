use std::{collections::HashSet, str::FromStr};

use anyhow::{bail, Error, Result};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn get_distance(&self, other: &Point) -> usize {
        (self.x as isize - other.x as isize).unsigned_abs()
            + (self.y as isize - other.y as isize).unsigned_abs()
    }
}

#[derive(Debug)]
struct Map {
    galaxies: Vec<Point>,
    empty_rows: HashSet<usize>,
    empty_cols: HashSet<usize>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let Some(first_line) = s.lines().next() else {
            bail!("Empty map.");
        };
        let width = first_line.len();

        let mut galaxies = Vec::new();
        let mut empty_rows: HashSet<usize> = (0..height).collect();
        let mut empty_cols: HashSet<usize> = (0..width).collect();

        for (y, row) in s.lines().enumerate() {
            for (x, c) in row.char_indices() {
                if c != '#' {
                    continue;
                }

                empty_rows.remove(&y);
                empty_cols.remove(&x);
                galaxies.push(Point { x, y });
            }
        }

        Ok(Map {
            galaxies,
            empty_rows,
            empty_cols,
        })
    }
}

impl Map {
    fn get_sum_paths(&self, factor: usize) -> usize {
        let galaxies = self.get_expanded_galaxies(factor);
        galaxies.iter().enumerate().fold(0, |acc, (i, galaxy)| {
            acc + galaxies[..i]
                .iter()
                .map(|other_galaxy| galaxy.get_distance(other_galaxy))
                .sum::<usize>()
        })
    }

    fn get_expanded_galaxies(&self, factor: usize) -> Vec<Point> {
        let mut new_galaxies = Vec::new();

        for galaxy in self.galaxies.iter() {
            new_galaxies.push(Point {
                x: galaxy.x
                    + self.empty_cols.iter().filter(|&&c| c < galaxy.x).count() * (factor - 1),
                y: galaxy.y
                    + self.empty_rows.iter().filter(|&&r| r < galaxy.y).count() * (factor - 1),
            });
        }

        new_galaxies
    }
}

fn part_1(input: &str) -> Result<usize> {
    let map = input.parse::<Map>()?;
    Ok(map.get_sum_paths(2))
}

fn part_2(input: &str) -> Result<usize> {
    let map = input.parse::<Map>()?;
    Ok(map.get_sum_paths(1000000))
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
    fn test_example() {
        let example = include_str!("example.txt");
        let result = part_1(example).unwrap();
        assert_eq!(374, result)
    }

    #[test]
    fn test_example_2() {
        let example = include_str!("example.txt");
        let map = example.parse::<Map>().unwrap();
        let result = map.get_sum_paths(10);
        assert_eq!(1030, result)
    }

    #[test]
    fn test_example_3() {
        let example = include_str!("example.txt");
        let map = example.parse::<Map>().unwrap();
        let result = map.get_sum_paths(100);
        assert_eq!(8410, result)
    }

    #[test]
    fn test_expansion() {
        let expected_expansion = include_str!("example_expanded.txt");
        let expected = expected_expansion.parse::<Map>().unwrap().galaxies;

        let example = include_str!("example.txt");
        let result = example.parse::<Map>().unwrap().get_expanded_galaxies(2);

        assert_eq!(expected, result)
    }
}
