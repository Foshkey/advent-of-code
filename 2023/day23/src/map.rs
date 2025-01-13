use std::collections::{HashMap, HashSet};

pub struct Map {
    grid: Vec<Vec<char>>,
    slippery: bool,
    start: Coord,
    end: Coord,
}

impl Map {
    pub fn new(input: &str, slippery: bool) -> Self {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
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
            slippery,
        }
    }

    pub fn get_longest_path(&self) -> Option<usize> {
        // Get the junction graph
        let graph = self.get_junction_graph();

        // Exhaustive search over the junction graph to get the longest distance
        let mut set = Vec::from([(self.start, HashMap::from([(self.start, 0)]))]);
        let mut results = Vec::new();

        while let Some((current, mut path)) = set.pop() {
            if current == self.end {
                // Add total distance to results
                results.push(path.values().sum());
                continue;
            }

            // Get any neighbors that aren't on the path so far
            let empty = HashMap::new();
            let mut neighbors: Vec<_> = graph
                .get(&current)
                .unwrap_or(&empty)
                .iter()
                .filter(|(neighbor, _)| !path.contains_key(neighbor))
                .collect();

            // Clone path for any new paths
            while neighbors.len() > 1 {
                let (&neighbor, &distance) = neighbors.pop().unwrap();
                let mut new_path = path.clone();
                new_path.insert(neighbor, distance);
                set.push((neighbor, new_path));
            }

            // This should be the final one (if at all) so don't clone
            if let Some((&neighbor, &distance)) = neighbors.pop() {
                path.insert(neighbor, distance);
                set.push((neighbor, path));
            }
        }

        results.into_iter().max()
    }

    fn get_neighbors(
        &self,
        position: &Coord,
        path: &HashSet<Coord>,
    ) -> Vec<(Coord, HashSet<Coord>)> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(|(d_row, d_col)| self.get_relative(position, d_row, d_col, path))
            .collect()
    }

    fn get_relative(
        &self,
        position: &Coord,
        d_row: isize,
        d_col: isize,
        path: &HashSet<Coord>,
    ) -> Option<(Coord, HashSet<Coord>)> {
        let row = position.row.checked_add_signed(d_row)?;
        let col = position.col.checked_add_signed(d_col)?;
        let &tile = self.grid.get(row)?.get(col)?;
        let next = Coord { col, row };
        if path.contains(&next) {
            return None;
        }

        // If it's not slippery then only pay attention to '#'
        if !self.slippery {
            return if tile == '#' {
                None
            } else {
                Some((next, HashSet::from([next])))
            };
        }

        let (last, mut traveled) = match tile {
            '#' => None,
            '>' => self.get_relative(&next, 0, 1, path),
            '^' => self.get_relative(&next, -1, 0, path),
            '<' => self.get_relative(&next, 0, -1, path),
            'v' => self.get_relative(&next, 1, 0, path),
            _ => Some((next, HashSet::new())),
        }?;

        // If not newly inserted (returns false), that means it's invalid
        if traveled.insert(next) {
            Some((last, traveled))
        } else {
            None
        }
    }

    /// Creates a graph between junctions (or spaces with more than 2 neighbors)
    fn get_junction_graph(&self) -> HashMap<Coord, HashMap<Coord, usize>> {
        let mut graph: HashMap<Coord, HashMap<Coord, usize>> = HashMap::new();
        let junctions = self.find_junctions();
        for &junction in &junctions {
            // Exhaustive search to nearby junctions
            let mut queue = Vec::from([(junction, HashSet::from([junction]))]);
            while let Some((current, mut path)) = queue.pop() {
                if current != junction && junctions.contains(&current) {
                    // Found a path to another junction, update the graph
                    let distance = path.len() - 1;
                    graph
                        .entry(junction)
                        .or_default()
                        .entry(current)
                        .and_modify(|e| *e = usize::max(*e, distance))
                        .or_insert(distance);
                    continue;
                }

                let mut neighbors = self.get_neighbors(&current, &path);

                // Clone path for any new paths
                while neighbors.len() > 1 {
                    let (neighbor, traveled) = neighbors.pop().unwrap();
                    let mut new_path = path.clone();
                    new_path.extend(traveled);
                    queue.push((neighbor, new_path));
                }

                // This should be the final one (if at all) so don't clone
                if let Some((neighbor, traveled)) = neighbors.pop() {
                    path.extend(traveled);
                    queue.push((neighbor, path));
                }
            }
        }
        graph
    }

    /// Returns a set of all junctions, which is a space with more than 2 neighbors
    fn find_junctions(&self) -> HashSet<Coord> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(|(col, &c)| {
                        if c == '#' {
                            return None;
                        }

                        let position = Coord { row, col };
                        if position == self.start
                            || position == self.end
                            || self.get_neighbors(&position, &HashSet::new()).len() > 2
                        {
                            Some(position)
                        } else {
                            None
                        }
                    })
                    .collect::<HashSet<Coord>>()
            })
            .collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Coord {
    row: usize,
    col: usize,
}
