use std::cmp::min;
use std::iter::{empty, once};

use super::*;

#[derive(PartialEq, Debug, Clone)]
pub struct StatePart2<'map> {
    you: ActorState<'map>,
    elephant: ActorState<'map>,
    passed_time: u8,
    opened_flow: u32,
    released_pressure: u32,
    remaining_valves: Valves,
}

#[derive(PartialEq, Debug, Clone)]
struct ActorState<'map> {
    position: &'map ValveId,
    action: Action<'map>,
}

#[derive(PartialEq, Debug, Clone)]
enum Action<'map> {
    Nothing,
    Moving {
        chosen_move: &'map Move<'map>,
        passed: u8,
    },
    Stopped,
}

impl<'map> StatePart2<'map> {
    fn new(position: &'map ValveId, remaining_valves: Valves) -> Self {
        Self {
            you: ActorState {
                action: Action::Nothing,
                position,
            },
            elephant: ActorState {
                action: Action::Nothing,
                position,
            },
            passed_time: 0,
            released_pressure: 0,
            opened_flow: 0,
            remaining_valves,
        }
    }

    fn time_before_next_decision(&self) -> Option<u8> {
        match (
            self.you.action.time_left(),
            self.elephant.action.time_left(),
        ) {
            (Some(your_time), Some(elephant_time)) => Some(min(your_time, elephant_time)),
            (Some(time), None) | (None, Some(time)) => Some(time),
            _ => None,
        }
    }

    fn nexts_action_from_position(
        &'map self,
        actor_state: &'map ActorState,
        timer: u8,
        all_moves: &'map HashMap<&ValveId, Vec<Move<'map>>>,
    ) -> Result<impl Iterator<Item = ActorState<'map>>> {
        Ok(all_moves
            .get(actor_state.position)
            .ok_or_else(|| anyhow!("cannot find moves from {}", actor_state.position))?
            .iter()
            .filter(|move_| self.remaining_valves.contains(move_.target).unwrap())
            .filter(move |move_| self.passed_time + move_.cost <= timer)
            .map(|move_| ActorState {
                position: move_.target,
                action: Action::Moving {
                    chosen_move: move_,
                    passed: 0,
                },
            })
            .chain(once(ActorState {
                position: actor_state.position,
                action: Action::Stopped,
            })))
    }

    fn wait(self, map: &Map) -> Result<Self> {
        let time = self.time_before_next_decision().unwrap_or(0);
        let (you, freed_flow_by_you) = self.you.wait(time, map)?;
        let (elephant, freed_flow_by_elephant) = self.elephant.wait(time, map)?;
        Ok(Self {
            you,
            elephant,
            passed_time: self.passed_time + time,
            opened_flow: self.opened_flow + freed_flow_by_you + freed_flow_by_elephant,
            released_pressure: self.released_pressure + self.opened_flow * u32::from(time),
            remaining_valves: self.remaining_valves,
        })
    }

    fn with_you<'a>(&'a self, actor_state: ActorState<'a>) -> StatePart2<'a> {
        StatePart2 {
            remaining_valves: self.remaining_valves.without(actor_state.position).unwrap(),
            you: actor_state,
            elephant: self.elephant.clone(),
            passed_time: self.passed_time,
            opened_flow: self.opened_flow,
            released_pressure: self.released_pressure,
        }
    }

    fn with_elephant<'a>(&'a self, actor_state: ActorState<'a>) -> StatePart2<'a> {
        StatePart2 {
            remaining_valves: self.remaining_valves.without(actor_state.position).unwrap(),
            elephant: actor_state,
            you: self.you.clone(),
            passed_time: self.passed_time,
            opened_flow: self.opened_flow,
            released_pressure: self.released_pressure,
        }
    }
}

impl<'map> ActorState<'map> {
    fn wait(self, time: u8, map: &Map) -> Result<(Self, u32)> {
        let (action, freed_flow) = self.action.wait(time, map)?;
        Ok((
            Self {
                position: self.position,
                action,
            },
            freed_flow,
        ))
    }
}

impl<'map> Action<'map> {
    fn time_left(&self) -> Option<u8> {
        use Action::*;

        match self {
            Nothing => Some(0),
            Stopped => None,
            Moving {
                chosen_move,
                passed,
            } => Some(chosen_move.cost - passed),
        }
    }

