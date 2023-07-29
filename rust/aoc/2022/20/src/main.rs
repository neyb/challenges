use std::iter;

fn main() {
    let numbers = parse(&["aoc", "2022", "20.txt"]);
    println!("part1 : {}", part1(&numbers));
    println!("part2 : {}", part2(&numbers));
}

type Value = i64;

fn part1(numbers: &[Value]) -> Value {
    let mut indexes = Numbers::from(numbers);
    indexes.switch_all();
    let origin = indexes.index_of(0).unwrap();
    indexes.get(origin + 1000) + indexes.get(origin + 2000) + indexes.get(origin + 3000)
}

fn part2(numbers: &[Value]) -> Value {
    let mut indexes = Numbers::from(
        numbers
            .iter()
            .map(|n| n * 811589153)
            .collect::<Vec<_>>()
            .as_ref(),
    );

    (0..10).for_each(|_| indexes.switch_all());

    let origin = indexes.index_of(0).unwrap();
    indexes.get(origin + 1000) + indexes.get(origin + 2000) + indexes.get(origin + 3000)
}

#[derive(PartialEq, Debug)]
struct Numbers {
    values: Vec<Number>,
}

impl From<&[Value]> for Numbers {
    fn from(numbers: &[Value]) -> Self {
        let indexes = numbers
            .iter()
            .enumerate()
            .map(|(index, &value)| Number { value, index })
            .collect::<Vec<_>>();
        Self { values: indexes }
    }
}

impl Numbers {
    fn switch_all(&mut self) {
        for i in 0..self.values.len() {
            self.switch(i)
        }
    }

    fn switch(&mut self, position_index: usize) {
        if self.len() > 1 {
            let len = self.values.len();
            let index = self.get_mut_by_origin_index(position_index);
            let old_index = index.index;
            let modulo = len as Value - 1;
            let rem = (index.index as Value + index.value).rem_euclid(modulo);
            index.index = if rem == 0 { modulo } else { rem } as usize;
            let new_index = index.index;

            for (position, number) in self.values.iter_mut().enumerate() {
                if position != position_index {
                    if (old_index..(new_index + 1)).contains(&number.index) {
                        number.index -= 1;
                    }
                    if (new_index..old_index).contains(&number.index) {
                        number.index += 1
                    }
                }
            }
        }
    }

    fn get_mut_by_origin_index(&mut self, index: usize) -> &mut Number {
        let len = self.len();
        self.values.get_mut(index % len).unwrap()
    }

    fn index_of(&self, value: Value) -> Option<usize> {
        self.iter().find(|n| n.value == value).map(|n| n.index)
    }

    fn get(&self, index: usize) -> &Value {
        let index = index % self.len();
        &self
            .values
            .iter()
            .find(|curr_index| curr_index.index == index)
            .unwrap()
            .value
    }

