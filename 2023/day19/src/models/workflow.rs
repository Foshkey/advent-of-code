use std::str::FromStr;

use anyhow::{Error, Result};

use super::step::Step;

#[derive(Debug, Clone, PartialEq)]
pub struct Workflow(Vec<Step>);

impl Workflow {
    pub fn new(steps: Vec<Step>) -> Self {
        Workflow(steps)
    }
}

impl FromStr for Workflow {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Workflow(
            s.trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|s| s.parse())
                .collect::<Result<Vec<Step>>>()?,
        ))
    }
}

impl IntoIterator for Workflow {
    type Item = Step;
    type IntoIter = std::vec::IntoIter<Step>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Workflow {
    type Item = &'a Step;
    type IntoIter = std::slice::Iter<'a, Step>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::models::step::StepResult;

    use super::*;

    #[test]
    fn test_parse() {
        let input = "{x<50:abc,m>300:A,R}";
        let expected = Workflow(vec![
            Step::Compare(
                'x',
                Ordering::Less,
                50,
                StepResult::Continue("abc".to_string()),
            ),
            Step::Compare('m', Ordering::Greater, 300, StepResult::Accept),
            Step::Final(StepResult::Reject),
        ]);
        let workflow: Workflow = input.parse().unwrap();
        assert_eq!(expected, workflow)
    }
}
