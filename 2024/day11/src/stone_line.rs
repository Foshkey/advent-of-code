use std::collections::HashMap;

#[derive(Debug)]
pub struct StoneLine {
    stones: Vec<Stone>,
}

impl StoneLine {
    pub(crate) fn blink(&mut self) {
        let mut new_stones: Vec<(usize, Stone)> = Vec::new();

        for (i, stone) in self.stones.iter_mut().enumerate() {
            if stone.number == 0 {
                stone.number = 1;
                continue;
            }

            let stone_str = stone.number.to_string();
            let stone_len = stone_str.len();
            if stone_len % 2 == 0 {
                let (left, right) = stone_str.split_at(stone_len / 2);
                stone.number = right.parse().unwrap();
                new_stones.push((
                    i,
                    Stone {
                        number: left.parse().unwrap(),
                        count: stone.count,
                    },
                ));
                continue;
            }

            stone.number *= 2024
        }

        while let Some((i, new_stone)) = new_stones.pop() {
            self.stones.insert(i, new_stone);
        }

        self.group()
    }

    pub(crate) fn len(&self) -> usize {
        self.stones.iter().map(|stone| stone.count).sum()
    }

    fn group(&mut self) {
        let mut stone_map: HashMap<usize, &mut Stone> = HashMap::new();
        let mut removals: Vec<usize> = Vec::new();

        for (i, stone) in self.stones.iter_mut().enumerate() {
            if let Some(base_stone) = stone_map.get_mut(&stone.number) {
                base_stone.count += stone.count;
                removals.push(i)
            } else {
                stone_map.insert(stone.number, stone);
            }
        }

        while let Some(index) = removals.pop() {
            self.stones.remove(index);
        }
    }
}

impl From<&str> for StoneLine {
    fn from(s: &str) -> Self {
        StoneLine {
            stones: s
                .split(' ')
                .map(|n| Stone {
                    number: n.parse().unwrap(),
                    count: 1,
                })
                .collect(),
        }
    }
}

#[derive(Debug)]
struct Stone {
    number: usize,
    count: usize,
}
