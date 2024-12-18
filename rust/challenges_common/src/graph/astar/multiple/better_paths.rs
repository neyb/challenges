use itertools::Itertools;
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub struct Paths<N> {
    starts: Vec<PE<N>>,
}

#[derive(Eq, PartialEq)]
struct PathElement<N> {
    node: N,
    nexts: Vec<Link<N>>,
}

impl<N: Hash> Hash for PathElement<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node.hash(state);
    }
}

type Link<N> = Option<PE<N>>;
type PE<N> = Rc<RefCell<PathElement<N>>>;

impl<N: Hash> Hash for PE<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.borrow().hash(state);
    }
}

impl<N: Hash + Eq> Paths<N> {
    fn prepend(&mut self, node: N, to: &N) {}
    // fn prepend(&mut self, node: N) {
    //     let new_start = Rc::new(PathElement {
    //         node,
    //         nexts: vec![Some(self.start.clone())],
    //     });
    //     self.start = new_start;
    // }

    fn path_elemens(&self) -> HashSet<&PE<N>> {
        let mut result = HashSet::new();
        let mut queue = VecDeque::new();
        queue.extend(&self.starts);

        while let Some(current) = queue.pop_front() {
            let links = current
                .borrow()
                .nexts
                .iter()
                .cloned()
                .flatten()
                .collect_vec();
            queue.extend(&links);
            result.insert(current);
        }

        result
    }

    pub fn iter(&self) -> impl Iterator<Item = &N> + use<'_, N> {
        self.path_elemens().map(|element| &element.borrow().node)
    }
}
