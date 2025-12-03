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
    instructions.get_password()
}

fn part_2(input: &str) -> Result<usize> {
    Ok(input.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let example = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(part_1(example).unwrap(), 3);
    }
}
