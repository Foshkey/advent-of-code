use std::{
    cmp::{max, min, Ordering},
    collections::HashMap,
    ops::Range,
};

use anyhow::{anyhow, Result};

use crate::models::{
    sequence::Sequence,
    step::{Step, StepResult},
    workflow::Workflow,
};

const MAX_RATING: u16 = 4000;
const MAX_RANGE: Range<u16> = 1..(MAX_RATING + 1);

pub fn count_distinct_combinations(sequence: &Sequence) -> Result<u64> {
    let paths = get_accepted_paths(sequence)?;
    let ranges_map = get_ranges(&paths);

    let mut count = 1;
    for (_, ranges) in ranges_map {
        count *= ranges.iter().map(|r| r.len() as u64).sum::<u64>()
    }

    Ok(count)
}

fn get_ranges(paths: &Vec<Vec<Step>>) -> HashMap<char, Vec<Range<u16>>> {
    let mut ranges = HashMap::new();

    for path in paths {
        let Some(range_map) = get_range(path) else {
            continue;
        };

        for (c, range) in range_map {
            let Some(ranges_map) = ranges.get_mut(&c) else {
                ranges.insert(c, vec![range]);
                continue;
            };

            let mut found_range = false;
            for existing_range in ranges_map {
                if ranges_overlap(&range, existing_range) {
                    *existing_range = merge_ranges(&range, existing_range);
                    found_range = true;
                    break;
                }
            }

            if !found_range {
                ranges.get_mut(&c).unwrap().push(range);
            }
        }
    }

    ranges
}

fn get_range(path: &Vec<Step>) -> Option<HashMap<char, Range<u16>>> {
    let mut range_map: HashMap<char, Range<u16>> = vec![
        ('x', MAX_RANGE),
        ('m', MAX_RANGE),
        ('a', MAX_RANGE),
        ('s', MAX_RANGE),
    ]
    .into_iter()
    .collect();

    for step in path {
        let Step::Compare(c, order, n, _) = step else {
            continue;
        };

        let Some(range) = range_map.get_mut(c) else {
            continue;
        };

        if range.contains(n) {
            match order {
                Ordering::Greater => *range = (*n + 1)..range.end,
                Ordering::Less => *range = range.start..*n,
                _ => (),
            }
        } else {
            // Invalid path
            return None;
        }
    }

    Some(range_map)
}

fn ranges_overlap(range1: &Range<u16>, range2: &Range<u16>) -> bool {
    range1.start < range2.end && range2.start < range1.end
}

fn merge_ranges(range1: &Range<u16>, range2: &Range<u16>) -> Range<u16> {
    let start = min(range1.start, range2.start);
    let end = max(range1.end, range2.end);
    start..end
}

fn get_accepted_paths(sequence: &Sequence) -> Result<Vec<Vec<Step>>> {
    let starting_workflow = sequence
        .workflows
        .get("in")
        .ok_or(anyhow!("Could not find workflow: in"))?;
    let mut paths: Vec<Vec<Step>> = Vec::new();
    let mut current_path: Vec<Step> = Vec::new();

    trace_path(sequence, starting_workflow, &mut paths, &mut current_path)?;

    Ok(paths)
}

fn trace_path(
    sequence: &Sequence,
    workflow: &Workflow,
    paths: &mut Vec<Vec<Step>>,
    current_path: &mut Vec<Step>,
) -> Result<()> {
    for step in workflow {
        match step {
            Step::Final(step_result) => {
                if *step_result == StepResult::Accept {
                    paths.push(current_path.clone());
                }
                return Ok(());
            }

            Step::Compare(_, _, _, StepResult::Accept) => {
                // Add accepted path to paths
                let mut accepted_path = current_path.clone();
                accepted_path.push(step.clear_result());
                paths.push(accepted_path.clone());

                // Continue with inverse
                current_path.push(step.clear_result().inverse())
            }

            Step::Compare(_, _, _, StepResult::Reject) => {
                current_path.push(step.clear_result().inverse());
            }

            Step::Compare(_, _, _, StepResult::Continue(name)) => {
                // Fork a new path for the next workflow
                let mut new_path = current_path.clone();
                new_path.push(step.clear_result());
                let workflow = sequence
                    .workflows
                    .get(name)
                    .ok_or(anyhow!("Could not find workflow: {}", name))?;
                trace_path(sequence, workflow, paths, &mut new_path)?;

                // Continue on current workflow with inverse
                current_path.push(step.clear_result().inverse());
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_distinct() {
        let sequence: Sequence =
            "in{x<3:a,R}\na{a>3997:b,R}\nb{m<5:c,R}\nc{s>3995:d,R}\nd{s>3999:A,A}"
                .parse()
                .unwrap();
        let expected = 2 * 3 * 4 * 5;
        let result = count_distinct_combinations(&sequence).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_ranges() {
        let paths: Vec<Vec<Step>> = [
            vec!["x<4:", "m>3995:"],
            vec!["x<30:", "x>10:", "m>3990:"],
            vec!["x<35:", "x>10:", "m>3991:"],
        ]
        .iter()
        .map(|p| p.iter().map(|s| s.parse().unwrap()).collect())
        .collect();

        let result = get_ranges(&paths);

        assert_eq!(vec![1..4, 11..35], result[&'x']);
        assert_eq!(vec![3991..4001], result[&'m']);
    }

    #[test]
    fn test_get_accepted_paths() {
        let sequence: Sequence = "in{x<232:abc,m>400:A,R}\nabc{a>1200:R,A}".parse().unwrap();
        let expected: Vec<Vec<Step>> = [vec!["x<232:", "a<1201:"], vec!["x>231:", "m>400:"]]
            .iter()
            .map(|p| p.iter().map(|s| s.parse().unwrap()).collect())
            .collect();
        let result = get_accepted_paths(&sequence).unwrap();
        assert_eq!(expected, result);

        let sequence: Sequence = "in{a>2000:A,A}".parse().unwrap();
        let expected: Vec<Vec<Step>> = [vec!["a>2000:"], vec!["a<2001:"]]
            .iter()
            .map(|p| p.iter().map(|s| s.parse().unwrap()).collect())
            .collect();
        let result = get_accepted_paths(&sequence).unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn test_get_accepted_paths_example() {
        let sequence: Sequence = include_str!("../example.txt").parse().unwrap();
        let result = get_accepted_paths(&sequence);
        assert!(result.is_ok());
    }
}