    fn iter(&self) -> impl Iterator<Item = &Number> {
        self.values.iter()
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    #[allow(dead_code)]
    fn as_vec(&self) -> Vec<Value> {
        let mut result = iter::repeat(None)
            .take(self.len())
            .collect::<Vec<Option<Value>>>();

        self.values.iter().for_each(|number| {
            let _ = std::mem::replace(&mut result[number.index], Some(number.value));
        });

        result.into_iter().map(|n| n.unwrap()).collect()
    }
}

#[derive(PartialEq, Debug)]
struct Number {
    value: Value,
    index: usize,
}

fn parse(path: &[&str]) -> Vec<Value> {
    challenges_common::get_input_lines(path)
        .map(|line| line.parse::<Value>().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::parse;

    #[test]
    fn given_test_can_be_parsed() {
        let parsed = parse(&["aoc", "2022", "20-test.txt"]);
        assert_eq!(parsed, vec![1, 2, -3, 3, -2, 0, 4])
    }

    mod part1 {
        use crate::*;

        macro_rules! numbers {
            ($($n:expr),*) => {Numbers::from(&[$($n as Value,)*] as &[Value])}
        }

        #[test]
        fn simple_right_switch() {
            let mut numbers = numbers![1, 2, 3];
            numbers.switch(0);
            assert_eq!(numbers.as_vec(), vec![2, 1, 3]);
        }

        #[test]
        fn left_switch() {
            let mut numbers = numbers![10, 20, -1];
            numbers.switch(2);
            assert_eq!(numbers.as_vec(), vec![10, -1, 20]);
        }

        #[test]
        fn left_to_the_end() {
            let mut numbers = numbers![10, -1, 20];
            numbers.switch(1);
            assert_eq!(numbers.as_vec(), vec![10, 20, -1]);
        }

        #[test]
        fn simple_positive_jump_left_switch() {
            let mut numbers = numbers![10, 30, 1];
            numbers.switch(2);
            assert_eq!(numbers.as_vec(), vec![10, 1, 30]);
        }

        #[test]
        fn simple_positive_jump_right_switch() {
            let mut numbers = numbers![5, 10, 20];
            numbers.switch(0);
            assert_eq!(numbers.as_vec(), vec![10, 5, 20]);
        }

        #[test]
        fn given_test_can_first_switch() {
            let numbers = parse(&["aoc", "2022", "20-test.txt"]);
            let mut numbers = Numbers::from(numbers.as_ref());
            assert_eq!(
                numbers,
                Numbers {
                    values: vec![
                        Number { value: 1, index: 0 },
                        Number { value: 2, index: 1 },
                        Number {
                            value: -3,
                            index: 2,
                        },
                        Number { value: 3, index: 3 },
                        Number {
                            value: -2,
                            index: 4,
                        },
                        Number { value: 0, index: 5 },
                        Number { value: 4, index: 6 },
                    ]
                }
            );
            numbers.switch(0);
            assert_eq!(
                numbers,
                Numbers {
                    values: vec![
                        Number { value: 1, index: 1 },
                        Number { value: 2, index: 0 },
                        Number {
                            value: -3,
                            index: 2,
                        },
                        Number { value: 3, index: 3 },
                        Number {
                            value: -2,
                            index: 4,
                        },
                        Number { value: 0, index: 5 },
                        Number { value: 4, index: 6 },
                    ]
                }
            );
        }

        #[test]
        fn given_test_step_by_step() {
            let numbers = parse(&["aoc", "2022", "20-test.txt"]);
            let mut numbers = Numbers::from(numbers.as_ref());

            assert_eq!(numbers.as_vec(), vec![1, 2, -3, 3, -2, 0, 4]);
            numbers.switch(0);
            assert_eq!(numbers.as_vec(), vec![2, 1, -3, 3, -2, 0, 4]);
            numbers.switch(1);
            assert_eq!(numbers.as_vec(), vec![1, -3, 2, 3, -2, 0, 4]);
            numbers.switch(2);
            assert_eq!(numbers.as_vec(), vec![1, 2, 3, -2, -3, 0, 4]);
            numbers.switch(3);
            assert_eq!(numbers.as_vec(), vec![1, 2, -2, -3, 0, 3, 4]);
            numbers.switch(4);
            assert_eq!(numbers.as_vec(), vec![1, 2, -3, 0, 3, 4, -2]);
            numbers.switch(5);
            assert_eq!(numbers.as_vec(), vec![1, 2, -3, 0, 3, 4, -2]);
            numbers.switch(6);
            assert_eq!(numbers.as_vec(), vec![1, 2, -3, 4, 0, 3, -2]);

            let origin = numbers.index_of(0).unwrap();
            let n1000 = numbers.get(origin + 1000);
            let n2000 = numbers.get(origin + 2000);
            let n3000 = numbers.get(origin + 3000);
            assert_eq!(n1000, &(4 as Value));
            assert_eq!(n2000, &(-3 as Value));
            assert_eq!(n3000, &(2 as Value));
        }

        #[test]
        fn given_test() {
            let numbers = parse(&["aoc", "2022", "20-test.txt"]);
            assert_eq!(part1(&numbers), 3)
        }
    }
}
