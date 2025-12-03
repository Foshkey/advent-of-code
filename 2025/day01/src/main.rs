use crate::instructions::Instructions;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

mod instructions;

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part_1(input)?);
    println!("Part 2: {}", part_2(input)?);
    Ok(())
}

fn part_1(input: &str) -> Result<usize> {
    let mut instructions: Instructions = input.parse()?;
    instructions.get_zero_landing_count()
}

fn part_2(input: &str) -> Result<usize> {
    let mut instructions: Instructions = input.parse()?;
    instructions.get_zero_passing_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE).unwrap(), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE).unwrap(), 6);
    }
}
