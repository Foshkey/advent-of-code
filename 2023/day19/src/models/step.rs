use std::{cmp::Ordering, str::FromStr};

use anyhow::{anyhow, bail, Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    Compare(char, Ordering, u16, StepResult),
    Final(StepResult),
}

impl Step {
    pub fn clear_result(&self) -> Self {
        match self {
            Self::Compare(c, o, n, _) => {
                Self::Compare(*c, *o, *n, StepResult::Continue("".to_string()))
            }
            Self::Final(_) => Self::Final(StepResult::Continue("".to_string())),
        }
    }

    pub fn inverse(&self) -> Self {
        match self {
            Self::Compare(c, order, n, result) => match order {
                Ordering::Greater => Self::Compare(*c, Ordering::Less, *n + 1, result.clone()),
                Ordering::Less => Self::Compare(*c, Ordering::Greater, *n - 1, result.clone()),
                _ => Self::Compare(*c, *order, *n, result.clone()),
            },
            step => step.clone(),
        }
    }

    pub fn has_continue(&self) -> bool {
        matches!(self, Self::Compare(_, _, _, StepResult::Continue(_)))
    }
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
        let value: u16 = chars.collect::<String>().parse()?;

        let step_result: StepResult = result.parse()?;

        Ok(Step::Compare(variable, operator, value, step_result))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

        let input = "x<1716:";
        let step: Step = input.parse().unwrap();
        let expected = Step::Compare(
            'x',
            Ordering::Less,
            1716,
            StepResult::Continue("".to_string()),
        );
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

    #[test]
    fn test_step_inverse() {
        let step = Step::Compare(
            'a',
            Ordering::Less,
            2006,
            StepResult::Continue("qkq".to_string()),
        );
        let expected = Step::Compare(
            'a',
            Ordering::Greater,
            2005,
            StepResult::Continue("qkq".to_string()),
        );
        assert_eq!(expected, step.inverse());

        let step = Step::Compare('a', Ordering::Greater, 1716, StepResult::Reject);
        let expected = Step::Compare('a', Ordering::Less, 1717, StepResult::Reject);
        assert_eq!(expected, step.inverse());

        let step = Step::Final(StepResult::Accept);
        assert_eq!(step, step.inverse());

        let step = Step::Final(StepResult::Continue("crn".to_string()));
        assert_eq!(step, step.inverse());
    }
}
