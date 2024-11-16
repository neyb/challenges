use std::fmt::Debug;
use std::rc::Rc;

pub fn detect_cycle<S, N>(init_state: S, next_state: N) -> Option<DetectedCycle<S>>
where
    N: FnMut(&S) -> Option<S>,
    S: Eq + Debug,
{
    let mut turtle_index = 0usize;
    let mut hare_index = 0usize;

    let mut gen = Generator::new(init_state, next_state);

    // first round
    while {
        turtle_index += 1;
        hare_index += 2;
        gen.at(turtle_index) != gen.at(hare_index)
    } {}
    let first_meet_at = turtle_index;

    // second round
    hare_index = turtle_index;
    while {
        turtle_index += 1;
        hare_index += 2;
        gen.at(turtle_index) != gen.at(hare_index)
    } {}
    let second_meet_at = turtle_index;

    let cycle_length = second_meet_at - first_meet_at;

    let starting_at_index = {
        let mut index = first_meet_at;
        loop {
            if index == 0 {
                break 0;
            }
            if gen.at(index) != gen.at(index + cycle_length) {
                break index;
            }
            index -= 1;
        }
    };

    let cycle_start = gen.at(starting_at_index).unwrap();
    let second_cycle_start = gen.at(starting_at_index + cycle_length).unwrap();

    drop(gen);

    Some(DetectedCycle {
        start_index: starting_at_index,
        size: cycle_length,
        start: Rc::try_unwrap(cycle_start).unwrap(),
        second_cycle_start: Rc::try_unwrap(second_cycle_start).unwrap(),
    })
}

pub struct DetectedCycle<S> {
    pub start_index: usize,
    pub size: usize,
    pub start: S,
    pub second_cycle_start: S,
}

struct Generator<S, N> {
    states: Vec<Rc<S>>,
    next_state: N,
}

impl<S, N> Generator<S, N>
where
    S: Eq,
    N: FnMut(&S) -> Option<S>,
{
    fn new(init_state: S, next_state: N) -> Self {
        Self {
            states: vec![Rc::new(init_state)],
            next_state,
        }
    }

    fn at(&mut self, index: usize) -> Option<Rc<S>> {
        while self.states.len() <= index {
            self.states
                .push(Rc::new((self.next_state)(self.states.last().unwrap())?));
        }

        Some(self.states[index].clone())
    }
}
