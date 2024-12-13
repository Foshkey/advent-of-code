use std::slice::Iter;

pub struct Equation {
    pub result: u128,
    numbers: Vec<u128>,
    with_concat: bool,
}

impl Equation {
    pub fn init(s: &str, with_concat: bool) -> Option<Self> {
        let (result, numbers) = s.split_once(":")?;

        let result = parse_u128(result)?;
        let numbers = numbers
            .trim()
            .split(' ')
            .map(parse_u128)
            .collect::<Option<Vec<u128>>>()?;

        Some(Equation {
            result,
            numbers,
            with_concat,
        })
    }

    pub fn is_possible(&self) -> bool {
        self.check_possible(0, self.numbers.iter())
    }

    fn check_possible(&self, sum: u128, mut numbers: Iter<'_, u128>) -> bool {
        let Some(next) = numbers.next() else {
            return self.result == sum;
        };

        if self.with_concat && self.check_possible(concat(&sum, next), numbers.clone()) {
            return true;
        }

        if self.check_possible(sum + next, numbers.clone()) {
            return true;
        }

        self.check_possible(sum * next, numbers)
    }
}

fn parse_u128(s: &str) -> Option<u128> {
    s.parse().ok()
}

fn concat(a: &u128, b: &u128) -> u128 {
    (a.to_string() + &b.to_string()).parse().unwrap()
}
