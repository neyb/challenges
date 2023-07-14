use super::*;

pub trait Part1 {
    fn get_max_released(&self, timer: u8) -> Result<u32>;
}

#[derive(PartialEq, Debug)]
struct State<'map> {
    position: &'map ValveId,
    passed_time: u8,
    opened_flow: u32,
    released_pressure: u32,
    remaining_valves: Vec<&'map ValveId>,
}

impl Part1 for Map {
    fn get_max_released(&self, timer: u8) -> Result<u32> {
        let all_moves = self.all_moves();
        return rec_explore(self, State::starting_state(&self, "AA")?, timer, &all_moves);

        fn rec_explore(
            map: &Map,
            from_state: State,
            timer: u8,
            all_moves: &HashMap<&ValveId, Vec<Move>>,
        ) -> Result<u32> {
            from_state
                .nexts(map, timer, all_moves)?
                .try_fold(
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
            position,
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
        all_moves: &'map HashMap<&ValveId, Vec<Move>>,
    ) -> Result<impl Iterator<Item=Result<State<'map>>>> {
        Ok(all_moves
            .get(self.position)
            .ok_or_else(|| anyhow!("cannot find moves from {}", self.position))?
            .iter()
            .filter(|a_move| self.remaining_valves.contains(&a_move.target))
            .map(|a_move| {
                let position_info = map.get(a_move.target)?;
                Ok(Self {
                    position: a_move.target,
                    opened_flow: self.opened_flow + position_info.flow,
                    remaining_valves: self
                        .remaining_valves
                        .clone()
                        .into_iter()
                        .filter(|valve| valve != &a_move.target)
                        .collect(),
                    released_pressure: self.released_pressure_after(a_move.cost),
                    passed_time: self.passed_time + a_move.cost,
                })
            })
            .filter(move |res_state| match res_state {
                Ok(state) => state.passed_time <= timer,
                _ => true,
            }))
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use crate::part1::*;

    #[allow(non_snake_case)]
    #[test]
    fn starting_point_is_AA() {
        let map = parse(&["aoc", "2022", "16-test.txt"]).unwrap();
        let state = State::starting_state(&map, "AA").unwrap();
        assert_eq!(state.passed_time, 0);
        assert_eq!(state.position, &parse_valve_id("AA").unwrap());
        assert_eq!(state.released_pressure, 0);
        assert_eq!(state.opened_flow, 0);
    }

    #[test]
    fn given_test_in_3_minutes() {
        let map = parse(&["aoc", "2022", "16-test.txt"]).unwrap();
        assert_eq!(map.get_max_released(3).unwrap(), 20)
    }

    #[test]
    fn given_test() {
        let map = parse(&["aoc", "2022", "16-test.txt"]).unwrap();
        assert_eq!(map.get_max_released(30).unwrap(), 1651)
    }
}
