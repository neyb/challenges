use std::collections::HashMap;
use std::iter;
use std::ops::{Add, AddAssign, Sub};
use std::str::FromStr;

use anyhow::anyhow;

use crate::Matter::*;

fn main() {
    let blueprints = parse(&["aoc", "2022", "19.txt"]);
    println!("part1: {}", score_after(&blueprints, 24));
    println!(
        "part2: {}",
        blueprints.get(0).unwrap().best_geodes_count_after(32)
            * blueprints.get(1).unwrap().best_geodes_count_after(32)
            * blueprints.get(2).unwrap().best_geodes_count_after(32)
    );
}

fn parse(path: &[&str]) -> Vec<Blueprint> {
    challenges_common::get_input_lines(path)
        .map(|line| line.parse::<Blueprint>().unwrap())
        .collect()
}

type TimeAmount = u8;
type ResourceAmount = u16;

fn score_after(blueprints: &[Blueprint], duration: TimeAmount) -> ResourceAmount {
    blueprints
        .iter()
        .map(|blueprint| blueprint.num as u16 * blueprint.best_geodes_count_after(duration))
        .sum()
}

#[derive(PartialEq, Debug)]
struct Blueprint {
    num: u8,
    factories: MatterRelatedResources<Cost>,
}

impl Blueprint {
    fn best_geodes_count_after(&self, duration: TimeAmount) -> ResourceAmount {
        let mut filter = StatesFilter {
            minimum_geodes: 0,
            total_duration: duration,
        };
        let max_pertinent_robot = self.max_pertinent_robot(duration);
        let mut cache = Cache::new();
        let previous_buildable_robots = None;
        return rec_best_geodes_count_after(
            &State::new(),
            self,
            duration,
            &mut filter,
            &max_pertinent_robot,
            &mut cache,
            previous_buildable_robots,
        );

        fn rec_best_geodes_count_after(
            state: &State,
            blueprint: &Blueprint,
            count: TimeAmount,
            filter: &mut StatesFilter,
            max_pertinent_matter: &MatterRelatedResources,
            cache: &mut Cache,
            previous_buildable_robots: Option<&MatterRelatedResources<bool>>,
        ) -> ResourceAmount {
            match cache.result_for(state) {
                Some(result) => result,
                None => {
                    let result = if state.passed_time == count {
                        state.resources.geode
                    } else {
                        let (buildable_robots, nexts) = state.nexts(
                            blueprint,
                            filter,
                            max_pertinent_matter,
                            previous_buildable_robots,
                        );

                        nexts.iter().fold(state.resources.geode, {
                            |max, state| {
                                max.max(rec_best_geodes_count_after(
                                    state,
                                    blueprint,
                                    count,
                                    filter,
                                    max_pertinent_matter,
                                    cache,
                                    Some(&buildable_robots),
                                ))
                            }
                        })
                    };
                    cache.store_result(state, result);
                    result
                }
            }
        }
    }

    fn max_pertinent_robot(&self, time_amount: TimeAmount) -> MatterRelatedResources {
        let for_matter = |robot_matter: Matter| {
            MATTERS
                .iter()
                .map(|matter| self.factories.get(*matter).values.get(robot_matter))
                .fold(0, |max, &amount| max.max(amount))
        };

        MatterRelatedResources {
            ore: for_matter(Ore),
            clay: for_matter(Clay),
            obsidian: for_matter(Obsidian),
            geode: time_amount as ResourceAmount,
        }
    }

    fn cost_for(&self, matter: Matter) -> &Cost {
        match matter {
            Ore => &self.factories.ore,
            Clay => &self.factories.clay,
            Obsidian => &self.factories.obsidian,
            Geode => &self.factories.geode,
        }
    }
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (
            _,
            blueprint_num,
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
        ) = lazy_regex::regex_captures!(
            "Blueprint (\\d+): \
                Each ore robot costs (\\d+) ore. \
                Each clay robot costs (\\d+) ore. \
                Each obsidian robot costs (\\d+) ore and (\\d+) clay. \
                Each geode robot costs (\\d+) ore and (\\d+) obsidian.",
            input
        )
        .ok_or_else(|| anyhow!("cannot parse \"{}\"", input))?;

