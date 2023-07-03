use core::cmp::Reverse;
use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    ops::Add,
    rc::Rc,
};

pub trait Cost: Ord + Copy + Add<Output = Self> + Sized + Default {}

impl<T: Ord + Copy + Add<Output = Self> + Sized + Default> Cost for T {}

pub trait Node: Hash + Eq + Clone {}

impl<T: Hash + Eq + Clone> Node for T {}

pub fn astar<N, C, Nexts>(
    starting_at: N,
    next: impl Fn(&N) -> Nexts,
    is_end: impl Fn(&N) -> bool,
    heuristic: impl Fn(&N) -> C,
) -> Option<Path<N, C>>
where
    N: Node,
    C: Cost,
    Nexts: Iterator<Item = Step<N, C>>,
{
    let mut queue: Queue<N, C> = BinaryHeap::new();
    let mut optimals: Optimals<N, C> = HashMap::new();
    let start = Rc::new(starting_at);

    let info = NodeInfo::start(start.clone(), &heuristic);
    add_node(&mut queue, &mut optimals, start, info);

    while let Some(Reverse(from)) = queue.pop() {
        if is_optimal(&optimals, &from) {
            let reached = &from.node;

            if is_end(reached) {
                return Some(rebuild_path(from));
            } else {
                for next_step in next(reached) {
                    let to = Rc::new(next_step.to);
                    let info = node_info(
                        from.clone(),
                        Rc::clone(&to),
                        next_step.additionnal_cost,
                        &optimals,
                        &heuristic,
                    );
                    add_node(&mut queue, &mut optimals, to, info);
                }
            }
        }
    }

    return None;

    type Queue<N, C> = BinaryHeap<Reverse<Rc<NodeInfo<N, C>>>>;
    type Optimals<N, C> = HashMap<Rc<N>, Rc<NodeInfo<N, C>>>;

    fn is_optimal<N, C>(optimals: &Optimals<N, C>, info: &Rc<NodeInfo<N, C>>) -> bool
    where
        N: Node,
        C: Cost,
    {
        match optimals.get(&info.node) {
            Some(existing) => existing == info,
            None => true,
        }
    }

    fn add_node<N, C>(
        queue: &mut Queue<N, C>,
        optimals: &mut Optimals<N, C>,
        node: Rc<N>,
        info: NodeInfo<N, C>,
    ) where
        N: Node,
        C: Cost,
    {
        let info = Rc::new(info);
        if is_improvement(&optimals, &info) {
            queue.push(Reverse(info.clone()));
            optimals.insert(node, info);
        }
    }

    fn rebuild_path<N: Node, C: Cost>(from: Rc<NodeInfo<N, C>>) -> Path<N, C> {
        // let from = &optimals.get(from).unwrap();
        let cost = from.cost;
        let mut nodes = Vec::new();

        let mut current = Some(from);
        while let Some(node_info) = current {
            let previous = &node_info.previous_ancestor;
            nodes.push(N::clone(&node_info.node));
            current = match previous {
                Some(node_info) => Some(Rc::clone(node_info)),
                None => None,
            }
        }
        nodes.reverse();

        Path { nodes, cost }
    }

    fn node_info<N, C>(
        from: Rc<NodeInfo<N, C>>,
        to: Rc<N>,
        additionnal_cost: C,
        optimals: &Optimals<N, C>,
        heuristic: impl Fn(&N) -> C,
    ) -> NodeInfo<N, C>
    where
        N: Node,
        C: Cost,
    {
        let heuristic = match optimals.get(&to) {
            Some(existing) => existing.heuristic,
            _ => heuristic(&to),
        };

        NodeInfo {
            node: to,
            cost: from.cost + additionnal_cost,
            previous_ancestor: Some(from),
            heuristic,
        }
    }

    fn is_improvement<N: Node, C: Cost>(optimals: &Optimals<N, C>, info: &NodeInfo<N, C>) -> bool {
        match optimals.get(&info.node) {
            Some(existing) => info.cost < existing.cost,
            None => true,
        }
    }
}

