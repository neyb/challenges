use super::*;
use anyhow::bail;
use std::cmp::min;
use std::iter::{empty, once};

pub trait Part2 {
    fn get_max_released(&self, timer: u8) -> Result<u32>;
}

#[derive(PartialEq, Debug, Clone)]
struct State<'map> {
    you: ActorState<'map>,
    elephant: ActorState<'map>,
    passed_time: u8,
    opened_flow: u32,
    released_pressure: u32,
    remaining_valves: Vec<&'map ValveId>,
}

#[derive(PartialEq, Debug, Clone)]
struct ActorState<'map> {
    position: &'map ValveId,
    action: Action<'map>,
}

#[derive(PartialEq, Debug, Clone)]
enum Action<'map> {
    Nothing,
    Moving { chosen_move: Move<'map>, passed: u8 },
    Stopped,
}

impl Part2 for Map {
    fn get_max_released(&self, timer: u8) -> Result<u32> {
        let all_moves = self.all_moves();
        return rec_explore(self, State::starting_state(&self, "AA")?, timer, &all_moves);

        fn rec_explore(
            map: &Map,
            from_state: State,
            timer: u8,
            all_moves: &HashMap<&ValveId, Vec<Move>>,
        ) -> Result<u32> {
            from_state.nexts(map, timer, all_moves)?.try_fold(
                from_state.released_pressure_after(timer - from_state.passed_time),
                |max, next_state| {
                    next_state.and_then(|next_state| {
                        rec_explore(map, next_state, timer, all_moves)
                            .map(|curr_max| max.max(curr_max))
                    })
                },
            )
        }
    }
}

impl<'map> State<'map> {
    fn starting_state<'a>(map: &'a Map, id: &str) -> Result<State<'a>> {
        let (position, _) = map
            .valves
            .get_key_value(&parse_valve_id(id)?)
            .ok_or_else(|| anyhow!("cannot find {} in map", id))?;
        let valves_to_open = map
            .valves
            .iter()
            .filter(|(_id, data)| data.flow > 0)
            .map(|(id, _data)| id)
            .collect::<Vec<_>>();
        Ok(State::new(position, valves_to_open))
    }

    fn new(position: &'map ValveId, remaining_valves: Vec<&'map ValveId>) -> Self {
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

    fn released_pressure_after(&self, duration: u8) -> u32 {
        self.released_pressure + self.opened_flow * u32::from(duration)
    }

    fn nexts(
        &'map self,
        map: &'map Map,
        timer: u8,
        all_moves: &'map HashMap<&ValveId, Vec<Move<'map>>>,
    ) -> Result<Box<dyn Iterator<Item = Result<State<'map>>> + 'map>> {
        use Action::*;

        let result: Box<dyn Iterator<Item = Result<State<'map>>>> =
            match (&self.you.action, &self.elephant.action) {
                (Nothing, _) => Box::new(
                    self.nexts_action_from_position(&self.you, timer, all_moves)?
                        .map(|actor_state| {
                            let remaining_valves = self
                                .remaining_valves
                                .clone()
                                .into_iter()
                                .filter(|valve| valve != &actor_state.position)
                                .collect();
                            Ok(Self {
                                you: actor_state,
                                remaining_valves,
                                ..self.clone()
                            })
                        }),
                ),
                (_, Nothing) => Box::new(
                    self.nexts_action_from_position(&self.elephant, timer, all_moves)?
                        .map(|actor_state| {
                            let remaining_valves = self
                                .remaining_valves
                                .clone()
                                .into_iter()
                                .filter(|valve| valve != &actor_state.position)
                                .collect();
                            Ok(Self {
                                elephant: actor_state,
                                remaining_valves,
                                ..self.clone()
                            })
                        }),
                ),
                (Moving { .. }, _) | (_, Moving { .. }) => {
                    Box::new(once(Ok(self.wait(self.time_before_next_decision()?, map)?)))
                }
                (Stopped, Stopped) => Box::new(empty()),
            };

        return Ok(result);
    }

    fn time_before_next_decision(&self) -> Result<u8> {
        match (
            self.you.action.time_left(),
            self.elephant.action.time_left(),
        ) {
            (Some(your_time), Some(elephant_time)) => Ok(min(your_time, elephant_time)),
            (Some(time), None) | (None, Some(time)) => Ok(time),
            _ => bail!("both stopped ? {:?}", self),
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
            .filter(|move_| self.remaining_valves.contains(&move_.target))
            .filter(move |move_| self.passed_time + move_.cost <= timer)
            .map(|move_| ActorState {
                position: move_.target,
                action: Action::Moving {
                    chosen_move: move_.clone(),
                    passed: 0,
                },
            })
            .chain(once(ActorState {
                position: actor_state.position,
                action: Action::Stopped,
            })))
    }

    fn wait(&self, time: u8, map: &Map) -> Result<Self> {
        let (you, freed_flow_by_you) = self.you.wait(time, map)?;
        let (elephant, freed_flow_by_elephant) = self.elephant.wait(time, map)?;
        Ok(Self {
            you,
            elephant,
            passed_time: self.passed_time + time,
            opened_flow: self.opened_flow + freed_flow_by_you + freed_flow_by_elephant,
            released_pressure: self.released_pressure + self.opened_flow * u32::from(time),
            remaining_valves: self.remaining_valves.clone(),
        })
    }
}

