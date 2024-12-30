use std::{collections::HashMap, str::FromStr};

pub struct Processor {
    nodes: HashMap<String, Node>,
}

impl Processor {
    pub fn get_z_value(&self) -> usize {
        let mut z = 0;
        let mut z_node = 0;

        while let Some(node) = self.nodes.get(&format!("z{z_node:0>2}")) {
            if self.get_state(node) {
                z += 2usize.pow(z_node)
            }
            z_node += 1;
        }

        z
    }

    fn get_state(&self, node: &Node) -> bool {
        match node {
            Node::Wire(state) => *state,
            Node::And(left, right) => {
                self.get_state(&self.nodes[left]) & self.get_state(&self.nodes[right])
            }
            Node::Or(left, right) => {
                self.get_state(&self.nodes[left]) | self.get_state(&self.nodes[right])
            }
            Node::Xor(left, right) => {
                self.get_state(&self.nodes[left]) ^ self.get_state(&self.nodes[right])
            }
        }
    }
}

impl FromStr for Processor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = HashMap::new();

        for line in s.lines() {
            let line = line.replace(" ", "");
            if let Some((name, state)) = line.split_once(':') {
                nodes.insert(name.to_string(), state.parse()?);
            }
            if let Some((gate, name)) = line.split_once("->") {
                nodes.insert(name.trim().to_string(), gate.parse()?);
            }
        }

        Ok(Processor { nodes })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Node {
    Wire(bool),
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((left, right)) = s.split_once("AND") {
            Ok(Node::And(left.to_string(), right.to_string()))
        } else if let Some((left, right)) = s.split_once("XOR") {
            Ok(Node::Xor(left.to_string(), right.to_string()))
        } else if let Some((left, right)) = s.split_once("OR") {
            Ok(Node::Or(left.to_string(), right.to_string()))
        } else if let Ok(value) = s.parse::<u8>() {
            Ok(Node::Wire(value != 0))
        } else {
            Err(format!(
                "Input not recognized to be a wire or AND/OR/XOR gate: {s}"
            ))
        }
    }
}
