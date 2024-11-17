use std::fmt::Debug;
use std::rc::Rc;

pub fn detect_cycle<S, N>(init_state: S, next_state: N) -> Option<DetectedCycle<S>>
where
    N: FnMut(&S) -> Option<S>,
    S: Eq + Debug,
{
    let mut generator = Generator::new(init_state, next_state);
    let DetectedCycle {
        start_index,
        size,
        start,
        second_cycle_start,
    } = detect_cycle_for_gen(&mut generator)?;

    drop(generator);

    Some(DetectedCycle {
        start_index,
        size,
        start: Rc::try_unwrap(start).unwrap(),
        second_cycle_start: Rc::try_unwrap(second_cycle_start).unwrap(),
    })
}

pub fn forecast_state<S, N>(init_state: S, next_state: N, target_index: usize) -> S
where
    N: FnMut(&S) -> Option<S>,
    S: Eq + Debug,
{
    Rc::try_unwrap({
        let mut generator = Generator::new(init_state, next_state);
        let index = get_identical_cycle_index(&mut generator, target_index);
        generator.at(index).unwrap()
    })
    .unwrap()
}

fn detect_cycle_for_gen<S, N>(gen: &mut Generator<S, N>) -> Option<DetectedCycle<Rc<S>>>
where
    S: Eq + Debug,
    N: FnMut(&S) -> Option<S>,
{
    let mut turtle_index = 0usize;
    let mut hare_index = 0usize;

    // first round
    while {
        turtle_index += 1;
        hare_index += 2;
        gen.at(turtle_index)? != gen.at(hare_index)?
    } {}
    let first_meet_at = turtle_index;

    // second round
    hare_index = turtle_index;
    while {
        turtle_index += 1;
        hare_index += 2;
        gen.at(turtle_index)? != gen.at(hare_index)?
    } {}
    let second_meet_at = turtle_index;

    let size = second_meet_at - first_meet_at;

    let start_index = {
        let mut index = first_meet_at;
        loop {
            if gen.at(index) != gen.at(index + size) {
                break index + 1;
            }
            if index == 0 {
                break 0;
            }
            index -= 1;
        }
    };

    Some(DetectedCycle {
        start_index,
        size,
        start: gen.at(start_index).unwrap(),
        second_cycle_start: gen.at(start_index + size).unwrap(),
    })
}

fn get_identical_cycle_index<S, N>(gen: &mut Generator<S, N>, target_index: usize) -> usize
where
    N: FnMut(&S) -> Option<S>,
    S: Eq + Debug,
{
    match detect_cycle_for_gen(gen) {
        Some(cycle) if target_index >= cycle.start_index => {
            ((target_index - cycle.start_index) % cycle.size) + cycle.start_index
        }
        _ => target_index,
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
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

#[cfg(test)]
mod tests {
    #[test]
    fn plus_3_mod_7_should_be_a_cycle() {
        let cycle = super::detect_cycle(12, |i| Some((i + 3) % 7)).unwrap();
        assert_eq!(cycle.start_index, 1);
        assert_eq!(cycle.size, 7);
        assert_eq!(cycle.start, 1);
        assert_eq!(cycle.second_cycle_start, 1);
    }

    #[test]
    fn forecasting_plus_3_mod_7_to_28() {
        let forecasted = super::forecast_state(12, |i| Some((i + 3) % 7), 28);
        assert_eq!(forecasted, 5);
    }

    #[test]
    fn forecasting_plus_3_mod_7_to_29() {
        let forecasted = super::forecast_state(12, |i| Some((i + 3) % 7), 29);
        assert_eq!(forecasted, 1);
    }

    #[test]
    fn forecasting_plus_3_mod_7_to_30() {
        let forecasted = super::forecast_state(12, |i| Some((i + 3) % 7), 30);
        assert_eq!(forecasted, 4);
    }

    #[test]
    fn three_first_int_are_not_a_cycle() {
        let v = [1, 2, 3];
        let mut i = 0;
        let cycle = super::detect_cycle(v[0], |_| {
            i += 1;
            v.get(i).cloned()
        });
        assert_eq!(cycle, None);
    }
}
