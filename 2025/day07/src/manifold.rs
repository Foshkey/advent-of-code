use std::collections::{BTreeSet, HashMap, HashSet};

pub struct Manifold {
    len: usize,
    splitters: BTreeSet<(usize, usize)>,
    start: (usize, usize),
}

impl From<&str> for Manifold {
    fn from(value: &str) -> Self {
        let mut splitters = BTreeSet::new();
        let mut start = (0, 0);

        for (row, line) in value.lines().enumerate() {
            for (col, c) in line.char_indices() {
                match c {
                    'S' => {
                        start = (row, col);
                    }
                    '^' => {
                        splitters.insert((row, col));
                    }
                    _ => {}
                }
            }
        }

        Self {
            splitters,
            start,
            len: value.lines().count(),
        }
    }
}

impl Manifold {
    pub fn get_num_splits(&self) -> usize {
        let mut beams = HashSet::from([self.start.1]);
        let mut count = 0;

        for row in 0..self.len {
            let mut split_beams = HashSet::new();
            let mut remove_beams = HashSet::new();

            for beam in beams.iter() {
                if self.splitters.contains(&(row, *beam)) {
                    remove_beams.insert(*beam);
                    split_beams.insert(beam - 1);
                    split_beams.insert(beam + 1);
                    count += 1;
                }
            }

            beams.extend(&split_beams);
            beams.retain(|b| !remove_beams.contains(b));
        }

        count
    }

    pub fn get_num_timelines(&self) -> usize {
        self.get_num_timelines_rec(self.start, &mut HashMap::new())
    }

    fn get_num_timelines_rec(
        &self,
        beam: (usize, usize),
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(&cached_result) = cache.get(&beam) {
            return cached_result;
        }

        let (row, col) = beam;

        if row >= self.len {
            return 1;
        }

        let result = if self.splitters.contains(&beam) {
            self.get_num_timelines_rec((row, col - 1), cache)
                + self.get_num_timelines_rec((row, col + 1), cache)
        } else {
            self.get_num_timelines_rec((row + 1, col), cache)
        };

        cache.insert(beam, result);
        result
    }
}
