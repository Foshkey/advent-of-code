use std::collections::HashMap;

use crate::{coord::Coord, region::Region};

#[derive(Debug)]
pub struct Map {
    regions: HashMap<char, Vec<Region>>,
}

impl Map {
    pub fn regions(&self) -> Vec<&Region> {
        self.regions.values().flatten().collect()
    }

    fn add_plot(&mut self, key: char, plot: Coord) {
        // Get the regions for the letter, if there are none then create one and return out
        let Some(key_regions) = self.regions.get_mut(&key) else {
            self.regions.insert(key, vec![Region::new(plot)]);
            return;
        };

        // See if any of the regions touches this plot, if so then add it and return out
        for region in key_regions.iter_mut() {
            if region.touches(plot) {
                region.insert(plot);
                return;
            }
        }

        // If we're at this point then create a new region with this plot
        key_regions.push(Region::new(plot))
    }
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let mut map = Map {
            regions: HashMap::new(),
        };

        for (row, line) in s.lines().enumerate() {
            for (col, c) in line.char_indices() {
                let plot = Coord::new(row, col);
                map.add_plot(c, plot);
            }
        }

        map
    }
}
