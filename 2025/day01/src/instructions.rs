use std::{collections::VecDeque, str::FromStr};

use crate::{Error, Result};

pub(crate) struct Instructions {
    position: u8,
    instruction_list: VecDeque<i16>,
    zero_count: usize,
}

impl FromStr for Instructions {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self {
            position: 50,
            instruction_list: s
                .lines()
                .map(|line| {
                    let (direction, num) = line.split_at(1);
                    let num: i16 = num.parse()?;
                    match direction {
                        "L" => Ok(-num),
                        "R" => Ok(num),
                        _ => Err(format!("Invalid direction: {}", direction).into()),
                    }
                })
                .collect::<Result<_>>()?,
            zero_count: 0,
        })
    }
}

impl Instructions {
    pub fn get_password(&mut self) -> Result<usize> {
        while let Some(num) = self.instruction_list.pop_front() {
            let mut position = (self.position as i16 + num) % 100;
            if position < 0 {
                position += 100;
            }

            self.position = position as u8;

            if position == 0 {
                self.zero_count += 1;
            }
        }

        Ok(self.zero_count)
    }
}
