advent_of_code::solution!(5);

use regex::Regex;

pub fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let lines: Vec<&str> = input.lines().collect();
    let mut container_stacks: Vec<Vec<char>> = vec![];
    let mut instructions: Vec<(usize, usize, usize)> = vec![];
    for i in 0..lines.len() {
        if lines[i].contains("move") {
            let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
            let caps = re.captures(lines[i]).unwrap();

            let number = caps
                .get(1)
                .map_or(0, |m| m.as_str().parse::<usize>().unwrap());
            let from = caps
                .get(2)
                .map_or(0, |m| m.as_str().parse::<usize>().unwrap());
            let to = caps
                .get(3)
                .map_or(0, |m| m.as_str().parse::<usize>().unwrap());
            instructions.push((number, from, to))
        } else if lines[i].contains('1') {
            // doing this part second since some instructions will contain "1"
            for j in 0..lines[i].len() {
                if lines[i].chars().nth(j).unwrap().is_numeric() {
                    let mut new_container: Vec<char> = vec![];
                    for k in (0..i).rev() {
                        if lines[k].chars().nth(j).unwrap().is_alphabetic() {
                            new_container.push(lines[k].chars().nth(j).unwrap())
                        }
                    }
                    container_stacks.push(new_container)
                }
            }
        }
    }
    (container_stacks, instructions)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut container_stacks, instructions) = parse_input(input);

    for (times, from, to) in instructions {
        for _ in 0..times {
            let container = container_stacks[from - 1].pop();
            container_stacks[to - 1].push(container.unwrap());
        }
    }

    let top_values: String = container_stacks.iter().map(|s| s.last().unwrap()).collect();
    Some(top_values)
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut container_stacks, instructions) = parse_input(input);

    for (times, from, to) in instructions {
        let at = container_stacks[from - 1].len() - times;
        let mut container_stack = container_stacks[from - 1].split_off(at);
        container_stacks[to - 1].append(&mut container_stack);
    }

    let top_values: String = container_stacks.iter().map(|s| s.last().unwrap()).collect();
    Some(top_values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
