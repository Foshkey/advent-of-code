use std::{collections::HashSet, str::FromStr};

use crate::{Error, Result};

pub struct ProductList {
    ids: Vec<(usize, usize)>,
}

impl FromStr for ProductList {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            ids: s
                .split(',')
                .map(|r| {
                    r.split_once('-')
                        .ok_or(Error::from(format!("Invalid range: {r}")))
                        .and_then(|(l, r)| -> Result<(usize, usize)> {
                            Ok((l.parse()?, r.parse()?))
                        })
                })
                .collect::<Result<Vec<(usize, usize)>>>()?,
        })
    }
}

impl ProductList {
    pub fn get_sum_doubles(&self) -> Result<usize> {
        Ok(self
            .ids
            .iter()
            .map(|&range| get_sum_doubles(range))
            .collect::<Result<Vec<usize>>>()?
            .iter()
            .sum())
    }

    pub fn get_sum_multiples(&self) -> Result<usize> {
        Ok(self
            .ids
            .iter()
            .map(|&range| get_sum_multiples(range))
            .collect::<Result<Vec<usize>>>()?
            .iter()
            .sum())
    }
}

fn get_sum_doubles((min, max): (usize, usize)) -> Result<usize> {
    let start = get_first_half_digits_floor(min)?;
    let end = get_first_half_digits_ceil(max)?;

    Ok((start..=end)
        .filter_map(|n| generate_invalid(n, 2))
        .filter(|n| (min..=max).contains(n))
        .sum())
}

fn get_first_half_digits_floor(num: usize) -> Result<usize> {
    let s = num.to_string();
    let half_len = s.len() / 2;
    Ok(s[..half_len].parse().unwrap_or_default())
}

fn get_first_half_digits_ceil(num: usize) -> Result<usize> {
    let s = num.to_string();
    let half_len = s.len().div_ceil(2);
    Ok(s[..half_len].parse().unwrap_or_default())
}

fn get_sum_multiples((min, max): (usize, usize)) -> Result<usize> {
    let min_len = min.to_string().len();
    let max_len = max.to_string().len();
    let half_len = max_len.div_ceil(2);

    let mut invalids: HashSet<usize> = HashSet::new();

    for i in 1..=half_len {
        let digit = max_len - i;
        let divisor = 10usize.pow(digit as u32);
        let start = min / divisor;
        let end = max / divisor;
        for n in start..=end {
            let repeat_min = min_len / n.to_string().len();
            let repeat_max = max_len / n.to_string().len();
            for r in repeat_min..=repeat_max {
                if let Some(candidate) = generate_invalid(n, r)
                    && (min..=max).contains(&candidate)
                {
                    invalids.insert(candidate);
                }
            }
        }
    }

    Ok(invalids.iter().sum())
}

fn generate_invalid(num: usize, repeat: usize) -> Option<usize> {
    if num == 0 {
        None
    } else {
        (0..repeat)
            .fold(String::default(), |s, _| format!("{s}{num}"))
            .parse()
            .ok()
    }
}
