use std::{cmp::Ordering, collections::HashMap, ops::Range};

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

    let mut count = 0;
    for path in paths {
        let Some(ranges) = get_ranges(&path) else {
            continue;
        };

        count += ranges
            .iter()
            .fold(1u64, |acc, (_, range)| acc * range.len() as u64);
    }

    Ok(count)
}

fn get_ranges(path: &Vec<Step>) -> Option<HashMap<char, Range<u16>>> {
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

fn get_accepted_paths(sequence: &Sequence) -> Result<Vec<Vec<Step>>> {
    let starting_workflow = sequence
        .workflows
        .get("in")
        .ok_or(anyhow!("Could not find workflow: in"))?;
    let mut accepted_paths: Vec<Vec<Step>> = Vec::new();
    let mut current_path: Vec<Step> = Vec::new();

    trace_path(
        sequence,
        starting_workflow,
        &mut accepted_paths,
        &mut current_path,
    )?;

    Ok(accepted_paths)
}

fn trace_path(
    sequence: &Sequence,
    workflow: &Workflow,
    accepted_paths: &mut Vec<Vec<Step>>,
    current_path: &mut Vec<Step>,
) -> Result<()> {
    for step in workflow {
        match step {
            Step::Final(step_result) => {
                match step_result {
                    // Add to accepted paths if accept
                    StepResult::Accept => accepted_paths.push(current_path.clone()),

                    // Continue with new workflow
                    StepResult::Continue(name) => {
                        let next = sequence
                            .workflows
                            .get(name)
                            .ok_or(anyhow!("Could not find workflow: {}", name))?;
                        trace_path(sequence, next, accepted_paths, current_path)?;
                    }

                    // Otherwise (Reject), don't do anything
                    _ => (),
                }

                // If we're at the final step, return out
                return Ok(());
            }

            Step::Compare(_, _, _, StepResult::Accept) => {
                // Add accepted path to the list
                let mut accepted_path = current_path.clone();
                accepted_path.push(step.clear_result());
                accepted_paths.push(accepted_path);
            }

            Step::Compare(_, _, _, StepResult::Continue(name)) => {
                // Fork a new path for the next workflow
                let mut new_path = current_path.clone();
                new_path.push(step.clear_result());
                let next = sequence
                    .workflows
                    .get(name)
                    .ok_or(anyhow!("Could not find workflow: {}", name))?;
                trace_path(sequence, next, accepted_paths, &mut new_path)?;
            }

            _ => (),
        }

        // Continue on current workflow with the inverse
        // (E.g. x<251:R means continue on with x>250)
        current_path.push(step.clear_result().inverse());
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
    fn test_count_distinct_example() {
        let sequence: Sequence = include_str!("../example.txt").parse().unwrap();
        let result = count_distinct_combinations(&sequence);
        assert!(result.is_ok());
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
