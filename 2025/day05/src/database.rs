use std::ops::RangeInclusive;

pub struct Database {
    fresh: Vec<RangeInclusive<usize>>,
    available: Vec<usize>,
}

impl From<&str> for Database {
    fn from(value: &str) -> Self {
        let mut fresh = Vec::new();
        let mut available = Vec::new();

        for line in value.lines() {
            if let Some((left, right)) = line.split_once('-') {
                let left = left.parse().unwrap_or_else(|s| panic!("Not a number: {s}"));
                let right = right
                    .parse()
                    .unwrap_or_else(|s| panic!("Not a number: {s}"));
                fresh.push(left..=right);
            } else if !line.is_empty() {
                let id = line.parse().unwrap_or_else(|s| panic!("Not a number: {s}"));
                available.push(id);
            }
        }

        Self { fresh, available }
    }
}

impl Database {
    pub fn count_available_fresh(&self) -> usize {
        self.available
            .iter()
            .filter(|id| self.fresh.iter().any(|range| range.contains(id)))
            .count()
    }

    pub fn count_all_fresh(&mut self) -> usize {
        while self.merge_fresh_ranges() > 0 {}
        self.fresh.iter().map(|r| r.end() + 1 - r.start()).sum()
    }

    fn merge_fresh_ranges(&mut self) -> usize {
        let mut new_ranges: Vec<RangeInclusive<usize>> = Vec::new();
        let mut merge_count = 0;

        'outer: while let Some(range) = self.fresh.pop() {
            for new_range in new_ranges.iter_mut() {
                if range.start() <= new_range.end() && new_range.start() <= range.end() {
                    // Ranges overlap, merge them
                    let merged_start = *range.start().min(new_range.start());
                    let merged_end = *range.end().max(new_range.end());
                    *new_range = merged_start..=merged_end;
                    merge_count += 1;
                    continue 'outer;
                }
            }

            new_ranges.push(range);
        }

        self.fresh = new_ranges;
        merge_count
    }
}