    fn wait(self, time: u8, map: &Map) -> Result<(Self, u32)> {
        use Action::*;

        Ok(match self {
            Moving {
                chosen_move,
                passed,
            } => {
                if passed + time >= chosen_move.cost {
                    (Nothing, map.get(chosen_move.target)?.flow)
                } else {
                    (
                        Moving {
                            chosen_move,
                            passed: passed + time,
                        },
                        0,
                    )
                }
            }
            _ => (self, 0),
        })
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Valves {
    remaining_valves: Vec<bool>,
}

impl Valves {
    fn all(size: usize) -> Self {
        Self {
            remaining_valves: vec![true; size],
        }
    }

    fn contains(&self, id: &ValveId) -> Result<bool> {
        self.remaining_valves
            .get(*id)
            .ok_or_else(|| anyhow!("out of bound ?"))
            .copied()
    }

    fn without(&self, id: &ValveId) -> Result<Self> {
        let mut result = self.clone();
        *result
            .remaining_valves
            .get_mut(*id)
            .ok_or_else(|| anyhow!("out of bound ?"))? = false;
        Ok(result)
    }
}

impl<'map> Startable<'map> for StatePart2<'map> {
    fn starting_state(map: &'map Map) -> Result<Self> {
        Ok(StatePart2::new(
            map.id_of("AA")?,
            Valves::all(map.hasher.nb_created),
        ))
    }
}

impl<'map> State for StatePart2<'map> {
    fn nexts<'a>(
        &'a self,
        map: &'a Map,
        timer: u8,
        all_moves: &'a HashMap<&'a ValveId, Vec<Move<'a>>>,
    ) -> Result<Box<dyn Iterator<Item = Result<Box<dyn State+'a>>> + 'a>>    {
        use Action::*;

        // let to_box_dyn = |res_state:Result<StatePart2>|
        //     res_state.map(|state|Box::new(state) as Box<dyn State>);

        Ok(match (&self.you.action, &self.elephant.action) {
            (Nothing, _) => Box::new(
                self.nexts_action_from_position(&self.you, timer, all_moves)?
                    .map(move |actor_state| self.with_you(actor_state).wait(map))
                    .map(|res_state| res_state.map(|state|Box::new(state) as Box<dyn State+'a>))
                ,
            ),
            (_, Nothing) => Box::new(
                self.nexts_action_from_position(&self.elephant, timer, all_moves)?
                    .map(|actor_state| self.with_elephant(actor_state).wait(map))
                    .map(|res_state| res_state.map(|state|Box::new(state) as Box<dyn State+'a>))
            ) ,
            // this code should never be called... can be replaced with a bail
            (Moving { .. }, _) | (_, Moving { .. }) => Box::new(once(self.clone().wait(map))
                .map(|res_state| res_state.map(|state|Box::new(state) as Box<dyn State+'a>))
            ),
            (Stopped, Stopped) => Box::new(empty())
            ,
        })
    }

    fn released_pressure_at(&self, time: u8) -> u32 {
        self.released_pressure + self.opened_flow * u32::from(time - self.passed_time)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(non_snake_case)]
    #[test]
    fn starting_point_is_AA() {
        let map = parse(&["aoc", "2022", "16-test.txt"]).unwrap();
        let state = StatePart2::starting_state(&map).unwrap();
        assert_eq!(state.passed_time, 0);
        assert_eq!(
            state.you,
            ActorState {
                action: Action::Nothing,
                position: map.id_of("AA").unwrap(),
            }
        );
        assert_eq!(state.released_pressure, 0);
        assert_eq!(state.opened_flow, 0);
    }

    mod wait {
        use crate::part2::Action::*;
        use crate::part2::*;

        #[test]
        fn state_wait_should_incr_flow() {
            let mut hasher = ValveIdHasher::new();
            let aa = *hasher.id_of_or_create("AA");
            let a_move = Move {
                target: &aa,
                cost: 10,
            };

            let state = StatePart2 {
                you: ActorState {
                    action: Moving {
                        chosen_move: &a_move,
                        passed: 0,
                    },
                    position: &aa,
                },
                elephant: ActorState {
                    position: &aa,
                    action: Stopped,
                },
                passed_time: 0,
                remaining_valves: Valves::all(0),
                opened_flow: 2,
                released_pressure: 0,
            };
            let mut map = Map {
                hasher,
                valves: HashMap::new(),
            };
            map.valves.insert(
                aa,
                ValveData {
                    flow: 0,
                    id: aa,
                    tunnels_to: Vec::new(),
                },
            );

            let new_state = state.wait(&map).unwrap();

            assert_eq!(new_state.passed_time, 10);
            assert_eq!(new_state.opened_flow, 2);
            assert_eq!(new_state.released_pressure, 20);
        }
    }

    mod nexts {
        use crate::part2::Action::*;
        use crate::part2::*;

        #[test]
        fn nexts_where_both_are_stopped_should_return_empty() {
            let mut hasher = ValveIdHasher::new();
            let aa = *hasher.id_of_or_create("AA");
            let state = StatePart2 {
                you: ActorState {
                    action: Stopped,
                    position: &aa,
                },
                elephant: ActorState {
                    position: &aa,
                    action: Stopped,
                },
                passed_time: 0,
                remaining_valves: Valves::all(0),
                opened_flow: 0,
                released_pressure: 0,
            };

            let map = Map {
                hasher,
                valves: HashMap::new(),
            };
            let all_moves = HashMap::new();
            let nexts = state
                .nexts(&map, 10, &all_moves)
                .unwrap()
                .collect::<Vec<_>>();

            assert_eq!(nexts.len(), 0)
        }
    }

    #[test]
    fn given_test_in_3_minutes() {
        let map = parse(&["aoc", "2022", "16-test.txt"]).unwrap();
        assert_eq!(map.get_max_released::<StatePart2>(3).unwrap(), 33)
    }

    #[test]
    fn given_test() {
        let map = parse(&["aoc", "2022", "16-test.txt"]).unwrap();
        assert_eq!(map.get_max_released::<StatePart2>(26).unwrap(), 1707)
    }
}
