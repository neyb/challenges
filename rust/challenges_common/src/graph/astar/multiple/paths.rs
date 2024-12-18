use std::collections::HashSet;
use std::hash::Hash;

pub struct Pathes<N> {
    nodes: HashSet<N>,
}

impl<N: Hash + Eq> Pathes<N> {
    pub fn new(nodes: HashSet<N>) -> Self {
        Self { nodes }
    }

    pub fn nodes(&self) -> &HashSet<N> {
        &self.nodes
    }

    pub fn add(&mut self, node: N) {
        self.nodes.insert(node);
    }
}