        Ok(Self {
            num: blueprint_num.parse()?,
            factories: MatterRelatedResources {
                ore: Cost::new(MatterRelatedResources::single(
                    ore_robot_ore_cost.parse()?,
                    Ore,
                )),
                clay: Cost::new(MatterRelatedResources::single(
                    clay_robot_ore_cost.parse()?,
                    Ore,
                )),
                obsidian: Cost::new(
                    MatterRelatedResources::single(obsidian_robot_ore_cost.parse()?, Ore)
                        + MatterRelatedResources::single(obsidian_robot_clay_cost.parse()?, Clay),
                ),
                geode: Cost::new(
                    MatterRelatedResources::single(geode_robot_ore_cost.parse()?, Ore)
                        + MatterRelatedResources::single(
                            geode_robot_obsidian_cost.parse()?,
                            Obsidian,
                        ),
                ),
            },
        })
    }
}

const MATTERS: [Matter; 4] = [Ore, Clay, Obsidian, Geode];
#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Matter {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(PartialEq, Debug)]
struct Cost {
    values: MatterRelatedResources,
}

impl Cost {
    fn new(values: MatterRelatedResources) -> Self {
        Cost { values }
    }
}

#[derive(PartialEq, Debug)]
struct State {
    passed_time: TimeAmount,
    last_move_idle: bool,
    resources: MatterRelatedResources,
    robots: MatterRelatedResources,
}

impl State {
    fn new() -> Self {
        Self {
            passed_time: 0,
            last_move_idle: false,
            resources: MatterRelatedResources::default(),
            robots: MatterRelatedResources::single(1, Ore),
        }
    }

    fn nexts(
        &self,
        blueprint: &Blueprint,
        filter: &mut StatesFilter,
        max_pertinent_matter: &MatterRelatedResources,
        previous_buildable_robots: Option<&MatterRelatedResources<bool>>,
    ) -> (MatterRelatedResources<bool>, Vec<Self>) {
        let mut builable_robots = MatterRelatedResources {
            ore: false,
            clay: false,
            obsidian: false,
            geode: false,
        };

        let result = [Geode, Obsidian, Clay, Ore]
            .iter()
            .filter(|&&matter| self.robots.get(matter) < max_pertinent_matter.get(matter))
            .flat_map(|&matter| match self.build_robot(blueprint, matter) {
                None => None,
                Some(next) => {
                    *builable_robots.get_mut(matter) = true;
                    match (self.last_move_idle, previous_buildable_robots) {
                        (true, Some(previous_buildable_robots))
                            if *previous_buildable_robots.get(matter) =>
                        {
                            None
                        }
                        _ => Some(next),
                    }
                }
            })
            .chain(iter::once(self.wait()))
            .filter(|state| filter.should_consider(state))
            .collect();
        (builable_robots, result)
    }

    fn minimum_geodes(&self, total_duration: TimeAmount) -> ResourceAmount {
        self.resources.geode + self.robots.geode * (total_duration - self.passed_time) as u16
    }

    fn maximum_geodes(&self, total_duration: TimeAmount) -> ResourceAmount {
        let time_left = (total_duration - self.passed_time) as ResourceAmount;
        let current_robot_production = self.robots.geode * time_left;
        let future_created_robots_production = if time_left != 0 {
            time_left * (time_left - 1) / 2
        } else {
            0
        };
        self.resources.geode + current_robot_production + future_created_robots_production
    }

    fn build_robot(&self, blueprint: &Blueprint, matter: Matter) -> Option<State> {
        let cost = blueprint.cost_for(matter);
        if self.resources.has(cost) {
            Some(Self {
                last_move_idle: false,
                passed_time: self.passed_time + 1,
                robots: &self.robots + &MatterRelatedResources::single(1, matter),
                resources: &(&self.resources + &self.robots) - &cost.values,
            })
        } else {
            None
        }
    }

    fn wait(&self) -> Self {
        Self {
            last_move_idle: true,
            passed_time: self.passed_time + 1,
            resources: &self.resources + &self.robots,
            robots: self.robots.clone(),
        }
    }
}

struct Cache {
    result_for_time_by_resources_and_robots:
        HashMap<(MatterRelatedResources, MatterRelatedResources), (ResourceAmount, TimeAmount)>,
}

impl Cache {
    fn new() -> Self {
        Self {
            result_for_time_by_resources_and_robots: HashMap::new(),
        }
    }
}

