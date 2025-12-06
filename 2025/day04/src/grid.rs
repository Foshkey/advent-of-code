use std::num::NonZeroUsize;

#[derive(Clone)]
pub struct Grid {
    grid: Vec<Vec<bool>>,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        Self {
            grid: value
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '@' => true,
                            '.' => false,
                            c => panic!("Unexpected character: {c}"),
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

impl Grid {
    pub fn get_accessible_rolls(&self) -> Vec<(usize, usize)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter().enumerate().filter_map(move |(c, &roll)| {
                    if self.is_accessible(roll, r, c) {
                        Some((r, c))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    pub fn get_num_removable(&self) -> usize {
        let mut grid = self.clone();
        let mut count = 0;
        while let Some(round) = NonZeroUsize::new(grid.remove_all_accessible()) {
            count += round.get();
        }
        count
    }

    fn remove_all_accessible(&mut self) -> usize {
        let accessible_rolls = self.get_accessible_rolls();
        let count = accessible_rolls.len();
        for (r, c) in accessible_rolls {
            self.grid[r][c] = false;
        }
        count
    }

    fn is_accessible(&self, roll: bool, r: usize, c: usize) -> bool {
        if !roll {
            return false;
        }

        let r = r as isize;
        let c = c as isize;
        let mut adj = 0;

        for ar in r - 1..=r + 1 {
            for ac in c - 1..=c + 1 {
                if ar == r && ac == c {
                    continue;
                }

                if ar >= 0
                    && ac >= 0
                    && self
                        .grid
                        .get(ar as usize)
                        .is_some_and(|row| row.get(ac as usize).is_some_and(|aroll| *aroll))
                {
                    adj += 1;
                }
            }
        }

        adj < 4
    }
}
