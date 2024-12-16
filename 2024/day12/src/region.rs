use std::collections::HashSet;

use crate::coord::Coord;

#[derive(Debug)]
pub struct Region {
    plots: HashSet<Coord>,
}

impl Region {
    pub fn new(plot: Coord) -> Self {
        Region {
            plots: HashSet::from([plot]),
        }
    }

    pub fn perimeter_len(&self) -> usize {
        self.plots
            .iter()
            .map(|plot| {
                let neighbors_count = self.get_neighbors(*plot).len();
                4 - neighbors_count
            })
            .sum()
    }

    pub fn count(&self) -> usize {
        self.plots.len()
    }

    pub fn count_sides(&self) -> usize {
        // Number of sides should be the same as number of corners
        let mut corners = 0;

        for &plot in &self.plots {
            let neighbors = self.get_neighbors(plot);
            let neighbors_len = neighbors.len();

            // If no neighbors, it has 4 corners, continue on
            if neighbors_len == 0 {
                corners += 4;
                continue;
            }

            // If 1 neighbor, then 2 corners, continue on
            if neighbors_len == 1 {
                corners += 2;
                continue;
            }

            // Check outside corners, like this example
            // ...
            // XX.
            // XX.
            if neighbors_len == 2
                && !(neighbors[0].row() == neighbors[1].row()
                    || neighbors[0].col() == neighbors[1].col())
            {
                corners += 1;
            }

            // And then finally count all the inside corners, like this example
            // XX.
            // XXX
            // XXX
            // This is a little more tricky since we have to look at diagonals
            for (d_row, d_col) in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
                if self.get_relative(plot, d_row, d_col).is_some() {
                    // The diagonal is part of the region, it's not a corner
                    continue;
                }

                // If neighbors of both the plot and the diagonal are part of the region,
                // then it's a corner
                if self.get_relative(plot, d_row, 0).is_some()
                    && self.get_relative(plot, 0, d_col).is_some()
                {
                    corners += 1;
                }
            }
        }

        corners
    }

    pub fn touches(&self, plot: Coord) -> bool {
        for p in self.plots.iter() {
            if p.touches(plot) {
                return true;
            }
        }

        false
    }

    pub fn insert(&mut self, plot: Coord) -> bool {
        self.plots.insert(plot)
    }

    pub fn merge(&mut self, other: &Region) {
        for plot in other.plots.iter() {
            self.plots.insert(*plot);
        }
    }

    fn get_neighbors(&self, plot: Coord) -> Vec<Coord> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(|(d_row, d_col)| self.get_relative(plot, d_row, d_col))
            .collect()
    }

    fn get_relative(&self, plot: Coord, d_row: isize, d_col: isize) -> Option<Coord> {
        let row = plot.row().checked_add_signed(d_row)?;
        let col = plot.col().checked_add_signed(d_col)?;
        let relative = Coord::new(row, col);
        if self.plots.contains(&relative) {
            Some(relative)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perimeter() {
        // AAAA
        // AAA
        let mut region = Region::new(Coord::new(0, 0));
        for (row, col) in [(0, 1), (0, 2), (0, 3), (1, 0), (1, 1), (1, 2)] {
            region.insert(Coord::new(row, col));
        }
        assert_eq!(region.perimeter_len(), 12);

        //  A
        // AAA
        // A
        let mut region = Region::new(Coord::new(0, 1));
        for (row, col) in [(1, 0), (1, 1), (1, 2), (2, 1)] {
            region.insert(Coord::new(row, col));
        }
        assert_eq!(region.perimeter_len(), 12);

        // AAAAAA
        let mut region = Region::new(Coord::new(0, 0));
        for (row, col) in [(0, 1), (0, 2), (0, 3), (0, 4), (0, 5)] {
            region.insert(Coord::new(row, col));
        }
        assert_eq!(region.perimeter_len(), 14);
    }
}
