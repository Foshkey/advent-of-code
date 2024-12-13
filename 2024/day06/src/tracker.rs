use std::collections::{HashMap, HashSet};

pub struct Tracker {
    positions: HashMap<(usize, usize), HashSet<(isize, isize)>>,
}
impl Tracker {
    pub fn new() -> Self {
        Tracker {
            positions: HashMap::new(),
        }
    }

    /// Marks a position with a given direction. If the position already exists,
    /// the direction is added to the set of directions for that position.
    ///
    /// # Arguments
    ///
    /// * `position` - A tuple representing the (row, col) coordinates of the position.
    /// * `direction` - A tuple representing the (d_row, d_col) direction to mark at the position.
    ///
    /// # Returns
    ///
    /// * `true` if the position and direction was newly inserted.
    /// * `false` if the position and direction already existed.
    pub fn mark(&mut self, position: (usize, usize), direction: (isize, isize)) -> bool {
        if let Some(directions) = self.positions.get_mut(&position) {
            directions.insert(direction)
        } else {
            self.positions.insert(position, HashSet::from([direction]));
            true
        }
    }
}
