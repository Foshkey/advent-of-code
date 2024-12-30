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

    pub fn find_maximum_clique(&self) -> HashSet<String> {
        let mut max_clique = HashSet::new();
        let mut r = HashSet::new();
        let mut p = self.connections.keys().cloned().collect::<HashSet<_>>();
        let mut x = HashSet::new();

        self.bron_kerbosch(&mut r, &mut p, &mut x, &mut max_clique);

        max_clique
    }

    fn bron_kerbosch(
        &self,
        r: &mut HashSet<String>,
        p: &mut HashSet<String>,
        x: &mut HashSet<String>,
        max_clique: &mut HashSet<String>,
    ) {
        // Gotta love graph theory. Classic algorithm to find maximum clique.
        if p.is_empty() && x.is_empty() {
            if r.len() > max_clique.len() {
                *max_clique = r.clone();
            }
            return;
        }

        let pivot = p.union(x).next().unwrap().clone();
        let p_without_neighbors = p
            .difference(&self.connections[&pivot])
            .cloned()
            .collect::<HashSet<_>>();

        for v in p_without_neighbors {
            r.insert(v.clone());
            let mut new_p = p
                .intersection(&self.connections[&v])
                .cloned()
                .collect::<HashSet<_>>();
            let mut new_x = x
                .intersection(&self.connections[&v])
                .cloned()
                .collect::<HashSet<_>>();
            self.bron_kerbosch(r, &mut new_p, &mut new_x, max_clique);
            r.remove(&v);
            p.remove(&v);
            x.insert(v);
        }
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
