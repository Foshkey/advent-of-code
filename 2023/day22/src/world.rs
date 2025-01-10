use std::collections::{HashMap, HashSet};

use crate::brick::Brick;

pub struct World {
    /// A map where the key is a supporting brick for the set
    bricks: HashMap<Brick, HashSet<Brick>>,
    /// A map where the key is supported by the set
    supported: HashMap<Brick, HashSet<Brick>>,
}

impl World {
    pub fn new(bricks: impl Iterator<Item = Brick>) -> Self {
        let mut bricks_vec: Vec<Brick> = bricks.collect();
        bricks_vec.sort();

        // Settle all the bricks
        let mut settled: Vec<&Brick> = Vec::new();
        for brick in bricks_vec.iter_mut() {
            let new_z = settled
                .iter()
                .filter(|&&other| brick.will_collide(other))
                .map(|&other| other.max_z())
                .max()
                .unwrap_or_default()
                + 1;
            brick.move_to(new_z);
            settled.push(brick);
        }

        // Build up the maps
        let mut supported = HashMap::new();
        let bricks = bricks_vec
            .iter()
            .enumerate()
            .map(|(i, brick)| {
                (
                    *brick,
                    // Find any bricks that this brick is supporting
                    bricks_vec[i..]
                        .iter()
                        .filter(|supported_brick| brick.supports(supported_brick))
                        .map(|supported_brick| {
                            // Also add to the supported map
                            supported
                                .entry(*supported_brick)
                                .or_insert(HashSet::new())
                                .insert(*brick);
                            *supported_brick
                        })
                        .collect(),
                )
            })
            .collect();

        World { bricks, supported }
    }

    pub fn count_safe(&self) -> usize {
        // Go through each brick
        self.bricks
            .iter()
            .filter(|(_, set)| {
                // Check all of the bricks that are supported by this brick
                set.iter().all(|brick| {
                    // If it's supported by more than 1, then we're good
                    self.supported
                        .get(brick)
                        .map(|s| s.len())
                        .unwrap_or_default()
                        > 1
                })
            })
            .count()
    }

    pub fn count_chain_reactions(&self) -> usize {
        self.bricks
            .keys()
            .map(|brick| self.count_falling(brick))
            .sum()
    }

    fn count_falling(&self, brick: &Brick) -> usize {
        // This is a set of falling bricks
        let mut falling = HashSet::from([*brick]);
        // And cloning the supported map since we'll removing entries as we go along
        let mut supported = self.supported.clone();

        // Keep searching for more bricks that will fall
        while let Some((&another, _)) = supported
            .iter()
            // If the entire supporting set is within the set of falling bricks
            .find(|(_, set)| set.is_subset(&falling))
        {
            // It's also falling
            falling.insert(another);
            // And remove it from supported so we don't consider it again
            supported.remove(&another);
        }

        // Minus one because we're not counting the original brick
        falling.len() - 1
    }
}

impl From<&str> for World {
    fn from(s: &str) -> Self {
        Self::new(s.lines().map(|line| line.into()))
    }
}
