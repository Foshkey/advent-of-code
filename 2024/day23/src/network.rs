use std::collections::{BTreeSet, HashMap, HashSet};

pub struct Network {
    connections: HashMap<String, HashSet<String>>,
}

impl Network {
    pub fn get_tri_connections(
        &self,
        filter: impl Fn(&str) -> bool + Copy,
    ) -> HashSet<BTreeSet<String>> {
        self.connections
            .iter()
            .filter(move |&(node, _)| filter(node))
            .flat_map(|(node, neighbors)| {
                neighbors.iter().flat_map(|neighbor| {
                    self.connections[neighbor]
                        .iter()
                        .filter(|&n| neighbors.contains(n))
                        .map(|neighbor_of_neighbor| {
                            BTreeSet::from([
                                node.clone(),
                                neighbor.clone(),
                                neighbor_of_neighbor.clone(),
                            ])
                        })
                })
            })
            .collect()
    }
}

impl From<&str> for Network {
    fn from(s: &str) -> Self {
        let mut connections = HashMap::new();

        for line in s.lines() {
            let Some((left, right)) = line.split_once('-') else {
                continue;
            };

            connections
                .entry(left.to_string())
                .or_insert(HashSet::new())
                .insert(right.to_string());

            connections
                .entry(right.to_string())
                .or_insert(HashSet::new())
                .insert(left.to_string());
        }

        Network { connections }
    }
}
