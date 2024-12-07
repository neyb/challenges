use anyhow::*;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

type Res = u64;
pub(crate) fn run(_content: &str) -> Result<Res> {
    let mut system: System = _content.parse()?;
    let mut press_button_report = PressButtonReport::new();
    for _ in 0..1000 {
        press_button_report = press_button_report + system.press_button();
    }
    Ok(press_button_report.low_count * press_button_report.high_count)
}

struct System {
    modules: HashMap<String, Module>,
}

impl System {
    fn new(modules: Vec<Module>) -> Self {
        let mut modules_by_name: HashMap<String, Module> = modules
            .clone()
            .into_iter()
            .map(|module| (module.name.clone(), module))
            .collect();

        for module in modules {
            for output in &module.outputs {
                if let Some(output_module) = modules_by_name.get_mut(output) {
                    if let ModuleType::Conjunction(conjunction) = &mut output_module.module_type {
                        conjunction
                            .internal_state
                            .insert(module.name.clone(), Pulse::Low);
                    }
                }
            }
        }

        Self {
            modules: modules_by_name,
        }
    }

    fn get_mut_module(&mut self, name: &str) -> Option<&mut Module> {
        self.modules.get_mut(name)
    }

    fn press_button(&mut self) -> PressButtonReport {
        let mut press_button_result = PressButtonReport::new();

        let mut deq = VecDeque::new();
        deq.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));

        while let Some((module_name_origin, module_name, pulse)) = deq.pop_front() {
            match pulse {
                Pulse::High => press_button_result.high_count += 1,
                Pulse::Low => press_button_result.low_count += 1,
            }

            if let Some((new_pulse, outputs)) = self.send(pulse, &module_name, &module_name_origin)
            {
                for output in outputs {
                    deq.push_back((module_name.clone(), output.clone(), new_pulse));
                }
            }
        }

        press_button_result
    }

    fn send(
        &mut self,
        pulse: Pulse,
        module_name: &str,
        module_name_origin: &str,
    ) -> Option<(Pulse, &Vec<String>)> {
        let module = self.get_mut_module(module_name)?;

        let output_pulse = module.get_output_pulse(pulse, module_name_origin);
        output_pulse.map(|output_pulse| (output_pulse, &module.outputs))
    }
}

struct PressButtonReport {
    low_count: Res,
    high_count: Res,
}

impl PressButtonReport {
    fn new() -> Self {
        Self {
            low_count: 0,
            high_count: 0,
        }
    }
}

impl std::ops::Add for PressButtonReport {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.low_count += rhs.low_count;
        self.high_count += rhs.high_count;
        self
    }
}

type ModuleName = String;
#[derive(Clone)]
struct Module {
    name: ModuleName,
    module_type: ModuleType,
    outputs: Vec<String>,
}

impl Module {
    fn get_output_pulse(&mut self, pulse: Pulse, module_name_origin: &str) -> Option<Pulse> {
        self.module_type.get_output_pulse(pulse, module_name_origin)
    }
}

#[derive(Clone)]
enum ModuleType {
    Broadcast,
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

impl ModuleType {
    fn get_output_pulse(&mut self, pulse: Pulse, module_name_origin: &str) -> Option<Pulse> {
        match self {
            Self::Broadcast => Some(pulse),
            Self::FlipFlop(flip_flop) => flip_flop.get_output_pulse(pulse),
            Self::Conjunction(conjunction) => {
                conjunction.get_output_pulse(pulse, module_name_origin)
            }
        }
    }
}

impl ModuleType {
    fn new_flip_flop() -> Self {
        Self::FlipFlop(FlipFlop {
            state: FlipFlopState::Off,
        })
    }

    fn new_conjunction() -> Self {
        Self::Conjunction(Conjunction {
            internal_state: HashMap::new(),
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Clone)]
struct FlipFlop {
    state: FlipFlopState,
}
impl FlipFlop {
    fn get_output_pulse(&mut self, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::High => None,
            Pulse::Low => match self.state {
                FlipFlopState::On => {
                    self.state = FlipFlopState::Off;
                    Some(Pulse::Low)
                }
                FlipFlopState::Off => {
                    self.state = FlipFlopState::On;
                    Some(Pulse::High)
                }
            },
        }
    }
}

#[derive(Clone)]
enum FlipFlopState {
    On,
    Off,
}

#[derive(Clone)]
struct Conjunction {
    internal_state: HashMap<String, Pulse>, // an vec is prob better
}

impl Conjunction {
    fn get_output_pulse(&mut self, pulse: Pulse, module_name: &str) -> Option<Pulse> {
        self.internal_state.insert(module_name.to_string(), pulse);
        if self
            .internal_state
            .values()
            .all(|pulse| *pulse == Pulse::High)
        {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }
}

impl FromStr for System {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let modules = s.lines().map(|line| line.parse()).try_collect()?;
        Ok(Self::new(modules))
    }
}

impl FromStr for Module {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (module_designation, outputs) = s
            .split(" -> ")
            .collect_tuple()
            .ok_or_else(|| anyhow!("cannot parse module for \"{s}\""))?;

        let outputs = outputs.split(", ").map(|s| s.to_string()).collect();

        let first_char = module_designation.chars().next().ok_or_else(|| {
            anyhow!("cannot parse module for \"{s}\": empty designation {module_designation}")
        })?;

        match first_char {
            '&' => Ok(Self {
                name: module_designation[1..].to_string(),
                module_type: ModuleType::new_conjunction(),
                outputs,
            }),
            '%' => Ok(Self {
                name: module_designation[1..].to_string(),
                module_type: ModuleType::new_flip_flop(),
                outputs,
            }),
            _ if module_designation == "broadcaster" => Ok(Self {
                name: "broadcaster".to_string(),
                module_type: ModuleType::Broadcast,
                outputs,
            }),
            _ => bail!("cannot parse module for \"{s}\": cannot determine module type"),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_test_1() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "20-test_1.txt"]);
        assert_eq!(run(&content).unwrap(), 32000000);
    }

    #[test]
    fn given_test_2() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "20-test_2.txt"]);
        assert_eq!(run(&content).unwrap(), 11687500);
    }

    #[test]
    fn given_test_2_first_button_press() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "20-test_2.txt"]);
        let mut system: System = content.parse().unwrap();
        let press_button_report = system.press_button();
        assert_eq!(press_button_report.low_count, 4);
        assert_eq!(press_button_report.high_count, 4);
    }

    #[test]
    fn given_test_2_second_button_press() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "20-test_2.txt"]);
        let mut system: System = content.parse().unwrap();
        system.press_button();
        let press_button_report = system.press_button();
        assert_eq!(press_button_report.low_count, 4);
        assert_eq!(press_button_report.high_count, 2);
    }

    #[test]
    fn given_test_2_third_button_press() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "20-test_2.txt"]);
        let mut system: System = content.parse().unwrap();
        system.press_button();
        system.press_button();
        let press_button_report = system.press_button();
        assert_eq!(press_button_report.low_count, 5);
        assert_eq!(press_button_report.high_count, 3);
    }
    #[test]
    fn given_test_2_fourth_button_press() {
        let content = challenges_common::get_input_content(&["aoc", "2023", "20-test_2.txt"]);
        let mut system: System = content.parse().unwrap();
        system.press_button();
        system.press_button();
        system.press_button();
        let press_button_report = system.press_button();
        assert_eq!(press_button_report.low_count, 4);
        assert_eq!(press_button_report.high_count, 2);
    }
}
