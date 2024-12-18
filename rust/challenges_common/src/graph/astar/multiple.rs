use crate::graph::astar::unwrap_rc_or_panic;
use crate::graph::{Cost, Node, Step};
use std::cell::RefCell;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::rc::Rc;

mod paths;

pub fn astar_multiple<N, C, Nexts>(
    starting_at: N,
    mut next: impl FnMut(&N) -> Nexts,
    is_end: impl Fn(&N) -> bool,
    heuristic: impl Fn(&N) -> C,
) -> Option<(HashSet<N>, C)>
where
    N: Node,
    C: Cost,
    Nexts: IntoIterator<Item = Step<N, C>>,
{
    let mut queue: Queue<N, C> = BinaryHeap::new();
    let mut optimals: Optimals<N, C> = HashMap::new();
    let mut ends = Vec::new();
    {
        let mut ends_costs = None;

        let info = AggregatedNodeInfo::start(starting_at, &heuristic);
        add_node(&mut queue, &mut optimals, info);

        while let Some(Reverse(from)) = queue.pop() {
            if let Some(ends_costs) = ends_costs {
                if from.borrow().score() > ends_costs {
                    break;
                }
            }

            if is_optimal(&optimals, &from, ends_costs) {
                // usefull clone ?
                let reached = { &from.borrow().node.clone() };

                if is_end(reached) {
                    ends_costs = Some(from.borrow().cost);
                    ends.push(from.clone());
                } else {
                    for next_step in next(reached) {
                        let info = node_info(
                            from.clone(),
                            Rc::new(next_step.to),
                            next_step.additional_cost,
                            &optimals,
                            &heuristic,
                        );
                        add_node(&mut queue, &mut optimals, info);
                    }
                }
            }
        }
    }

    return rebuild_paths(ends, queue, optimals);

    type Queue<N, C> = BinaryHeap<Reverse<Rc<RefCell<AggregatedNodeInfo<N, C>>>>>;
    type Optimals<N, C> = HashMap<Rc<N>, Rc<RefCell<AggregatedNodeInfo<N, C>>>>;

    fn is_optimal<N: Node, C: Cost>(
        optimals: &Optimals<N, C>,
        info: &Rc<RefCell<AggregatedNodeInfo<N, C>>>,
        ends_cost: Option<C>,
    ) -> bool {
        if let Some(ends_cost) = ends_cost {
            if info.borrow().score() > ends_cost {
                return false;
            }
        }

        match optimals.get(&info.borrow().node) {
            Some(existing) => info.borrow().cost <= existing.borrow().cost,
            None => true,
        }
    }

    fn add_node<N, C>(
        queue: &mut Queue<N, C>,
        optimals: &mut Optimals<N, C>,
        mut info: AggregatedNodeInfo<N, C>,
    ) where
        N: Node,
        C: Cost,
    {
        let to_add = match optimals.get(&info.node) {
            None => {
                let info = Rc::new(RefCell::new(info));
                let node = Rc::clone(&info.as_ref().borrow().node);
                Some((node, info))
            }
            Some(existing) => {
                let ordering = { info.cost.cmp(&existing.borrow().cost) };
                match { ordering } {
                    Ordering::Less => {
                        let info = Rc::new(RefCell::new(info));
                        let node = Rc::clone(&info.borrow().node);
                        Some((node, info))
                    }
                    Ordering::Equal => {
                        if let (Some(existing_previous_ancestors), Some(info_previous_ancestors)) = (
                            &mut existing.borrow_mut().previous_ancestors,
                            &info.previous_ancestors,
                        ) {
                            existing_previous_ancestors
                                .extend(info_previous_ancestors.iter().cloned());
                        };
                        None
                    }
                    Ordering::Greater => None,
                }
            }
        };

        if let Some((node, info)) = to_add {
            queue.push(Reverse(Rc::clone(&info)));
            optimals.insert(node, info);
        }
    }

    fn rebuild_paths<N: Node, C: Cost>(
        ends: Vec<Rc<RefCell<AggregatedNodeInfo<N, C>>>>,
        queue: Queue<N, C>,
        optimals: Optimals<N, C>,
    ) -> Option<(HashSet<N>, C)> {
        drop(queue);
        drop(optimals);

        if ends.is_empty() {
            return None;
        }

        let mut nodes = HashSet::new();
        let cost = { ends[0].borrow().cost };

        for end in ends {
            let mut queue = VecDeque::new();
            queue.push_back(end);

            while let Some(end) = queue.pop_front() {
                nodes.insert(end.borrow().node.clone());

                if let Some(previous_ancestors) = &end.borrow().previous_ancestors {
                    for nodeInfo in previous_ancestors {
                        queue.push_back(nodeInfo.clone());
                    }
                }
            }
        }

        Some((nodes.into_iter().map(unwrap_rc_or_panic).collect(), cost))
    }

    fn node_info<N, C>(
        from: Rc<RefCell<AggregatedNodeInfo<N, C>>>,
        to: Rc<N>,
        additional_cost: C,
        optimals: &Optimals<N, C>,
        heuristic: impl Fn(&N) -> C,
    ) -> AggregatedNodeInfo<N, C>
    where
        N: Node,
        C: Cost,
    {
        let heuristic = match optimals.get(&to) {
            Some(existing) => existing.borrow().heuristic,
            _ => heuristic(&to),
        };

        AggregatedNodeInfo {
            node: to,
            cost: from.borrow().cost + additional_cost,
            previous_ancestors: Some(vec![from.clone()]),
            heuristic,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct AggregatedNodeInfo<N, C>
where
    N: Node,
    C: Cost,
{
    node: Rc<N>,
    previous_ancestors: Option<Vec<Rc<RefCell<AggregatedNodeInfo<N, C>>>>>, // remove refcell
    cost: C,
    heuristic: C,
}

impl<N: Node, C: Cost> AggregatedNodeInfo<N, C> {
    fn start(start: N, heuristic: &dyn Fn(&N) -> C) -> Self {
        let heuristic = heuristic(&start);
        Self {
            node: Rc::new(start),
            previous_ancestors: None,
            cost: C::default(),
            heuristic,
        }
    }

    fn score(&self) -> C {
        self.cost + self.heuristic
    }
}

impl<N: Node, C: Cost> PartialOrd for AggregatedNodeInfo<N, C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<N: Node, C: Cost> Ord for AggregatedNodeInfo<N, C> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score().cmp(&other.score())
    }
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
                    additional_cost: *weight,
                })
        }

        fn path(&self, from: u8, to: u8) -> Option<(HashSet<u8>, u8)> {
            astar_multiple(from, |&n| self.next(n), |&n| n == to, |_| 0)
        }
    }

    #[derive(Hash, PartialEq, Eq, Clone)]
    #[allow(dead_code)]
    struct WeightedNode {
        id: u8,
    }

    #[test]
    fn with_1_edge() {
        let (nodes, cost) = astar_multiple(
            0,
            |_| {
                once(Step {
                    to: 1,
                    additional_cost: 1,
                })
            },
            |n| n == &1,
            |_| 0,
        )
        .unwrap();

        assert_eq!(cost, 1);
        assert_eq!(nodes, { HashSet::from_iter(vec![0, 1]) });
    }

    #[test]
    fn with_1_edge_with_graph() {
        let (nodes, cost) = CustomGraph::same_weights(vec![(0, 1)]).path(0, 1).unwrap();

        assert_eq!(cost, 1);
        assert_eq!(nodes, HashSet::from_iter(vec![0, 1]));
    }

    #[test]
    fn with_5_edges() {
        let (nodes, cost) = CustomGraph::same_weights(vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)])
            .path(0, 5)
            .unwrap();

        assert_eq!(cost, 5);
        assert_eq!(nodes, HashSet::from_iter(vec![0, 1, 2, 3, 4, 5]));
    }

    #[test]
    fn with_3_edges_and_2_paths() {
        let (nodes, cost) = CustomGraph::same_weights(vec![(0, 1), (0, 2), (2, 3), (1, 3)])
            .path(0, 3)
            .unwrap();

        assert_eq!(cost, 2);
        assert_eq!(nodes, HashSet::from_iter(vec![0, 1, 2, 3]));
    }

    #[test]
    fn with_no_path() {
        let paths = CustomGraph::same_weights(vec![(0, 1), (0, 2), (2, 3), (1, 3)]).path(0, 5);

        assert_eq!(paths, None);
    }

    // #[test]
    // fn with_different_paths() {
    //     let path = CustomGraph {
    //         edges: vec![(0, 1, 10), (1, 3, 20), (0, 2, 20), (2, 3, 5)],
    //     }
    //     .path(0, 3)
    //     .unwrap();
    //
    //     assert_eq!(path.cost, 25);
    //     assert_eq!(path.nodes, vec![0, 2, 3]);
    // }
    //
    // #[test]
    // fn with_different_paths_but_heuristic_favor_a_longest() {
    //     let path = CustomGraph {
    //         edges: vec![(0, 1, 10), (1, 3, 20), (0, 2, 20), (2, 3, 5)],
    //     }
    //     .path_with_heuristic(0, 3, |n| match n {
    //         0 => 0,
    //         1 => 0,
    //         2 => 100,
    //         3 => 0,
    //         _ => panic!("..."),
    //     })
    //     .unwrap();
    //
    //     assert_eq!(path.cost, 30);
    //     assert_eq!(path.nodes, vec![0, 1, 3]);
    // }
}
