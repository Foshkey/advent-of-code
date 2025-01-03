use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    hash::Hash,
    str::FromStr,
};

#[derive(Clone)]
pub struct Processor {
    inputs: BTreeMap<String, bool>,
    gates: HashMap<String, Gate>,
    bit_size: u8,
}

impl Processor {
    pub fn get_value(&self, letter: char) -> Result<usize, String> {
        let mut value = 0;

        for node_num in 0..=self.bit_size {
            if self.get_state(&format!("{letter}{node_num:0>2}"))? {
                value += 2usize.pow(node_num as u32)
            }
        }

        Ok(value)
    }

    pub fn get_wrong_gates(&self) -> BTreeSet<String> {
        // Cloning self as mutable so that we can set inputs and test them
        let mut test_processor = self.clone();
        test_processor.set_y_test_state();
        test_processor.find_swaps()
    }

    fn get_state(&self, key: &String) -> Result<bool, String> {
        self.get_state_rec(key, &mut HashSet::new())
    }

    fn get_state_rec(&self, key: &String, tracker: &mut HashSet<String>) -> Result<bool, String> {
        // Check if it's input
        if let Some(state) = self.inputs.get(key) {
            return Ok(*state);
        }

        // Check if we're in a recursive loop
        if tracker.contains(key) {
            return Err(format!(
                "Recursive loop detected, encountered {key} before while computing state"
            ));
        }
        tracker.insert(key.clone());

        // Get the gate
        let gate = self
            .gates
            .get(key)
            .ok_or(format!("Could not find gate or input: {key}"))?;

        // Recursively process the gate, cloning the tracker for left leafs of the tree
        Ok(match gate {
            Gate::And(left, right) => {
                self.get_state_rec(left, &mut tracker.clone())?
                    & self.get_state_rec(right, tracker)?
            }
            Gate::Or(left, right) => {
                self.get_state_rec(left, &mut tracker.clone())?
                    | self.get_state_rec(right, tracker)?
            }
            Gate::Xor(left, right) => {
                self.get_state_rec(left, &mut tracker.clone())?
                    ^ self.get_state_rec(right, tracker)?
            }
        })
    }

    fn find_swaps(&mut self) -> BTreeSet<String> {
        let mut swaps = BTreeSet::new();
        let mut passed = false;
        let original_gates: Vec<_> = self.gates.keys().cloned().collect();

        while !passed {
            let Err(mut last_failed_at) = self.test() else {
                break;
            };

            // Loop through all swap combinations and find a swap that makes it better
            let mut best = None;
            for (i, gate1) in original_gates.iter().enumerate() {
                for (j, gate2) in original_gates.iter().enumerate() {
                    if i >= j {
                        continue;
                    }

                    // Swap the gates and test
                    self.swap_gates(gate1, gate2);

                    // Run the test, figure out where it fails
                    if let Err(failed_at) = self.test() {
                        println!("{gate1} {gate2} failed at {failed_at}");
                        // If we found a higher bit, save it
                        if failed_at > last_failed_at {
                            println!(
                                "Swap candidate found: {gate1} {gate2}, failed at {failed_at}"
                            );
                            best = Some((gate1, gate2));
                            last_failed_at = failed_at;
                        }
                    } else {
                        // If okay, then passed, break out
                        passed = true;
                        best = Some((gate1, gate2));
                        break;
                    }

                    // Swap back and continue testing
                    self.swap_gates(gate1, gate2);
                }

                if passed {
                    break;
                }
            }

            // Save the best as a permanent swap
            if let Some((gate1, gate2)) = best {
                self.swap_gates(gate1, gate2);
                swaps.insert(gate1.clone());
                swaps.insert(gate2.clone());
            }
        }

        swaps
    }

    fn swap_gates(&mut self, gate1: &String, gate2: &String) {
        if let (Some(gate1_value), Some(gate2_value)) =
            (self.gates.remove(gate1), self.gates.remove(gate2))
        {
            self.gates.insert(gate1.clone(), gate2_value);
            self.gates.insert(gate2.clone(), gate1_value);
        }
    }

    /// Sets the bit at n (from the right) to true
    fn set_x_state(&mut self, n: u8) {
        self.inputs.insert(format!("x{n:0>2}"), true);
    }

    /// Sets x to 0 (all 0s)
    fn reset_x_state(&mut self) {
        let start = "x00".to_string();
        let end = format!("x{:0>2}", self.bit_size - 1);
        for (_, state) in self.inputs.range_mut(start..=end) {
            *state = false;
        }
    }

    /// Sets y to 1 (all 0s with last being 1)
    fn set_y_test_state(&mut self) {
        // Set y00 to 1
        if let Some(state) = self.inputs.get_mut("y00") {
            *state = true
        }

        let start = "y01".to_string();
        let end = format!("y{:0>2}", self.bit_size - 1);
        for (_, state) in self.inputs.range_mut(start..=end) {
            *state = false;
        }
    }

    /// Tests the adder architecture by looping through all the bits on x while y is set to 1
    fn test(&mut self) -> Result<(), u8> {
        self.reset_x_state();
        for bit in 0..self.bit_size {
            // Sets the next bit from 0 to 1
            self.set_x_state(bit);

            // Expected true bit is the next one up (E.g. 001 + 111 = 1000)
            let expected_true_bit = bit + 1;

            // Test all the bits
            for b in 0..=self.bit_size {
                let state = self.get_state(&format!("z{b:0>2}")).map_err(|_| bit)?;

                // If it's not the expected true bit and true, or
                // if it's the expected true bit and false
                if (b != expected_true_bit && state) || (b == expected_true_bit && !state) {
                    // It's wrong
                    return Err(bit);
                }
            }
        }

        Ok(())
    }
}

impl FromStr for Processor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inputs = BTreeMap::new();
        let mut gates = HashMap::new();
        let mut bit_size = 0u8;

        for line in s.lines() {
            let line = line.replace(" ", "");
            if let Some((name, state)) = line.split_once(':') {
                let state = state
                    .parse::<u8>()
                    .map_err(|_| format!("Input is not 0 or 1: {state}"))?;
                inputs.insert(name.to_string(), state != 0);
            }
            if let Some((gate, name)) = line.split_once("->") {
                if let Some(bit_str) = name.strip_prefix('z') {
                    if let Ok(bit) = bit_str.parse::<u8>() {
                        bit_size = u8::max(bit_size, bit);
                    }
                }
                gates.insert(name.trim().to_string(), gate.parse()?);
            }
        }

        Ok(Processor {
            inputs,
            gates,
            bit_size,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Gate {
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

impl FromStr for Gate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((left, right)) = s.split_once("AND") {
            Ok(Gate::And(left.to_string(), right.to_string()))
        } else if let Some((left, right)) = s.split_once("XOR") {
            Ok(Gate::Xor(left.to_string(), right.to_string()))
        } else if let Some((left, right)) = s.split_once("OR") {
            Ok(Gate::Or(left.to_string(), right.to_string()))
        } else {
            Err(format!(
                "Input not recognized to be an AND/OR/XOR gate: {s}"
            ))
        }
    }
}
