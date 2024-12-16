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
                let neighbors_count = self.neighbors(*plot).len();
                4 - neighbors_count
            })
            .sum()
    }

    pub fn count(&self) -> usize {
        self.plots.len()
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

    fn neighbors(&self, plot: Coord) -> Vec<Coord> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .filter_map(|(d_row, d_col)| {
                let row = plot.row().checked_add_signed(*d_row)?;
                let col = plot.col().checked_add_signed(*d_col)?;
                let neighbor = Coord::new(row, col);
                if self.plots.contains(&neighbor) {
                    Some(neighbor)
                } else {
                    None
                }
            })
            .collect()
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