impl Cache {
    fn result_for(&self, state: &State) -> Option<ResourceAmount> {
        self.result_for_time_by_resources_and_robots
            .get(&(state.resources.clone(), state.robots.clone()))
            .filter(|(_, passed)| passed <= &state.passed_time)
            .map(|(result, _)| *result)
    }

    fn store_result(&mut self, state: &State, result: ResourceAmount) {
        self.result_for_time_by_resources_and_robots.insert(
            (state.resources.clone(), state.robots.clone()),
            (result, state.passed_time),
        );
    }
}

#[derive(Clone)]
struct StatesFilter {
    total_duration: TimeAmount,
    minimum_geodes: ResourceAmount,
}

impl StatesFilter {
    fn should_consider(&mut self, state: &State) -> bool {
        self.minimum_geodes = self
            .minimum_geodes
            .max(state.minimum_geodes(self.total_duration));
        state.maximum_geodes(self.total_duration) >= self.minimum_geodes
    }
}

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
struct MatterRelatedResources<T = ResourceAmount> {
    ore: T,
    clay: T,
    obsidian: T,
    geode: T,
}

impl<T> MatterRelatedResources<T> {
    #[allow(dead_code)]
    fn new(ore: T, clay: T, obsidian: T, geode: T) -> Self {
        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }

    fn get(&self, matter: Matter) -> &T {
        match matter {
            Ore => &self.ore,
            Clay => &self.clay,
            Obsidian => &self.obsidian,
            Geode => &self.geode,
        }
    }

    fn get_mut(&mut self, matter: Matter) -> &mut T {
        match matter {
            Ore => &mut self.ore,
            Clay => &mut self.clay,
            Obsidian => &mut self.obsidian,
            Geode => &mut self.geode,
        }
    }
}

impl MatterRelatedResources<ResourceAmount> {
    fn has(&self, cost: &Cost) -> bool {
        MATTERS
            .iter()
            .all(|matter| self.get(*matter) >= cost.values.get(*matter))
    }
}

impl<T: Default + AddAssign> MatterRelatedResources<T> {
    fn single(amount: T, matter: Matter) -> Self {
        let mut result = MatterRelatedResources::default();
        *result.get_mut(matter) += amount;
        result
    }
}

impl<T> Add for MatterRelatedResources<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = MatterRelatedResources<T>;

    fn add(self, other: Self) -> Self::Output {
        &self + &other
    }
}

impl<T> Add for &MatterRelatedResources<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = MatterRelatedResources<T>;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
}

impl<T> Sub for &MatterRelatedResources<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = MatterRelatedResources<T>;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

