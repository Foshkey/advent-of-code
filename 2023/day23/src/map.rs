use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap, HashSet},
};

pub struct Map {
    grid: Vec<Vec<char>>,
    start: Coord,
    end: Coord,
}

impl Map {
    pub fn get_longest_path(&self, slippery: bool) -> HashSet<Coord> {
        // What if we do A* but take the largest f_score first?
        let mut set = BTreeSet::from([SetItem {
            score: 0,
            position: self.start,
            path: HashSet::new(),
        }]);
        let mut steps = HashMap::from([(self.start, 0)]);
        let mut paths = Vec::new();

        while let Some(SetItem {
            score: _,
            position: current,
            mut path,
        }) = set.pop_last()
        {
            if current == self.end {
                paths.push(path);
                continue;
            }

            let mut to_add = Vec::new();
            for (neighbor, traveled) in self.get_neighbors(&current, &path, slippery) {
                let tentative_steps = steps[&current] + traveled.len();
                let is_better_path = steps
                    .get(&neighbor)
                    .map_or(true, |&steps| tentative_steps > steps);

                if is_better_path {
                    steps.insert(neighbor, tentative_steps);
                    to_add.push((neighbor, traveled));
                }
            }

            // Clone path for any new paths
            while to_add.len() > 1 {
                let (neighbor, traveled) = to_add.pop().unwrap();
                let mut new_path = path.clone();
                new_path.extend(traveled);
                set.insert(SetItem {
                    score: new_path.len() + neighbor.distance(&self.end),
                    position: neighbor,
                    path: new_path,
                });
            }

            // This should be the final one (if at all) so don't clone
            if let Some((neighbor, traveled)) = to_add.pop() {
                path.extend(traveled);
                set.insert(SetItem {
                    score: path.len() + neighbor.distance(&self.end),
                    position: neighbor,
                    path,
                });
            }
        }

        paths.into_iter().fold(HashSet::new(), |largest, set| {
            if set.len() > largest.len() {
                set
            } else {
                largest
            }
        })
    }

    fn get_neighbors(
        &self,
        position: &Coord,
        path: &HashSet<Coord>,
        slippery: bool,
    ) -> Vec<(Coord, HashSet<Coord>)> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(|(d_row, d_col)| self.get_relative(position, d_row, d_col, path, slippery))
            .collect()
    }

    fn get_relative(
        &self,
        position: &Coord,
        d_row: isize,
        d_col: isize,
        path: &HashSet<Coord>,
        slippery: bool,
    ) -> Option<(Coord, HashSet<Coord>)> {
        let row = position.row.checked_add_signed(d_row)?;
        let col = position.col.checked_add_signed(d_col)?;
        let &tile = self.grid.get(row)?.get(col)?;
        let next = Coord { col, row };
        if path.contains(&next) {
            return None;
        }

        // If it's not slippery then count slopes as '.'
        if !slippery {
            return if tile == '#' {
                None
            } else {
                Some((next, HashSet::from([next])))
            };
        }

        let (last, mut traveled) = match tile {
            '.' => Some((next, HashSet::new())),
            '>' => self.get_relative(&next, 0, 1, path, slippery),
            '^' => self.get_relative(&next, -1, 0, path, slippery),
            '<' => self.get_relative(&next, 0, -1, path, slippery),
            'v' => self.get_relative(&next, 1, 0, path, slippery),
            _ => None,
        }?;

        // If not newly inserted (returns false), that means it's invalid
        if traveled.insert(next) {
            Some((last, traveled))
        } else {
            None
        }
    }

    fn print_path(&self, path: &HashSet<Coord>) {
        for (row, line) in self.grid.iter().enumerate() {
            for (col, &c) in line.iter().enumerate() {
                if path.contains(&Coord { row, col }) {
                    print!("O");
                } else {
                    print!("{c}");
                }
            }
            println!();
        }
        println!();
    }
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let grid: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
        let start = grid
            .first()
            .unwrap()
            .iter()
            .position(|c| *c == '.')
            .unwrap();
        let end = grid.last().unwrap().iter().position(|c| *c == '.').unwrap();
        Self {
            start: Coord { row: 0, col: start },
            end: Coord {
                row: grid.len() - 1,
                col: end,
            },
            grid,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    pub fn distance(&self, other: &Coord) -> usize {
        (self.col as isize - other.col as isize).unsigned_abs()
            + (self.row as isize - other.row as isize).unsigned_abs()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SetItem {
    score: usize,
    position: Coord,
    path: HashSet<Coord>,
}

impl Ord for SetItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score
            .cmp(&other.score)
            .then(self.position.cmp(&other.position))
    }
}

impl PartialOrd for SetItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
