use std::{
    collections::{HashMap, HashSet},
    u64,
};

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<u64> {
    let mut blizzards = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let dir = match c {
                'v' => Some((0, 1)),
                '<' => Some((-1, 0)),
                '>' => Some((1, 0)),
                '^' => Some((0, -1)),
                _ => None,
            };

            if dir.is_some() {
                blizzards.push(((x, y), dir.unwrap()));
            }
        }
    }
    let max_x = input.lines().next().unwrap().len();
    let max_y = input.lines().count();
    let end = vec![(max_x - 2, max_y - 1)];

    Some(find_quickest_path((1, 0), &end, (max_x, max_y), &blizzards))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut blizzards = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let dir = match c {
                'v' => Some((0, 1)),
                '<' => Some((-1, 0)),
                '>' => Some((1, 0)),
                '^' => Some((0, -1)),
                _ => None,
            };

            if dir.is_some() {
                blizzards.push(((x, y), dir.unwrap()));
            }
        }
    }
    let max_x = input.lines().next().unwrap().len();
    let max_y = input.lines().count();
    let end = vec![(max_x - 2, max_y - 1), (1, 0), (max_x - 2, max_y - 1)];

    Some(find_quickest_path((1, 0), &end, (max_x, max_y), &blizzards))
}

pub fn find_quickest_path(
    position: (usize, usize),
    end: &[(usize, usize)],
    limits: (usize, usize),
    blizzards: &Vec<((usize, usize), (i32, i32))>,
) -> u64 {
    let mut queue = vec![position];
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0), (0, 0)];
    let mut goal_index = 0;
    let mut seconds = 1;
    while !queue.is_empty() {
        if queue.contains(&end[goal_index]) {
            queue = vec![end[goal_index]];
            goal_index += 1;
            if goal_index >= end.len() {
                return seconds - 1;
            }
        }
        let blizzards_at_time = calculate_blizzards(blizzards, seconds, limits);
        let mut new_queue = vec![];
        for pos in queue {
            for dir in &directions {
                let new_pos = (
                    pos.0.checked_add_signed(dir.0),
                    pos.1.checked_add_signed(dir.1),
                );
                if ((new_pos.0.is_some_and(|x| x > 0 && x < limits.0 - 1)
                    && new_pos.1.is_some_and(|x| x > 0 && x < limits.1 - 1))
                    || (new_pos.0.is_some()
                        && new_pos.1.is_some()
                        && end.contains(&(new_pos.0.unwrap(), new_pos.1.unwrap()))))
                    && !blizzards_at_time.contains(&(new_pos.0.unwrap(), new_pos.1.unwrap()))
                {
                    new_queue.push((new_pos.0.unwrap(), new_pos.1.unwrap()));
                }
            }
        }
        new_queue.sort();
        new_queue.dedup();
        queue = new_queue;
        seconds += 1;
    }
    0
}

pub fn calculate_blizzards(
    blizzards: &Vec<((usize, usize), (i32, i32))>,
    seconds: u64,
    limits: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut b = vec![];
    for bliz in blizzards {
        let (start, dir) = bliz;
        let new_position = (
            ((start.0 as i32 - 1 + (dir.0 * (seconds as i32 % (limits.0 as i32 - 2))) as i32)
                .rem_euclid(limits.0 as i32 - 2)
                + 1) as usize,
            ((start.1 as i32 - 1 + (dir.1 * (seconds as i32 % (limits.1 as i32 - 2))) as i32)
                .rem_euclid(limits.1 as i32 - 2)
                + 1) as usize,
        );
        b.push(new_position);
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }
}
