use std::{collections::HashMap, str::FromStr};

use anyhow::{anyhow, bail, Error, Result};

type Cache = HashMap<Row, usize>;

/// Represents the contiguous group portion of each row (E.g. 1,1,3)
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct ContiguousGroups(Vec<usize>);

impl FromStr for ContiguousGroups {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let groups = s
            .split(',')
            .map(|n| n.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;
        Ok(ContiguousGroups(groups))
    }
}

impl ContiguousGroups {
    /// Returns the sum of the contiguous group, i.e. total damaged in the row.
    fn sum(&self) -> usize {
        self.0.iter().sum()
    }

    /// Removes the first element and returns that. If empty, returns 0.
    fn pop_first(&mut self) -> usize {
        if self.0.is_empty() {
            return 0;
        }

        self.0.remove(0)
    }

    /// Determines the largest possible gap given the pattern length.
    fn get_largest_gap(&self, pattern_len: usize) -> usize {
        let total_damaged = self.sum();
        if pattern_len <= total_damaged {
            return 0;
        }

        let total_groups = self.0.len();
        let taken_up_by_other_gaps = if total_groups > 2 {
            total_groups - 2
        } else {
            0
        };

        pattern_len - (total_damaged - taken_up_by_other_gaps)
    }
}

/// Represents the pattern portion of the row (E.g. ???.###)
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Pattern(String);

impl FromStr for Pattern {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        for c in s.chars() {
            if c != '.' && c != '#' && c != '?' {
                bail!("{s} is not a valid pattern.");
            }
        }
        Ok(Pattern(s.to_string()))
    }
}

impl Pattern {
    /// Builds one section of the pattern given number of operational (.) or damaged (#),
    /// E.g. (2, 3) would return ..###.
    fn build_section(operational: usize, damaged: usize, end: bool) -> Self {
        Self(".".repeat(operational) + &"#".repeat(damaged) + if end { "" } else { "." })
    }

    /// Determines whether the pattern matches the other pattern passed in. ? are ignored.
    /// I.e. #.#.### will match ???.### and vice versa.
    fn matches(&self, other: &Pattern) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }

        let mut other_chars = other.0.chars();
        for c in self.0.chars() {
            let Some(other_c) = other_chars.next() else {
                return false;
            };

            if c == '?' || other_c == '?' {
                continue;
            }

            if c != other_c {
                return false;
            }
        }

        true
    }

    /// Splits the pattern at the given index.
    fn split_at(&self, index: usize) -> (Self, Self) {
        if index >= self.len() {
            return (self.clone(), Pattern("".to_string()));
        }

        let (first, second) = self.0.split_at(index);
        (Pattern(first.to_string()), Pattern(second.to_string()))
    }

    /// Returns the length of the pattern.
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Row {
    pattern: Pattern,
    contiguous_groups: ContiguousGroups,
}

impl FromStr for Row {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (pattern, cont_groups_str) =
            s.split_once(' ').ok_or(anyhow!("No space in line {s}"))?;
        let contiguous_groups = cont_groups_str.parse::<ContiguousGroups>()?;
        Ok(Row {
            pattern: pattern.parse::<Pattern>()?,
            contiguous_groups,
        })
    }
}

impl Row {
    /// Counts the possible arrangements with this row, given a cache to be used across calls.
    fn count_arrangements_with_cache(&self, cache: &mut Cache) -> usize {
        // First check the cache to see if there's an entry.
        if let Some(arrangements) = cache.get(self) {
            return *arrangements;
        }

        let Row {
            pattern,
            contiguous_groups,
        } = self;

        // Determine if we're at the end state by checking if the groups are empty
        if contiguous_groups.0.is_empty() {
            // If there are still damaged springs left over, then not a valid arrangement
            if pattern.0.contains('#') {
                return 0;
            // Otherwise it is
            } else {
                return 1;
            }
        }

        // Figure out the largest gap we have to check
        let largest_gap = contiguous_groups.get_largest_gap(pattern.len());

        // Then iterate from smallest to largest gap for the immediately next gap
        let mut valid_patterns = 0;
        for gap_len in 0..=largest_gap {
            // Split the contiguous groups
            let mut remaining_groups = contiguous_groups.clone();
            let first_num = remaining_groups.pop_first();

            // Build a potential match for the first pattern
            let potential_match =
                Pattern::build_section(gap_len, first_num, remaining_groups.0.is_empty());

            // Split the pattern
            let (sub_pattern, next_pattern) = pattern.split_at(potential_match.len());

            // If it matches then continue to evaluate the remaining contiguous groups
            if sub_pattern.matches(&potential_match) {
                let sub_row = Row {
                    pattern: next_pattern,
                    contiguous_groups: remaining_groups,
                };

                // Add it to the count (1 is valid, 0 is not valid)
                valid_patterns += sub_row.count_arrangements_with_cache(cache);
            }
        }

        // Add it to the cache and return the value
        cache.insert(self.clone(), valid_patterns);
        valid_patterns
    }