impl<'map> ActorState<'map> {
    fn wait(&self, time: u8, map: &Map) -> Result<(Self, u32)> {
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

    fn wait(&self, time: u8, map: &Map) -> Result<(Self, u32)> {
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
                            chosen_move: chosen_move.clone(),
                            passed: passed + time,
                        },
                        0,
                    )
                }
            }
            _ => (self.clone(), 0),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[allow(non_snake_case)]
    #[test]
    fn starting_point_is_AA() {
        let map = parse(&["aoc", "2022", "16-test.txt"]).unwrap();
        let state = State::starting_state(&map, "AA").unwrap();
        assert_eq!(state.passed_time, 0);
        assert_eq!(
            state.you,
            ActorState {
                action: Action::Nothing,
                position: &parse_valve_id("AA").unwrap()
            }
        );
        assert_eq!(state.released_pressure, 0);
        assert_eq!(state.opened_flow, 0);
    }

    mod wait {
        use crate::part2::*;
        use crate::part2::Action::*;

        #[test]
        fn state_wait_should_incr_flow(){
            let aa = parse_valve_id("AA").unwrap();
            let state = State {
                you: ActorState {
                    action: Stopped,
                    position: &aa,
                },
                elephant: ActorState {
                    position: &aa,
                    action: Stopped,
                },
                passed_time: 0,
                remaining_valves: vec![],
                opened_flow: 2,
                released_pressure: 0,
            };
            let map = Map { valves: HashMap::new() };
            let new_state = state.wait(10, &map).unwrap();
            assert_eq!(new_state, State {
                you: ActorState {
                    action: Stopped,
                    position: &aa,
                },
                elephant: ActorState {
                    position: &aa,
                    action: Stopped,
                },
                passed_time: 10,
                remaining_valves: vec![],
                opened_flow: 2,
                released_pressure: 20,
            })
        }
    }

    mod nexts {
        use crate::part2::*;
        use crate::part2::Action::*;

        #[test]
        fn nexts_where_both_are_stopped_should_return_empty() {
            let aa = parse_valve_id("AA").unwrap();
            let state = State {
                you: ActorState {
                    action: Stopped,
                    position: &aa,
                },
                elephant: ActorState {
                    position: &aa,
                    action: Stopped,
                },
                passed_time: 0,
                remaining_valves: vec![],
                opened_flow: 0,
                released_pressure: 0,
            };

            let map = Map {
                valves: HashMap::new(),
            };
            let all_moves = HashMap::new();
            let nexts = state.nexts(&map, 10, &all_moves).unwrap().collect::<Vec<_>>();

            assert_eq!(nexts.len(), 0)
        }

        #[test]
        fn nexts_where_both_are_moving_should_wait() {
            // let map = parse(&["aoc", "2022", "16-test.txt"]).unwrap();
        }
    }


    #[test]
    fn given_test_in_3_minutes() {
        let map = parse(&["aoc", "2022", "16-test.txt"]).unwrap();
        assert_eq!(map.get_max_released(3).unwrap(), 33)
    }

    #[test]
    fn given_test() {
        let map = parse(&["aoc", "2022", "16-test.txt"]).unwrap();
        assert_eq!(map.get_max_released(26).unwrap(), 1707)
    }
}
