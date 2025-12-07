use std::collections::HashSet;

pub struct Manifold {
    len: usize,
    splitters: HashSet<(usize, usize)>,
    start: (usize, usize),
}

impl From<&str> for Manifold {
    fn from(value: &str) -> Self {
        let mut splitters = HashSet::new();
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
}
