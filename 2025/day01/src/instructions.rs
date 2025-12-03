use std::{collections::VecDeque, str::FromStr};

use crate::{Error, Result};

pub(crate) struct Instructions {
    position: usize,
    instruction_list: VecDeque<isize>,
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
                    let num: isize = num.parse()?;
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
    pub fn get_zero_landing_count(&mut self) -> Result<usize> {
        while let Some(num) = self.instruction_list.pop_front() {
            let mut position = (self.position as isize + num) % 100;
            if position < 0 {
                position += 100;
            }

            self.position = position as usize;

            if position == 0 {
                self.zero_count += 1;
            }
        }

        Ok(self.zero_count)
    }

    pub fn get_zero_passing_count(&mut self) -> Result<usize> {
        while let Some(mut num) = self.instruction_list.pop_front() {
            // Count full rotations
            self.zero_count += num.unsigned_abs() / 100;
            num %= 100;

            let mut new_position = self.position as isize + num;

            if new_position >= 100 {
                // If over 100, we passed zero.
                new_position -= 100;
                self.zero_count += 1;
            } else if new_position < 0 {
                // If we're in the negative, add 100
                new_position += 100;
                // And if we didn't start at 0, then we passed it
                if self.position != 0 {
                    self.zero_count += 1;
                }
            } else if new_position == 0 && self.position != 0 {
                // Finally, if we landed on zero, count it
                self.zero_count += 1;
            }

            self.position = new_position as usize;
        }

        Ok(self.zero_count)
    }
}
