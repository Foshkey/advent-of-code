use std::collections::HashMap;

use crate::{coord::Coord, region::Region};

#[derive(Debug)]
pub struct Map {
    regions: HashMap<char, Vec<Region>>,
}

impl Map {
    pub fn get_fence_cost(&self) -> usize {
        self.regions
            .iter()
            .flat_map(|(_, regions)| regions)
            .map(|region| region.perimeter_len() * region.count())
            .sum()
    }

    pub fn get_fence_cost_with_bulk_discount(&self) -> usize {
        self.regions
            .iter()
            .flat_map(|(_, regions)| regions)
            .map(|region| region.count_sides() * region.count())
            .sum()
    }

    fn add_plot(&mut self, key: char, plot: Coord) {
        // Get the regions for the letter, if there are none then create one and return out
        let Some(key_regions) = self.regions.get_mut(&key) else {
            self.regions.insert(key, vec![Region::new(plot)]);
            return;
        };

        // See if any of the regions touches this plot, if so then add it and return out
        let mut touching_regions = key_regions
            .iter_mut()
            .enumerate()
            .filter(|(_, region)| region.touches(plot));

        if let Some((_, base_region)) = touching_regions.next() {
            base_region.insert(plot);

            // Merge other regions if they're touching as well
            let mut indices_to_remove: Vec<_> = touching_regions
                .map(|(index, region)| {
                    base_region.merge(region);
                    index
                })
                .collect();

            while let Some(index) = indices_to_remove.pop() {
                key_regions.remove(index);
            }

            return;
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
