use std::{
    collections::{BTreeSet, HashMap},
    fmt::Display,
};

pub struct Memory {
    end: Coord,
    grid: Vec<Vec<bool>>,
    corruptions: Vec<Coord>,
    corrupted: usize,
}

impl Memory {
    pub fn new(size: usize, input: &str) -> Self {
        Memory {
            end: Coord {
                x: size - 1,
                y: size - 1,
            },
            grid: vec![vec![true; size]; size],
            corruptions: input.lines().map(|line| line.into()).collect(),
            corrupted: 0,
        }
    }

    pub fn corrupt(&mut self, size: usize) {
        for _ in 0..size {
            self.corrupt_next();
        }
    }

    pub fn corrupt_next(&mut self) {
        let Some(&position) = self.corruptions.get(self.corrupted) else {
            return;
        };

        let Some(tile) = self.get_mut(&position) else {
            return;
        };

        *tile = false;
        self.corrupted += 1;
    }

    pub fn get_last_corruption(&self) -> Option<Coord> {
        self.corruptions.get(self.corrupted - 1).copied()
    }

    pub fn find_path(&self) -> Option<usize> {
        // This is your typical A* search algorithm
        let start = Coord { x: 0, y: 0 };
        let mut set = BTreeSet::from([(0, start)]);
        let mut steps = HashMap::from([(start, 0)]);

        while let Some((_, current)) = set.pop_first() {
            if current == self.end {
                return Some(steps[&self.end]);
            }

            for neighbor in self.get_neighbors(&current) {
                let tentative_steps = steps[&current] + 1;
                let is_better_path = steps
                    .get(&neighbor)
                    .is_none_or(|&steps| tentative_steps < steps);

                if is_better_path {
                    steps.insert(neighbor, tentative_steps);
                    set.insert((tentative_steps + neighbor.distance(&self.end), neighbor));
                }
            }
        }

        None
    }

    fn get_neighbors(&self, position: &Coord) -> Vec<Coord> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(|(d_x, d_y)| self.get_relative(position, d_x, d_y))
            .collect()
    }

    fn get_relative(&self, position: &Coord, d_x: isize, d_y: isize) -> Option<Coord> {
        let x = position.x.checked_add_signed(d_x)?;
        let y = position.y.checked_add_signed(d_y)?;
        let &safe = self.grid.get(y)?.get(x)?;
        if safe {
            Some(Coord { x, y })
        } else {
            None
        }
    }

    fn get_mut(&mut self, position: &Coord) -> Option<&mut bool> {
        self.grid.get_mut(position.y)?.get_mut(position.x)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    pub fn distance(&self, other: &Coord) -> usize {
        (self.x as isize - other.x as isize).unsigned_abs()
            + (self.y as isize - other.y as isize).unsigned_abs()
    }
}

impl From<&str> for Coord {
    fn from(s: &str) -> Self {
        let (x, y) = s.split_once(',').unwrap();
        Coord {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_item_ordering() {
        let position1 = Coord { x: 1, y: 1 };
        let position2 = Coord { x: 1, y: 2 };
        let position3 = Coord { x: 0, y: 0 };

        let mut queue = BTreeSet::new();
        queue.insert((3, position1));
        queue.insert((1, position1));
        queue.insert((2, position2));
        queue.insert((0, position3));

        assert_eq!(position3, queue.pop_first().unwrap().1);
        assert_eq!(position1, queue.pop_first().unwrap().1);
        assert_eq!(position2, queue.pop_first().unwrap().1);
        assert_eq!(position1, queue.pop_first().unwrap().1);
    }
}
