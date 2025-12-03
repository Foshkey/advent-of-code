use std::str::FromStr;

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
    pub fn get_sum_invalid_ids(&self) -> Result<usize> {
        Ok(self
            .ids
            .iter()
            .map(|&range| get_sum_invalid(range))
            .collect::<Result<Vec<usize>>>()?
            .iter()
            .sum())
    }
}

fn get_sum_invalid((min, max): (usize, usize)) -> Result<usize> {
    let start = get_first_half_digits_floor(min)?;
    let end = get_first_half_digits_ceil(max)?;

    Ok((start..=end)
        .filter_map(generate_invalid)
        .filter(|&n| (min..=max).contains(&n))
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

fn generate_invalid(num: usize) -> Option<usize> {
    if num == 0 {
        None
    } else {
        format!("{num}{num}").parse().ok()
    }
}
