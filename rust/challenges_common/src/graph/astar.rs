use core::cmp::Reverse;
use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    ops::Add,
    rc::Rc,
};

fn astar<N, C, Nexts>(
    starting_at: N,
    next: impl Fn(&N) -> Nexts,
    is_end: impl Fn(&N) -> bool,
    heuristic: impl Fn(&N) -> C,
) -> Option<Path<N, C>>
where
    N: Node<C>,
    C: Cost,
    Nexts: Iterator<Item = Step<N, C>>,
{
    type Queue<'n, N, C> = BinaryHeap<Reverse<Rc<NodeInfo<N, C>>>>;
    type Optimals<'n, N, C> = HashMap<Rc<N>, Rc<NodeInfo<N, C>>>;

    fn add_node<'n, N, C>(
        queue: &mut Queue<'n, N, C>,
        optimals: &mut Optimals<'n, N, C>,
        node: Rc<N>,
        info: NodeInfo<N, C>,
    ) where
        N: Node<C>,
        C: Cost,
    {
        let info = Rc::new(info);
        if is_improvement(&optimals, &info) {
            queue.push(Reverse(info.clone()));
            optimals.insert(node, info);
        }
    }

    fn node_info<'n, N, C>(
        from: Rc<NodeInfo<N, C>>,
        to: Rc<N>,
        additionnal_cost: C,
        optimals: &Optimals<N, C>,
        heuristic: impl Fn(&N) -> C,
    ) -> NodeInfo<N, C>
    where
        N: Node<C>,
        C: Cost,
    {
        let existing = optimals.get(&to);
        let heuristic = existing
            .map(|existing| existing.heuristic)
            .unwrap_or_else(|| heuristic(&to));
        let cost = from.cost + additionnal_cost;
        NodeInfo {
            node: to,
            previous_ancestor: Some(from),
            cost,
            heuristic,
        }
    }

    fn is_improvement<N: Node<C>, C: Cost>(
        optimals: &Optimals<N, C>,
        info: &NodeInfo<N, C>,
    ) -> bool {
        match optimals.get(&info.node) {
            Some(existing) => info.cost < existing.cost,
            None => true,
        }
    }

    fn is_optimal<'n, N, C>(optimals: &Optimals<N, C>, info: &Rc<NodeInfo<N, C>>) -> bool
    where
        N: Node<C>,
        C: Cost,
    {
        match optimals.get(&info.node) {
            Some(existing) => existing == info,
            None => true,
        }
    }

    fn rebuild_path<N: Node<C>, C: Cost>(
        optimals: &mut Optimals<N, C>,
        from: Rc<NodeInfo<N, C>>,
    ) -> Path<N, C> {
        // let from = &optimals.get(from).unwrap();
        let cost = from.cost;
        let mut nodes = Vec::new();

        let mut current = Some(from);
        while let Some(node_info) = current {
            nodes.push(N::clone(&node_info.node));
            current = match &node_info.previous_ancestor {
                Some(node_info) => Some(Rc::clone(node_info)),
                None => None,
            }
        }
        nodes.reverse();

        Path { nodes, cost }
    }

    let mut queue: Queue<N, C> = BinaryHeap::new();
    let mut optimals: Optimals<N, C> = HashMap::new();
    let start = Rc::new(starting_at);

    let info = NodeInfo::start(start.clone(), &heuristic);
    add_node(&mut queue, &mut optimals, start, info);

    while let Some(Reverse(from)) = queue.pop() {
        if is_optimal(&optimals, &from) {
            let reached = &from.node;

            if is_end(reached) {
                return Some(rebuild_path(&mut optimals, from));
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

    None
}

trait Cost: Ord + Copy + Add<Output = Self> + Sized {}
impl<T: Ord + Copy + Add<Output = Self> + Sized> Cost for T {}

trait Node<Cost>
where
    Self: Hash + Eq + Clone,
    Cost: self::Cost,
{
    fn cost(&self) -> Cost;
}

#[derive(PartialEq, Eq)]
struct NodeInfo<N, C>
where
    N: Node<C>,
    C: Cost,
{
    node: Rc<N>,
    previous_ancestor: Option<Rc<NodeInfo<N, C>>>,
    cost: C,
    heuristic: C,
}

impl<N: Node<C>, C: Cost> NodeInfo<N, C> {
    fn start(start: Rc<N>, heuristic: &dyn Fn(&N) -> C) -> Self {
        let cost = N::cost(&start);
        let heuristic = heuristic(&start);
        Self {
            node: start,
            previous_ancestor: None,
            cost,
            heuristic,
        }
    }

    fn score(&self) -> C {
        self.cost + self.heuristic
    }
}

impl<'n, N: Node<C>, C: Cost> PartialOrd for NodeInfo<N, C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'n, N: Node<C>, C: Cost> Ord for NodeInfo<N, C> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score().cmp(&other.score())
    }
}

#[derive(PartialEq, Eq)]
struct Path<Node, Cost> {
    nodes: Vec<Node>,
    cost: Cost,
}

#[derive(PartialEq, Eq)]
struct Step<Node, Cost> {
    to: Node,
    additionnal_cost: Cost,
}

impl<T, Cost> Node<Cost> for T
where
    Cost: self::Cost + Default,
    T: Hash + Eq + Clone,
{
    fn cost(&self) -> Cost {
        Cost::default()
    }
}

#[cfg(test)]
mod test {
    use std::iter::once;

    use super::{astar, Path, Step};

    struct CustomGraph {
        edges: Vec<(u8, u8, u8)>,
    }

    impl CustomGraph {
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
