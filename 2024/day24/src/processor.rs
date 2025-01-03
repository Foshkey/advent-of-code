use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    str::FromStr,
};

#[derive(Clone)]
pub struct Processor {
    inputs: HashMap<String, bool>,
    gates: HashMap<String, Gate>,
    bit_size: u8,
}

impl Processor {
    pub fn get_output(&self) -> usize {
        let mut value = 0;

        for node_num in 0..=self.bit_size {
            if self.get_state(&format!("z{node_num:0>2}")) {
                value += 2usize.pow(node_num as u32)
            }
        }

        value
    }

    fn get_state(&self, key: &String) -> bool {
        // Check if it's an input
        if let Some(state) = self.inputs.get(key) {
            return *state;
        }

        // Get the gate
        let Gate {
            inputs: [left_input, right_input],
            op,
        } = self.gates.get(key).unwrap();

        // Recursively process the gate
        let left = self.get_state(left_input);
        let right = self.get_state(right_input);
        match op {
            Op::And => left & right,
            Op::Or => left | right,
            Op::Xor => left ^ right,
        }
    }

    pub fn get_wrong_gates(&self) -> HashSet<String> {
        let mut wrong = HashSet::new();

        for (output, Gate { inputs, op }) in &self.gates {
            // If it's an output wire coming from something other than XOR (except last)
            if output.starts_with('z')
                && *op != Op::Xor
                && output != &format!("z{:0>2}", self.bit_size)
            {
                wrong.insert(output.clone());
            }

            // If it's an XOR and doesn't touch any input/output wires
            if *op == Op::Xor
                && !output.starts_with('z')
                && !inputs.iter().any(|i| self.inputs.contains_key(i))
            {
                wrong.insert(output.clone());
            }

            // AND gates can only go into OR gates (except first)
            if *op == Op::And
                && !inputs.contains(&"x00".to_string())
                && self
                    .gates
                    .values()
                    .any(|gate| gate.inputs.contains(output) && gate.op != Op::Or)
            {
                wrong.insert(output.clone());
            }

            // XOR gates should never go into OR gates
            if *op == Op::Xor
                && self
                    .gates
                    .values()
                    .any(|gate| gate.inputs.contains(output) && gate.op == Op::Or)
            {
                wrong.insert(output.clone());
            }
        }

        wrong
    }
}

impl FromStr for Processor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inputs = HashMap::new();
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
pub struct Gate {
    inputs: [String; 2],
    op: Op,
}

impl FromStr for Gate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((left, right)) = s.split_once("AND") {
            Ok(Gate {
                inputs: [left.to_string(), right.to_string()],
                op: Op::And,
            })
        } else if let Some((left, right)) = s.split_once("XOR") {
            Ok(Gate {
                inputs: [left.to_string(), right.to_string()],
                op: Op::Xor,
            })
        } else if let Some((left, right)) = s.split_once("OR") {
            Ok(Gate {
                inputs: [left.to_string(), right.to_string()],
                op: Op::Or,
            })
        } else {
            Err(format!(
                "Input not recognized to be an AND/OR/XOR gate: {s}"
            ))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Op {
    And,
    Or,
    Xor,
}
