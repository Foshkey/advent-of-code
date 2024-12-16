use std::collections::HashMap;

#[derive(Debug)]
pub struct StoneLine {
    // Key: number on the stone
    // Value: how many stones there are
    stones: HashMap<u128, u128>,
}

impl StoneLine {
    pub(crate) fn blink(&mut self) {
        let mut new_stones = Vec::new();

        for (number, count) in self.stones.iter_mut() {
            if *number == 0 {
                new_stones.push((1, *count));
                *count = 0;
                continue;
            }

            let stone_str = number.to_string();
            let stone_len = stone_str.len();
            if stone_len % 2 == 0 {
                let (left, right) = stone_str.split_at(stone_len / 2);
                new_stones.push((left.parse().unwrap(), *count));
                new_stones.push((right.parse().unwrap(), *count));
                *count = 0;
                continue;
            }

            new_stones.push((*number * 2024, *count));
            *count = 0;
        }

        for (number, count) in new_stones {
            if let Some(stored) = self.stones.get_mut(&number) {
                *stored += count;
            } else {
                self.stones.insert(number, count);
            }
        }
    }

    pub(crate) fn len(&self) -> u128 {
        self.stones.values().sum()
    }
}

impl From<&str> for StoneLine {
    fn from(s: &str) -> Self {
        StoneLine {
            stones: s.split(' ').map(|n| (n.parse().unwrap(), 1)).collect(),
        }
    }
}
