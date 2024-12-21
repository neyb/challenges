use super::*;

#[derive(PartialEq, Debug)]
pub struct StatePart1<'map> {
    position: &'map ValveId,
    passed_time: u8,
    opened_flow: u32,
    released_pressure: u32,
    remaining_valves: Vec<&'map ValveId>,
}

impl<'map> StatePart1<'map> {
    fn new(position: &'map ValveId, remaining_valves: Vec<&'map ValveId>) -> Self {
        Self {
            position,
            passed_time: 0,
            released_pressure: 0,
            opened_flow: 0,
            remaining_valves,
        }
    }
}

impl<'map> Startable<'map> for StatePart1<'map> {
    fn starting_state(map: &'map Map) -> Result<Self> {
        let position = map.id_of("AA")?;
        let valves_to_open = map.valves_to_open();
        Ok(StatePart1::new(position, valves_to_open))
    }
}

impl State for StatePart1<'_> {
    fn nexts<'a>(
        &'a self,
        map: &'a Map,
        timer: u8,
        all_moves: &'a HashMap<&'a ValveId, Vec<Move<'a>>>,
    ) -> Result<Box<dyn Iterator<Item = Result<Box<dyn State + 'a>>> + 'a>> {
        Ok(Box::new(
            all_moves
                .get(self.position)
                .ok_or_else(|| anyhow!("cannot find moves from {}", self.position))?
                .iter()
                .filter(|a_move| self.remaining_valves.contains(&a_move.target))
                .map(|a_move| {
                    let position_info = map.get(a_move.target)?;
                    Ok(StatePart1 {
                        position: a_move.target,
                        opened_flow: self.opened_flow + position_info.flow,
                        remaining_valves: self
                            .remaining_valves
                            .clone()
                            .into_iter()
                            .filter(|valve| valve != &a_move.target)
                            .collect(),
                        released_pressure: self
                            .released_pressure_at(self.passed_time + a_move.cost),
                        passed_time: self.passed_time + a_move.cost,
                    })
                })
                .filter(move |res_state| match res_state {
                    Ok(state) => state.passed_time <= timer,
                    _ => true,
                })
                .map(|res_state| res_state.map(|state| Box::new(state) as Box<dyn State>)),
        ))
    }

    fn released_pressure_at(&self, time: u8) -> u32 {
        self.released_pressure + self.opened_flow * u32::from(time - self.passed_time)
    }
}

#[cfg(test)]
mod test {
    use crate::part1::*;

    #[allow(non_snake_case)]
    #[test]
    fn starting_point_is_AA() {
        let map = parse(&["aoc", "2022", "16-test.txt"]).unwrap();
        let aa_id = map.id_of("AA").unwrap();
        let state = StatePart1::starting_state(&map).unwrap();
        assert_eq!(state.passed_time, 0);
        assert_eq!(state.position, aa_id);
        assert_eq!(state.released_pressure, 0);
        assert_eq!(state.opened_flow, 0);
    }

    #[test]
    fn given_test_in_3_minutes() {
        let map = parse(&["aoc", "2022", "16-test.txt"]).unwrap();
        assert_eq!(map.get_max_released::<StatePart1>(3).unwrap(), 20)
    }

    #[test]
    fn given_test() {
        let map = parse(&["aoc", "2022", "16-test.txt"]).unwrap();
        assert_eq!(map.get_max_released::<StatePart1>(30).unwrap(), 1651)
    }
}
