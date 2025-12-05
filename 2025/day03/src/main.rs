pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part_1(input)?);
    println!("Part 2: {}", part_2(input)?);
    Ok(())
}

fn part_1(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|bank| get_joltage(bank, 2))
        .collect::<Result<Vec<usize>>>()?
        .iter()
        .sum())
}

fn part_2(input: &str) -> Result<usize> {
    Ok(input
        .lines()
        .map(|bank| get_joltage(bank, 12))
        .collect::<Result<Vec<usize>>>()?
        .iter()
        .sum())
}

fn get_joltage(bank: &str, limit: usize) -> Result<usize> {
    let mut digits = vec![0u8; limit];
    // Loop through each character
    for (char_i, char) in bank.char_indices() {
        // Loop through each digit
        for digit_i in 0..limit {
            // If it's too close to the end, skip
            if char_i > bank.len() - (limit - digit_i) {
                continue;
            }

            // Parse
            let n = char.to_string().parse()?;

            // Check if it's bigger
            if n > digits[digit_i] {
                digits[digit_i] = n;
                // Wipe out the remaining digits
                digits[(digit_i + 1)..limit].fill(0);
                break;
            }
        }
    }
    Ok(digits.iter().fold(0, |acc, &d| acc * 10 + d as usize))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(EXAMPLE).unwrap(), 357);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(EXAMPLE).unwrap(), 3121910778619);
    }
}
