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

    pub fn perimeter(&self) -> usize {
        1
    }

    pub fn len(&self) -> usize {
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
}
