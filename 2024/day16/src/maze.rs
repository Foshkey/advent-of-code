use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub struct BestPaths {
    pub score: usize,
    pub best_tiles: HashSet<(usize, usize)>,
}

pub struct Maze {
    start: (usize, usize),
    end: (usize, usize),
    // True represents empty space
    grid: Vec<Vec<bool>>,
}

impl Maze {
    pub fn get_best_paths(&self, search_all_paths: bool) -> Option<BestPaths> {
        let start_position = Position {
            coords: self.start,
            direction: (0, 1),
        };
        let mut heap = BinaryHeap::new();
        heap.push(State {
            score: 0,
            position: start_position,
            path: HashSet::from([self.start]),
        });

        let mut scores = HashMap::new();
        scores.insert(start_position, 0);

        let mut final_score = None;
        let mut best_tiles = HashSet::new();

        while let Some(State {
            score,
            position,
            path,
        }) = heap.pop()
        {
            if position.coords == self.end {
                let final_score = *final_score.get_or_insert(score);

                if score == final_score {
                    best_tiles.extend(path);
                }

                if !search_all_paths {
                    break;
                }

                continue;
            }

            if let Some(&stored_score) = scores.get(&position) {
                if score > stored_score {
                    // We already found a better way
                    continue;
                }
            }

            for (cost, next_position) in self.get_next_positions(position) {
                let next_score = score + cost;
                let is_better_path = scores
                    .get(&next_position)
                    .map_or(true, |&score| next_score <= score);

                if !is_better_path {
                    continue;
                }

                scores.insert(next_position, next_score);
                let mut path = path.clone();
                path.insert(next_position.coords);
                heap.push(State {
                    score: next_score,
                    position: next_position,
                    path,
                });
            }
        }

        final_score.map(|score| BestPaths { score, best_tiles })
    }

    fn get_next_positions(&self, position: Position) -> Vec<(usize, Position)> {
        let mut next_positions = Vec::new();

        // Continue straight
        if let Some(coords) = self.get_next(position) {
            next_positions.push((
                1,
                Position {
                    coords,
                    direction: position.direction,
                },
            ));
        }

        // Turns
        for turn in self.get_turns(position.direction) {
            next_positions.push((
                1000,
                Position {
                    direction: turn,
                    coords: position.coords,
                },
            ));
        }

        next_positions
    }

    fn get_next(&self, position: Position) -> Option<(usize, usize)> {
        let row = position.coords.0.checked_add_signed(position.direction.0)?;
        let col = position.coords.1.checked_add_signed(position.direction.1)?;
        let is_empty = self.grid.get(row)?.get(col)?;
        let new_position = (row, col);

        if *is_empty {
            Some(new_position)
        } else {
            None
        }
    }

    fn get_turns(&self, direction: (isize, isize)) -> Vec<(isize, isize)> {
        match direction {
            (-1, 0) => vec![(0, -1), (0, 1)],
            (1, 0) => vec![(0, 1), (0, -1)],
            (0, -1) => vec![(1, 0), (-1, 0)],
            (0, 1) => vec![(1, 0), (-1, 0)],
            _ => vec![],
        }
    }
}

impl From<&str> for Maze {
    fn from(s: &str) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let grid = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.char_indices()
                    .map(|(col, c)| {
                        if c == 'S' {
                            start = (row, col)
                        }
                        if c == 'E' {
                            end = (row, col)
                        }
                        c != '#'
                    })
                    .collect()
            })
            .collect();
        Maze { start, end, grid }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    score: usize,
    position: Position,
    path: HashSet<(usize, usize)>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Position {
    coords: (usize, usize),
    direction: (isize, isize),
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .coords
            .cmp(&self.coords)
            .then_with(|| self.direction.cmp(&other.direction))
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