#[derive(PartialEq, Eq)]
struct NodeInfo<N, C>
where
    N: Node,
    C: Cost,
{
    node: Rc<N>,
    previous_ancestor: Option<Rc<NodeInfo<N, C>>>,
    cost: C,
    heuristic: C,
}

impl<N: Node, C: Cost> NodeInfo<N, C> {
    fn start(start: Rc<N>, heuristic: &dyn Fn(&N) -> C) -> Self {
        let heuristic = heuristic(&start);
        Self {
            node: start,
            previous_ancestor: None,
            cost: C::default(),
            heuristic,
        }
    }

    fn score(&self) -> C {
        self.cost + self.heuristic
    }
}

impl<N: Node, C: Cost> PartialOrd for NodeInfo<N, C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<N: Node, C: Cost> Ord for NodeInfo<N, C> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score().cmp(&other.score())
    }
}

#[derive(PartialEq, Eq)]
pub struct Path<Node, Cost> {
    pub nodes: Vec<Node>,
    pub cost: Cost,
}

#[derive(PartialEq, Eq)]
pub struct Step<Node, Cost> {
    pub to: Node,
    pub additionnal_cost: Cost,
}

#[cfg(test)]
mod test {
    use std::iter::once;

    use super::*;

    struct CustomGraph<N, E> {
        edges: Vec<(N, N, E)>,
    }

    impl CustomGraph<u8, u8> {
        fn same_weights(edges: Vec<(u8, u8)>) -> Self {
            Self {
                edges: edges.into_iter().map(|(from, to)| (from, to, 1)).collect(),
            }
        }

        fn next(&self, from: u8) -> impl Iterator<Item = Step<u8, u8>> + '_ {
            self.edges
                .iter()
                .filter(move |(at, _to, _weight)| from == *at)
                .map(|(_from, to, weight)| Step {
                    to: *to,
                    additionnal_cost: *weight,
                })
        }

        fn path(&self, from: u8, to: u8) -> Option<Path<u8, u8>> {
            astar(from, |&n| self.next(n), |&n| n == to, |_| 0)
        }

        fn path_with_heuristic(
            &self,
            from: u8,
            to: u8,
            heuristic: impl Fn(&u8) -> u8,
        ) -> Option<Path<u8, u8>> {
            astar(from, |&n| self.next(n), |&n| n == to, heuristic)
        }
    }

    #[derive(Hash, PartialEq, Eq, Clone)]
    struct WeightedNode {
        id: u8,
    }

    #[test]
    fn with_1_edge() {
        let path = astar(
            0,
            |_| {
                once(Step {
                    to: 1,
                    additionnal_cost: 1,
                })
            },
            |n| n == &1,
            |_| 0,
        )
        .unwrap();

        assert_eq!(path.cost, 1);
        assert_eq!(path.nodes, vec![0, 1]);
    }

    #[test]
    fn with_1_edge_with_graph() {
        let path = CustomGraph::same_weights(vec![(0, 1)]).path(0, 1).unwrap();

        assert_eq!(path.cost, 1);
        assert_eq!(path.nodes, vec![0, 1]);
    }

    #[test]
    fn with_5_edges() {
        let path = CustomGraph::same_weights(vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)])
            .path(0, 5)
            .unwrap();

        assert_eq!(path.cost, 5);
        assert_eq!(path.nodes, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn with_different_paths() {
        let path = CustomGraph {
            edges: vec![(0, 1, 10), (1, 3, 20), (0, 2, 20), (2, 3, 5)],
        }
        .path(0, 3)
        .unwrap();

        assert_eq!(path.cost, 25);
        assert_eq!(path.nodes, vec![0, 2, 3]);
    }

    #[test]
    fn with_different_paths_but_heuristic_favor_a_longest() {
        let path = CustomGraph {
            edges: vec![(0, 1, 10), (1, 3, 20), (0, 2, 20), (2, 3, 5)],
        }
        .path_with_heuristic(0, 3, |n| match n {
            0 => 0,
            1 => 0,
            2 => 100,
            3 => 0,
            _ => panic!("..."),
        })
        .unwrap();

        assert_eq!(path.cost, 30);
        assert_eq!(path.nodes, vec![0, 1, 3]);
    }
}