impl<T: Default> Default for MatterRelatedResources<T> {
    fn default() -> Self {
        Self {
            ore: T::default(),
            clay: T::default(),
            obsidian: T::default(),
            geode: T::default(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing() {
        let blueprints = parse(&["aoc", "2022", "19-test.txt"]);

        let expected = vec![
            Blueprint {
                num: 1,
                factories: MatterRelatedResources {
                    ore: Cost::new(MatterRelatedResources::single(4, Ore)),
                    clay: Cost::new(MatterRelatedResources::single(2, Ore)),
                    obsidian: Cost::new(
                        MatterRelatedResources::single(3, Ore)
                            + MatterRelatedResources::single(14, Clay),
                    ),
                    geode: Cost::new(
                        MatterRelatedResources::single(2, Ore)
                            + MatterRelatedResources::single(7, Obsidian),
                    ),
                },
            },
            Blueprint {
                num: 2,
                factories: MatterRelatedResources {
                    ore: Cost::new(MatterRelatedResources::single(2, Ore)),
                    clay: Cost::new(MatterRelatedResources::single(3, Ore)),
                    obsidian: Cost::new(
                        &MatterRelatedResources::single(3, Ore)
                            + &MatterRelatedResources::single(8, Clay),
                    ),
                    geode: Cost::new(
                        &MatterRelatedResources::single(3, Ore)
                            + &MatterRelatedResources::single(12, Obsidian),
                    ),
                },
            },
        ];
        assert_eq!(blueprints, expected)
    }

    mod given_step_by_step {
        use crate::*;

        fn test_step(
            blueprint: &Blueprint,
            states_filter: &mut StatesFilter,
            builable_robots: Option<&MatterRelatedResources<bool>>,
            prev_state: &State,
            duration: TimeAmount,
            expected_contained_state: &State,
        ) -> MatterRelatedResources<bool> {
            let (buildable_robots, nexts) = prev_state.nexts(
                blueprint,
                states_filter,
                &blueprint.max_pertinent_robot(duration),
                builable_robots,
            );
            assert!(
                nexts.contains(expected_contained_state),
                "{:?} does not contains {:?}",
                nexts,
                expected_contained_state
            );
            buildable_robots
        }

        fn step_by_step(blueprint_index: usize, duration: TimeAmount, steps: Vec<State>) {
            let blueprints = parse(&["aoc", "2022", "19-test.txt"]);
            let blueprint = blueprints.get(blueprint_index).unwrap();

            let mut filter = StatesFilter {
                minimum_geodes: 0,
                total_duration: duration,
            };

            let mut state = State::new();
            let mut buildable_robots = None;

            for expected_next_state in steps {
                buildable_robots = Some(test_step(
                    blueprint,
                    &mut filter,
                    buildable_robots.as_ref(),
                    &state,
                    duration,
                    &expected_next_state,
                ));
                state = expected_next_state;
            }
        }

        #[test]
        fn part1_bp1() {
            step_by_step(
                0,
                24,
                vec![
                    State {
                        passed_time: 1,
                        last_move_idle: true,
                        robots: MatterRelatedResources::new(1, 0, 0, 0),
                        resources: MatterRelatedResources::new(1, 0, 0, 0),
                    },
                    State {
                        passed_time: 2,
                        last_move_idle: true,
                        robots: MatterRelatedResources::new(1, 0, 0, 0),
                        resources: MatterRelatedResources::new(2, 0, 0, 0),
                    },
                    State {
                        passed_time: 3,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(1, 1, 0, 0),
                        resources: MatterRelatedResources::new(1, 0, 0, 0),
                    },
                    State {
                        passed_time: 4,
                        last_move_idle: true,
                        robots: MatterRelatedResources::new(1, 1, 0, 0),
                        resources: MatterRelatedResources::new(2, 1, 0, 0),
                    },
                    State {
                        passed_time: 5,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(1, 2, 0, 0),
                        resources: MatterRelatedResources::new(1, 2, 0, 0),
                    },
                    State {
                        passed_time: 6,
                        last_move_idle: true,
                        robots: MatterRelatedResources::new(1, 2, 0, 0),
                        resources: MatterRelatedResources::new(2, 4, 0, 0),
                    },
                    State {
                        passed_time: 7,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(1, 3, 0, 0),
                        resources: MatterRelatedResources::new(1, 6, 0, 0),
                    },
                    State {
                        passed_time: 8,
                        last_move_idle: true,
                        robots: MatterRelatedResources::new(1, 3, 0, 0),
                        resources: MatterRelatedResources::new(2, 9, 0, 0),
                    },
                    State {
                        passed_time: 9,
                        last_move_idle: true,
                        robots: MatterRelatedResources::new(1, 3, 0, 0),
                        resources: MatterRelatedResources::new(3, 12, 0, 0),
                    },
                    State {
                        passed_time: 10,
                        last_move_idle: true,
                        robots: MatterRelatedResources::new(1, 3, 0, 0),
                        resources: MatterRelatedResources::new(4, 15, 0, 0),
                    },
                    State {
                        passed_time: 11,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(1, 3, 1, 0),
                        resources: MatterRelatedResources::new(2, 4, 0, 0),
                    },
                    State {
                        passed_time: 12,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(1, 4, 1, 0),
                        resources: MatterRelatedResources::new(1, 7, 1, 0),
                    },
                    State {
                        passed_time: 13,
                        last_move_idle: true,
                        robots: MatterRelatedResources::new(1, 4, 1, 0),
                        resources: MatterRelatedResources::new(2, 11, 2, 0),
                    },
                    State {
                        passed_time: 14,
                        last_move_idle: true,
                        robots: MatterRelatedResources::new(1, 4, 1, 0),
                        resources: MatterRelatedResources::new(3, 15, 3, 0),
                    },
                    State {
                        passed_time: 15,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(1, 4, 2, 0),
                        resources: MatterRelatedResources::new(1, 5, 4, 0),
                    },
                    State {
                        passed_time: 16,
                        last_move_idle: true,
                        robots: MatterRelatedResources::new(1, 4, 2, 0),
                        resources: MatterRelatedResources::new(2, 9, 6, 0),
                    },
                    State {
                        passed_time: 17,
                        last_move_idle: true,
                        robots: MatterRelatedResources::new(1, 4, 2, 0),
                        resources: MatterRelatedResources::new(3, 13, 8, 0),
                    },
                    State {
                        passed_time: 18,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(1, 4, 2, 1),
                        resources: MatterRelatedResources::new(2, 17, 3, 0),
                    },
                ],
            )
        }

        #[test]
        fn part2_bp1() {
            step_by_step(
                0,
                32,
                vec![
                    State {
                        passed_time: 1,
                        last_move_idle: true,
                        robots: MatterRelatedResources::new(1, 0, 0, 0),
                        resources: MatterRelatedResources::new(1, 0, 0, 0),
                    },
                    State {
                        passed_time: 2,
                        last_move_idle: true,
                        robots: MatterRelatedResources::new(1, 0, 0, 0),
                        resources: MatterRelatedResources::new(2, 0, 0, 0),
                    },
                    State {
                        passed_time: 3,
                        last_move_idle: true,
                        robots: MatterRelatedResources::new(1, 0, 0, 0),
                        resources: MatterRelatedResources::new(3, 0, 0, 0),
                    },
                    State {
                        passed_time: 4,
                        last_move_idle: true,
                        robots: MatterRelatedResources::new(1, 0, 0, 0),
                        resources: MatterRelatedResources::new(4, 0, 0, 0),
                    },
                    State {
                        passed_time: 5,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(2, 0, 0, 0),
                        resources: MatterRelatedResources::new(1, 0, 0, 0),
                    },
                    State {
                        passed_time: 6,
                        last_move_idle: true,
                        robots: MatterRelatedResources::new(2, 0, 0, 0),
                        resources: MatterRelatedResources::new(3, 0, 0, 0),
                    },
                    State {
                        passed_time: 7,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(2, 1, 0, 0),
                        resources: MatterRelatedResources::new(3, 0, 0, 0),
                    },
                    State {
                        passed_time: 8,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(2, 2, 0, 0),
                        resources: MatterRelatedResources::new(3, 1, 0, 0),
                    },
                    State {
                        passed_time: 9,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(2, 3, 0, 0),
                        resources: MatterRelatedResources::new(3, 3, 0, 0),
                    },
                    State {
                        passed_time: 10,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(2, 4, 0, 0),
                        resources: MatterRelatedResources::new(3, 6, 0, 0),
                    },
                    State {
                        passed_time: 11,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(2, 5, 0, 0),
                        resources: MatterRelatedResources::new(3, 10, 0, 0),
                    },
                    State {
                        passed_time: 12,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(2, 6, 0, 0),
                        resources: MatterRelatedResources::new(3, 15, 0, 0),
                    },
                    State {
                        passed_time: 13,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(2, 7, 0, 0),
                        resources: MatterRelatedResources::new(3, 21, 0, 0),
                    },
                    State {
                        passed_time: 14,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(2, 7, 1, 0),
                        resources: MatterRelatedResources::new(2, 14, 0, 0),
                    },
                    State {
                        passed_time: 15,
                        last_move_idle: true,
                        robots: MatterRelatedResources::new(2, 7, 1, 0),
                        resources: MatterRelatedResources::new(4, 21, 1, 0),
                    },
                    State {
                        passed_time: 16,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(2, 7, 2, 0),
                        resources: MatterRelatedResources::new(3, 14, 2, 0),
                    },
                    State {
                        passed_time: 17,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(2, 7, 3, 0),
                        resources: MatterRelatedResources::new(2, 7, 4, 0),
                    },
                    State {
                        passed_time: 18,
                        last_move_idle: true,
                        robots: MatterRelatedResources::new(2, 7, 3, 0),
                        resources: MatterRelatedResources::new(4, 14, 7, 0),
                    },
                    State {
                        passed_time: 19,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(2, 7, 4, 0),
                        resources: MatterRelatedResources::new(3, 7, 10, 0),
                    },
                    State {
                        passed_time: 20,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(2, 7, 4, 1),
                        resources: MatterRelatedResources::new(3, 14, 7, 0),
                    },
                    State {
                        passed_time: 21,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(2, 7, 5, 1),
                        resources: MatterRelatedResources::new(2, 7, 11, 1),
                    },
                    State {
                        passed_time: 22,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(2, 7, 5, 2),
                        resources: MatterRelatedResources::new(2, 14, 9, 2),
                    },
                    State {
                        passed_time: 23,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(2, 7, 5, 3),
                        resources: MatterRelatedResources::new(2, 21, 7, 4),
                    },
                    State {
                        passed_time: 24,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(2, 7, 5, 4),
                        resources: MatterRelatedResources::new(2, 28, 5, 7),
                    },
                    State {
                        passed_time: 25,
                        last_move_idle: true,
                        robots: MatterRelatedResources::new(2, 7, 5, 4),
                        resources: MatterRelatedResources::new(4, 35, 10, 11),
                    },
                    State {
                        passed_time: 26,
                        last_move_idle: false,
                        robots: MatterRelatedResources::new(2, 7, 5, 5),
                        resources: MatterRelatedResources::new(4, 42, 8, 15),
                    },
                    /*
                    == Minute 27 ==
                    Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
                    2 ore-collecting robots collect 2 ore; you now have 4 ore.
                    7 clay-collecting robots collect 7 clay; you now have 49 clay.
                    5 obsidian-collecting robots collect 5 obsidian; you now have 6 obsidian.
                    5 geode-cracking robots crack 5 geodes; you now have 20 open geodes.
                    The new geode-cracking robot is ready; you now have 6 of them.

                    == Minute 28 ==
                    2 ore-collecting robots collect 2 ore; you now have 6 ore.
                    7 clay-collecting robots collect 7 clay; you now have 56 clay.
                    5 obsidian-collecting robots collect 5 obsidian; you now have 11 obsidian.
                    6 geode-cracking robots crack 6 geodes; you now have 26 open geodes.

                    == Minute 29 ==
                    Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
                    2 ore-collecting robots collect 2 ore; you now have 6 ore.
                    7 clay-collecting robots collect 7 clay; you now have 63 clay.
                    5 obsidian-collecting robots collect 5 obsidian; you now have 9 obsidian.
                    6 geode-cracking robots crack 6 geodes; you now have 32 open geodes.
                    The new geode-cracking robot is ready; you now have 7 of them.

                    == Minute 30 ==
                    Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
                    2 ore-collecting robots collect 2 ore; you now have 6 ore.
                    7 clay-collecting robots collect 7 clay; you now have 70 clay.
                    5 obsidian-collecting robots collect 5 obsidian; you now have 7 obsidian.
                    7 geode-cracking robots crack 7 geodes; you now have 39 open geodes.
                    The new geode-cracking robot is ready; you now have 8 of them.

                    == Minute 31 ==
                    Spend 2 ore and 7 obsidian to start building a geode-cracking robot.
                    2 ore-collecting robots collect 2 ore; you now have 6 ore.
                    7 clay-collecting robots collect 7 clay; you now have 77 clay.
                    5 obsidian-collecting robots collect 5 obsidian; you now have 5 obsidian.
                    8 geode-cracking robots crack 8 geodes; you now have 47 open geodes.
                    The new geode-cracking robot is ready; you now have 9 of them.

                    == Minute 32 ==
                    2 ore-collecting robots collect 2 ore; you now have 8 ore.
                    7 clay-collecting robots collect 7 clay; you now have 84 clay.
                    5 obsidian-collecting robots collect 5 obsidian; you now have 10 obsidian.
                    9 geode-cracking robots crack 9 geodes; you now have 56 open geodes.
                                         */
                ],
            )
        }
    }

    #[test]
    #[ignore]
    fn given_test_part1() {
        let blueprints = parse(&["aoc", "2022", "19-test.txt"]);
        let result = score_after(&blueprints, 24);
        assert_eq!(result, 33)
    }

    #[test]
    fn given_test_part2_bp1() {
        let blueprints = parse(&["aoc", "2022", "19-test.txt"]);
        let blueprint = blueprints.get(0).unwrap();
        assert_eq!(blueprint.best_geodes_count_after(32), 56);
    }

    #[test]
    fn given_test_part2_bp2() {
        let blueprints = parse(&["aoc", "2022", "19-test.txt"]);
        let blueprint = blueprints.get(1).unwrap();
        assert_eq!(blueprint.best_geodes_count_after(32), 62);
    }
}
