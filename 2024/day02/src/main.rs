use std::cmp::Ordering;

const INPUT: &str = include_str!("input.txt");

fn is_safe(sequence: &[usize]) -> bool {
    let mut numbers = sequence.iter();
    let Some(mut previous) = numbers.next() else {
        return true;
    };
    let mut trend = Ordering::Equal;

    for num in numbers {
        // Make sure the trend is not different
        let comparison = previous.cmp(num);
        if trend != Ordering::Equal && comparison != trend {
            return false;
        }

        // Make sure it's not too big of a jump
        let difference = (*previous as isize - *num as isize).unsigned_abs();
        if !(1..=3).contains(&difference) {
            return false;
        }

        // Set for next round
        trend = comparison;
        previous = num;
    }

    true
}

struct Report {
    sequence: Vec<usize>,
}

impl Report {
    fn is_safe(&self) -> bool {
        is_safe(&self.sequence)
    }

    fn is_safe_dampened(&self) -> bool {
        if is_safe(&self.sequence) {
            return true;
        }

        for index in 0..self.sequence.len() {
            let mut sequence = self.sequence.clone();
            sequence.remove(index);
            if is_safe(&sequence) {
                return true;
            }
        }

        false
    }
}

struct ReportCollection {
    reports: Vec<Report>,
}

impl ReportCollection {
    fn get_num_safe(&self) -> usize {
        self.reports
            .iter()
            .filter(|report| report.is_safe())
            .count()
    }

    fn get_num_safe_dampened(&self) -> usize {
        self.reports
            .iter()
            .filter(|report| report.is_safe_dampened())
            .count()
    }
}

impl From<&str> for ReportCollection {
    fn from(s: &str) -> Self {
        let reports: Vec<Report> = s
            .lines()
            .map(|line| Report {
                sequence: line
                    .split_whitespace()
                    .map(|num_str| num_str.parse::<usize>().unwrap())
                    .collect(),
            })
            .collect();

        ReportCollection { reports }
    }
}

fn part_1(input: &str) -> usize {
    ReportCollection::from(input).get_num_safe()
}

fn part_2(input: &str) -> usize {
    ReportCollection::from(input).get_num_safe_dampened()
}

fn main() {
    println!("Part 1: {:?}", part_1(INPUT));
    println!("Part 2: {:?}", part_2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn test_example_part_1() {
        let result = part_1(EXAMPLE);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_example_part_2() {
        let result = part_2(EXAMPLE);
        assert_eq!(result, 4);
    }
}
