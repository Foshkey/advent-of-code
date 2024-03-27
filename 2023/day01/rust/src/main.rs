use std::collections::{BTreeMap, HashMap};

fn parse(input: String) -> Result<u8, String> {
    match input.parse::<u8>() {
        Ok(d) => Ok(d),
        Err(_) => Err(format!("Failed to parse digits from string {}", input)),
    }
}

fn find_digits(
    input: &str,
    map: Option<&HashMap<&str, u8>>,
) -> Result<BTreeMap<usize, u8>, String> {
    // Digits will be keyed by their index in the input
    let mut digits = BTreeMap::new();

    // Add the non-spelled-out digits first
    for (i, c) in input.char_indices() {
        if c.is_digit(10) {
            let digit = parse(c.to_string())?;
            digits.insert(i, digit);
        }
    }

    // Then search each key in the map for the spelled out digits
    if let Some(m) = map {
        for (&key, &value) in m {
            for (index, _) in input.match_indices(key) {
                digits.insert(index, value);
            }
        }
    }

    Ok(digits)
}

fn get_calibration_value(line: &str, map: Option<&HashMap<&str, u8>>) -> Result<u8, String> {
    let digits = find_digits(line, map)?;
    let Some((_, &first)) = digits.first_key_value() else {
        return Err(format!("Could not find first digit in string {}.", line));
    };
    let Some((_, &last)) = digits.last_key_value() else {
        return Err(format!("Could not find last digit in string {}.", line));
    };
    Ok(first * 10 + last)
}

fn get_total_calibration_value_with_map(input: &str, map: Option<&HashMap<&str, u8>>) -> Result<u32, String> {
    let mut errors = vec![];
    let calibration_value = input
        .lines()
        .map(|line| get_calibration_value(line, map))
        .filter_map(|result| result.map_err(|e| errors.push(e)).ok())
        .map(|value| value as u32)
        .sum();

    if !errors.is_empty() {
        return Err(format!("Failed to get calibration value: {:?}", errors))
    }

    Ok(calibration_value)
}

fn get_total_calibration_value(input: &str) -> Result<u32, String> {
    get_total_calibration_value_with_map(input, None)
}

fn get_total_calibration_value_spelled_out(input: &str) -> Result<u32, String> {
    let digit_map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("zero", 0),
    ]);

    get_total_calibration_value_with_map(input, Some(&digit_map))
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {:?}", get_total_calibration_value(input));
    println!(
        "Part 2: {:?}",
        get_total_calibration_value_spelled_out(input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("example.txt");
        assert_eq!(Ok(142), get_total_calibration_value(input));
    }

    #[test]
    fn test_example_2() {
        let input = include_str!("example2.txt");
        assert_eq!(Ok(281), get_total_calibration_value_spelled_out(input));
    }
}
