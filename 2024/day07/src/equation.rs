use std::{slice::Iter, str::FromStr};

pub enum EquationError {
    NoColon,
    IntParseError,
}

pub struct Equation {
    pub result: u128,
    numbers: Vec<u128>,
}

impl Equation {
    pub fn is_possible(&self) -> bool {
        self.check_possible(0, self.numbers.iter())
    }

    fn check_possible(&self, sum: u128, mut numbers: Iter<'_, u128>) -> bool {
        let Some(next) = numbers.next() else {
            return self.result == sum;
        };

        if self.check_possible(sum + next, numbers.clone()) {
            return true;
        }

        self.check_possible(sum * next, numbers)
    }
}

impl FromStr for Equation {
    type Err = EquationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((result, numbers)) = s.split_once(":") else {
            return Err(EquationError::NoColon);
        };

        fn parse_u128(s: &str) -> Result<u128, EquationError> {
            s.parse().map_err(|_| EquationError::IntParseError)
        }

        let result = parse_u128(result)?;
        let numbers = numbers
            .trim()
            .split(' ')
            .map(parse_u128)
            .collect::<Result<Vec<u128>, EquationError>>()?;

        Ok(Equation { result, numbers })
    }
}