    /// Expands this row as described in the problem. Copies contiguous groups and patterns separated by ?
    fn expand(&mut self, factor: usize) {
        self.contiguous_groups = ContiguousGroups(self.contiguous_groups.0.repeat(factor));
        self.pattern = Pattern(
            (0..factor)
                .map(|_| self.pattern.0.clone())
                .collect::<Vec<String>>()
                .join("?"),
        );
    }
}

fn part_1(input: &str) -> Result<usize> {
    let mut cache = Cache::new();
    input.lines().try_fold(0, |acc, l| {
        let num = l.parse::<Row>()?.count_arrangements_with_cache(&mut cache);
        Ok(acc + num)
    })
}

fn part_2(input: &str) -> Result<usize> {
    let mut cache = Cache::new();
    input.lines().try_fold(0, |acc, l| {
        let mut row = l.parse::<Row>()?;
        row.expand(5);
        let num = row.count_arrangements_with_cache(&mut cache);
        Ok(acc + num)
    })
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
        let input = include_str!("example.txt");
        let result = part_1(input);
        assert_eq!(21, result.unwrap())
    }

    #[test]
    fn test_example_part2() {
        let input = include_str!("example.txt");
        let result = part_2(input);
        assert_eq!(525152, result.unwrap())
    }

    #[test]
    fn test_count_arrangements() {
        let mut cache = Cache::new();
        assert_eq!(
            1,
            "??#?#????#.... 3,2"
                .parse::<Row>()
                .unwrap()
                .count_arrangements_with_cache(&mut cache)
        );
        assert_eq!(
            4,
            "???#????????#. 7,1"
                .parse::<Row>()
                .unwrap()
                .count_arrangements_with_cache(&mut cache)
        );
        assert_eq!(
            2,
            "..????#?.???.?#?. 4,3"
                .parse::<Row>()
                .unwrap()
                .count_arrangements_with_cache(&mut cache)
        );
        assert_eq!(
            3,
            ".?.????#????# 2,4,1"
                .parse::<Row>()
                .unwrap()
                .count_arrangements_with_cache(&mut cache)
        );
        assert_eq!(
            6,
            "?#?#.#????.?#??# 4,1,1,2,1"
                .parse::<Row>()
                .unwrap()
                .count_arrangements_with_cache(&mut cache)
        );
        assert_eq!(
            4,
            "??#??#?.?#?? 1,2,2"
                .parse::<Row>()
                .unwrap()
                .count_arrangements_with_cache(&mut cache)
        );
        assert_eq!(
            4,
            "?????#.#?. 1,1,1"
                .parse::<Row>()
                .unwrap()
                .count_arrangements_with_cache(&mut cache)
        );
        assert_eq!(
            10,
            ".#?????????# 1,1,1,2"
                .parse::<Row>()
                .unwrap()
                .count_arrangements_with_cache(&mut cache)
        );
        assert_eq!(
            1,
            "??.#??#??? 2,3"
                .parse::<Row>()
                .unwrap()
                .count_arrangements_with_cache(&mut cache)
        );
    }

    #[test]
    fn test_build_pattern() {
        assert_eq!(".##.", Pattern::build_section(1, 2, false).0);
        assert_eq!("#.", Pattern::build_section(0, 1, false).0);
        assert_eq!(".", Pattern::build_section(0, 0, false).0);
        assert_eq!("#", Pattern::build_section(0, 1, true).0);
        assert_eq!("..##", Pattern::build_section(2, 2, true).0);
    }

    #[test]
    fn test_split_at() {
        assert_eq!(
            (Pattern("#.#.".to_string()), Pattern(".#".to_string())),
            Pattern("#.#..#".to_string()).split_at(4)
        );
        assert_eq!(
            (Pattern("#.#..#".to_string()), Pattern("".to_string())),
            Pattern("#.#..#".to_string()).split_at(20)
        );
    }
}
