use std::str::FromStr;

pub struct Schematic {
    keys: Vec<[u8; 5]>,
    locks: Vec<[u8; 5]>,
}

impl Schematic {
    pub fn get_unique_fits(&self) -> usize {
        let mut num = 0;
        for lock in &self.locks {
            for key in &self.keys {
                if fits(key, lock) {
                    num += 1;
                }
            }
        }
        num
    }
}

impl FromStr for Schematic {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut keys = Vec::new();
        let mut locks = Vec::new();
        let mut current = [0; 5];
        let mut count = 0;

        for line in s.lines() {
            match count {
                0 => (),
                6 => {
                    if line.chars().all(|c| c == '#') {
                        keys.push(current);
                    } else {
                        locks.push(current);
                    }
                    current = [0; 5];
                }
                7 => {
                    count = -1;
                }
                _ => {
                    for (i, c) in line.char_indices() {
                        if c == '#' {
                            current[i] += 1;
                        }
                    }
                }
            }
            count += 1;
        }

        Ok(Schematic { keys, locks })
    }
}

fn fits(a: &[u8; 5], b: &[u8; 5]) -> bool {
    a.iter().zip(b.iter()).all(|(x, y)| x + y <= 5)
}
