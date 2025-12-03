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

            if num > 0 {
                // Positive is straight-forward, add and correct for boundary
                self.position += num as usize;
                if self.position >= 100 {
                    self.position -= 100;
                    self.zero_count += 1;
                }
            } else if num < 0 {
                // Store new position as signed int
                let mut position = self.position as isize + num;
                // Correct if we're past 0
                if position < 0 {
                    position += 100;
                    // If we didn't start a 0, count it as a zero count
                    if self.position != 0 {
                        self.zero_count += 1;
                    }
                } else if position == 0 {
                    // If we landed on zero, count it
                    self.zero_count += 1;
                }
                // Store new position
                self.position = position as usize;
            }
        }

        Ok(self.zero_count)
    }
}
