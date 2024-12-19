advent_of_code::solution!(11);

use std::collections::VecDeque;

use regex::Regex;

#[derive(Debug)]
pub struct Monkey {
    items: VecDeque<u64>,
    operation_sign: char,
    operation_num: Option<u64>,
    test_num: u64,
    true_monkey: usize,
    false_monkey: usize,
    items_handled: u64,
    divide: bool,
}

impl Monkey {
    fn new(
        items: Vec<u64>,
        operation_sign: char,
        operation_num: Option<u64>,
        test_num: u64,
        true_monkey: usize,
        false_monkey: usize,
        divide: bool,
    ) -> Self {
        Self {
            items: VecDeque::from(items),
            operation_sign,
            operation_num,
            test_num,
            true_monkey,
            false_monkey,
            items_handled: 0,
            divide,
        }
    }
    fn inspect(&self, item: u64) -> u64 {
        let value_mod = if self.operation_num.is_some() {
            self.operation_num.unwrap()
        } else {
            item
        };

        let max_anxiety = if self.operation_sign == '+' {
            item + value_mod
        } else {
            item * value_mod
        };

        if self.divide {
            max_anxiety / 3
        } else {
            max_anxiety
        }
    }

    fn throw(&self, item: u64) -> usize {
        if item % self.test_num == 0 {
            self.true_monkey
        } else {
            self.false_monkey
        }
    }
}

pub fn parse_monkeys(input: &str, divide: bool) -> Vec<Monkey> {
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;
    let si_regex = Regex::new(r"Starting items: (.*)").unwrap();
    let op_regex = Regex::new(r"Operation: new = old (\+|\*) (\d+|old)").unwrap();
    let test_regex = Regex::new(r"Test: divisible by (\d+)").unwrap();
    let true_regex = Regex::new(r"If true: throw to monkey (\d+)").unwrap();
    let false_regex = Regex::new(r"If false: throw to monkey (\d+)").unwrap();
    let mut monkies: Vec<Monkey> = vec![];
    while i < lines.len() {
        let si_cap = si_regex.captures(lines[i + 1]).unwrap();
        let op_cap = op_regex.captures(lines[i + 2]).unwrap();
        let test_cap = test_regex.captures(lines[i + 3]).unwrap();
        let true_cap = true_regex.captures(lines[i + 4]).unwrap();
        let false_cap = false_regex.captures(lines[i + 5]).unwrap();
        monkies.push(Monkey::new(
            si_cap[1]
                .split(", ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
            op_cap[1].chars().next().unwrap(),
            op_cap[2].parse::<u64>().ok(),
            test_cap[1].parse::<u64>().unwrap(),
            true_cap[1].parse::<usize>().unwrap(),
            false_cap[1].parse::<usize>().unwrap(),
            divide,
        ));
        i += 7
    }

    monkies
}

fn monkey_around(mut monkies: Vec<Monkey>, rounds: u64) -> Option<u64> {
    let cycle_length = monkies.iter().fold(1, |a, m| a * m.test_num);
    for _ in 0..rounds {
        for i in 0..monkies.len() {
            while !monkies[i].items.is_empty() {
                let mut item = monkies[i].items.pop_front().unwrap();
                item = monkies[i].inspect(item) % cycle_length;
                let monkey_index = monkies[i].throw(item);
                monkies[monkey_index].items.push_back(item);
                monkies[i].items_handled += 1
            }
        }
    }

    monkies.sort_by(|a, b| b.items_handled.cmp(&a.items_handled));

    Some(monkies[0].items_handled * monkies[1].items_handled)
}

pub fn part_one(input: &str) -> Option<u64> {
    let monkies = parse_monkeys(input, true);
    monkey_around(monkies, 20)
}

pub fn part_two(input: &str) -> Option<u64> {
    let monkies = parse_monkeys(input, false);
    monkey_around(monkies, 10000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
