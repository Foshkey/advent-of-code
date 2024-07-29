use anyhow::bail;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct Part {
    pub ratings: HashMap<char, u16>,
}

impl Part {
    pub fn total_rating(&self) -> u32 {
        self.ratings.values().map(|&n| n as u32).sum()
    }
}

impl FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ratings = HashMap::new();

        let fields: Vec<&str> = s
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .collect();

        for field in fields {
            if field.is_empty() {
                continue;
            }

            let key_value: Vec<&str> = field.split('=').collect();
            if key_value.len() != 2 {
                bail!("Invalid input format: {}", field);
            }

            let key = key_value[0].trim().chars().next().unwrap();
            let value = key_value[1].trim().parse::<u16>()?;

            ratings.insert(key, value);
        }

        Ok(Part { ratings })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_valid_input() {
        let input = "{a=1, b=2, c=3}";
        let part: Part = input.parse().unwrap();

        assert_eq!(part.ratings.get(&'a'), Some(&1));
        assert_eq!(part.ratings.get(&'b'), Some(&2));
        assert_eq!(part.ratings.get(&'c'), Some(&3));
    }

    #[test]
    fn test_from_str_invalid_input() {
        let input = "{a=1, b=2, c}";
        let result: Result<Part, _> = input.parse();

        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_empty_input() {
        let input = "{}";
        let part: Part = input.parse().unwrap();

        assert!(part.ratings.is_empty());
    }

    #[test]
    fn test_from_str_duplicate_keys() {
        let input = "{a=1, b=2, a=3}";
        let part: Part = input.parse().unwrap();

        assert_eq!(part.ratings.get(&'a'), Some(&3));
        assert_eq!(part.ratings.get(&'b'), Some(&2));
    }
}
