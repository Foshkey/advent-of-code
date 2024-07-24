use std::{collections::HashMap, str::FromStr};

use anyhow::{bail, Error, Result};

use super::{part::Part, step::Step};

pub type Workflow = Vec<Step>;

#[derive(Debug, PartialEq)]
pub struct Sequence {
    pub workflows: HashMap<String, Workflow>,
    pub parts: Vec<Part>,
}

impl FromStr for Sequence {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut workflows = HashMap::new();
        let mut parts = Vec::new();

        let mut lines = s.lines();
        for line in lines.by_ref() {
            if line.is_empty() {
                // continue with parts
                break;
            }

            let workflow_split: Vec<&str> = line.split('{').collect();
            if workflow_split.len() != 2 {
                bail!("Invalid input format: {}", line);
            }

            let name = workflow_split[0].trim();
            let steps: Workflow = workflow_split[1]
                .trim_matches(|c| c == '}')
                .split(',')
                .map(|s| s.parse())
                .collect::<Result<Vec<Step>>>()?;

            workflows.insert(name.to_string(), steps);
        }

        for line in lines.by_ref() {
            let part: Part = line.parse()?;
            parts.push(part);
        }

        Ok(Sequence { workflows, parts })
    }
}

#[cfg(test)]
mod tests {
    use super::super::step::StepResult;
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let sequence: Sequence = input.parse().unwrap();

        assert_eq!(11, sequence.workflows.len());
        assert_eq!(
            &vec![
                Step::Compare(
                    's',
                    Ordering::Greater,
                    2770,
                    StepResult::Continue("qs".to_string())
                ),
                Step::Compare(
                    'm',
                    Ordering::Less,
                    1801,
                    StepResult::Continue("hdj".to_string())
                ),
                Step::Final(StepResult::Reject)
            ],
            sequence.workflows.get("qqz").unwrap()
        );

        assert_eq!(5, sequence.parts.len());
        assert_eq!(
            Part {
                ratings: vec![('x', 2036), ('m', 264), ('a', 79), ('s', 2244),]
                    .into_iter()
                    .collect()
            },
            sequence.parts[2]
        );
    }
}
