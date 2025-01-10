use std::collections::{HashMap, HashSet};

use crate::brick::Brick;

pub struct World {
    /// A map where the key is the ONLY supporting brick for the set
    bricks: HashMap<Brick, HashSet<Brick>>,
}

impl World {
    pub fn new(bricks: impl Iterator<Item = Brick>) -> Self {
        let mut bricks: Vec<Brick> = bricks.collect();
        bricks.sort();

        // Settle all the bricks
        let mut settled: Vec<&Brick> = Vec::new();
        for brick in bricks.iter_mut() {
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

        // Build up the support map
        let bricks = bricks
            .iter()
            .enumerate()
            .map(|(i, brick)| {
                (
                    *brick,
                    // Find any bricks that this brick is supporting
                    bricks[i..]
                        .iter()
                        .enumerate()
                        .filter(|(j, supported)| {
                            // brick supports this brick
                            brick.supports(supported)
                                // and there is no other brick that supports this brick
                                && !bricks[..i + j]
                                    .iter()
                                    .any(|other| brick != other && other.supports(supported))
                        })
                        .map(|(_, supported)| *supported)
                        .collect(),
                )
            })
            .collect();

        World { bricks }
    }

    pub fn count_safe(&self) -> usize {
        // If the brick isn't the only supporter of any other bricks, then it's safe
        self.bricks
            .iter()
            .filter(|&(_, supported)| supported.is_empty())
            .count()
    }

    pub fn count_chain_reactions(&self) -> usize {
        let mut results: HashMap<Brick, usize> = HashMap::new();
        self.bricks
            .keys()
            .map(|brick| self.count_will_fall(brick, &mut results))
            .sum()
    }

    fn count_will_fall(&self, brick: &Brick, results: &mut HashMap<Brick, usize>) -> usize {
        if let Some(result) = results.get(brick) {
            return *result;
        }

        let Some(set) = self.bricks.get(brick) else {
            return 0;
        };

        let result = set
            .iter()
            .map(|supported| self.count_will_fall(supported, results) + 1)
            .sum();
        results.insert(*brick, result);
        result
    }
}

impl From<&str> for World {
    fn from(s: &str) -> Self {
        Self::new(s.lines().map(|line| line.into()))
    }
}
