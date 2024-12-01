use std::{collections::HashMap, str::FromStr};

const INPUT: &str = include_str!("input.txt");

struct Lists {
    left: Vec<usize>,
    right: Vec<usize>,
    right_count: HashMap<usize, usize>,
}

impl Lists {
    fn get_total_distance(&self) -> usize {
        self.left
            .iter()
            .enumerate()
            .map(|(index, &left_num)| {
                (left_num as isize - self.right[index] as isize).unsigned_abs()
            })
            .sum()
    }

    fn get_similarity_score(&self) -> u128 {
        self.left
            .iter()
            .map(|&left_num| {
                left_num as u128 * *self.right_count.get(&left_num).unwrap_or(&0) as u128
            })
            .sum()
    }
}

impl FromStr for Lists {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left = Vec::<usize>::new();
        let mut right = Vec::<usize>::new();
        let mut right_count = HashMap::<usize, usize>::new();

        for line in s.lines() {
            let mut parts = line.split_whitespace();
            if let (Some(left_str), Some(right_str)) = (parts.next(), parts.next()) {
                let left_int = left_str.parse::<usize>()?;
                let right_int = right_str.parse::<usize>()?;
                left.push(left_int);
                right.push(right_int);

                if let Some(count) = right_count.get_mut(&right_int) {
                    *count += 1;
                } else {
                    right_count.insert(right_int, 1);
                }
            }
        }

        left.sort();
        right.sort();

        Ok(Lists {
            left,
            right,
            right_count,
        })
    }
}

fn part_1() -> Result<usize, std::num::ParseIntError> {
    Ok(Lists::from_str(INPUT)?.get_total_distance())
}
fn part_2() -> Result<u128, std::num::ParseIntError> {
    Ok(Lists::from_str(INPUT)?.get_similarity_score())
}

fn main() {
    println!("Part 1: {:?}", part_1());
    println!("Part 2: {:?}", part_2());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn test_example_distance() {
        let lists = Lists::from_str(EXAMPLE).unwrap();
        assert_eq!(lists.get_total_distance(), 11);
    }

    #[test]
    fn test_example_similarity() {
        let lists = Lists::from_str(EXAMPLE).unwrap();
        assert_eq!(lists.get_similarity_score(), 31);
    }
}
