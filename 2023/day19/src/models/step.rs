use std::{cmp::Ordering, str::FromStr};

use anyhow::{anyhow, bail, Error, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Step {
    Compare(char, Ordering, u32, StepResult),
    Final(StepResult),
}

impl FromStr for Step {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() < 2 {
            return Ok(Step::Final(s.parse()?));
        }

        let condition = parts[0];
        let result = parts[1];

        let mut chars = condition.chars();
        let variable = chars.next().ok_or(anyhow!("Invalid input format: {}", s))?;
        let operator = match chars.next() {
            Some('<') => Ordering::Less,
            Some('>') => Ordering::Greater,
            _ => bail!("Invalid input format: {}", s),
        };
        let value: u32 = chars.collect::<String>().parse()?;

        let step_result: StepResult = result.parse()?;

        Ok(Step::Compare(variable, operator, value, step_result))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StepResult {
    Accept,
    Reject,
    Continue(String),
}

impl FromStr for StepResult {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "A" => Ok(StepResult::Accept),
            "R" => Ok(StepResult::Reject),
            _ => Ok(StepResult::Continue(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_from_str_valid_input() {
        let input = "a<2006:qkq";
        let step: Step = input.parse().unwrap();
        let expected = Step::Compare(
            'a',
            Ordering::Less,
            2006,
            StepResult::Continue("qkq".to_string()),
        );
        assert_eq!(expected, step);

        let input = "a>1716:R";
        let step: Step = input.parse().unwrap();
        let expected = Step::Compare('a', Ordering::Greater, 1716, StepResult::Reject);
        assert_eq!(expected, step);

        let input = "A";
        let step: Step = input.parse().unwrap();
        let expected = Step::Final(StepResult::Accept);
        assert_eq!(expected, step);

        let input = "crn";
        let step: Step = input.parse().unwrap();
        let expected = Step::Final(StepResult::Continue("crn".to_string()));
        assert_eq!(expected, step);
    }
}
