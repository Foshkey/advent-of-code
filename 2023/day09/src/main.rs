use std::{num::ParseIntError, str::FromStr};
use anyhow::{bail, Error, Result};

#[derive(Debug)]
struct Sequence {
    numbers: Vec<i32>,
}

impl FromStr for Sequence {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Sequence {
            numbers: s
                .split(' ')
                .map(|n| n.parse::<i32>())
                .collect::<Result<Vec<i32>, ParseIntError>>()?,
        })
    }
}

impl Sequence {
    fn get_next_number(&self) -> Result<i32> {
        if self.is_all_zero() {
            return Ok(0);
        }

        let next_sequence = self.generate_next_sequence()?;
        Ok(self.get_last_number()? + next_sequence.get_next_number()?)
    }

    fn get_prev_number(&self) -> Result<i32> {
        if self.is_all_zero() {
            return Ok(0);
        }

        let next_sequence = self.generate_next_sequence()?;
        Ok(self.get_first_number()? - next_sequence.get_prev_number()?)
    }

    fn get_first_number(&self) -> Result<i32> {
        let Some(first) = self.numbers.first() else { bail!("Empty sequence.") };
        Ok(*first)
    }

    fn get_last_number(&self) -> Result<i32> {
        let Some(last) = self.numbers.last() else { bail!("Empty sequence.") };
        Ok(*last)
    }

    fn is_all_zero(&self) -> bool {
        self.numbers.iter().all(|&n| n == 0)
    }

    fn generate_next_sequence(&self) -> Result<Sequence> {
        let mut nums = self.numbers.iter();
        let mut next_seq = Vec::<i32>::new();
        let Some(mut prev) = nums.next() else { bail!("Empty sequence."); };
        let mut valid_sequence = false;

        for num in nums {
            next_seq.push(num - prev);
            prev = num;
            valid_sequence = true;
        }

        if !valid_sequence {
            bail!("Generated empty sequence.");
        }

        Ok(Sequence { numbers: next_seq })
    }
}

fn solution(input: &str) -> Result<i32> {
    let sequences: Vec<Sequence> = input
        .lines()
        .map(|line| line.parse::<Sequence>())
        .collect::<Result<Vec<Sequence>>>()?;
    
    sequences.iter().map(|sequence| sequence.get_next_number()).sum()
}

fn solution_backwards(input: &str) -> Result<i32> {
    let sequences: Vec<Sequence> = input
        .lines()
        .map(|line| line.parse::<Sequence>())
        .collect::<Result<Vec<Sequence>>>()?;
    
    sequences.iter().map(|sequence| sequence.get_prev_number()).sum()
}

fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("Part 1: {result:?}");
    let result_pt2 = solution_backwards(input);
    println!("Part 2: {result_pt2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        let result = solution(input).unwrap();
        assert_eq!(114, result);
    }

    #[test]
    fn test_example_backwards() {
        let input = include_str!("example.txt");
        let result = solution_backwards(input).unwrap();
        assert_eq!(2, result);
    }
}
