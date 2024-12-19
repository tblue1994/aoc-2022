advent_of_code::solution!(9);
use std::collections::HashSet;

pub fn move_rope(input: &str, rope_len: i32) -> Option<usize> {
    let movements: Vec<(&str, i32)> = input
        .lines()
        .map(|s| s.split(' ').collect::<Vec<&str>>())
        .map(|a| (a[0], a[1].parse::<i32>().unwrap()))
        .collect();
    let mut hash_set: HashSet<(i32, i32)> = HashSet::new();
    let mut rope: Vec<(i32, i32)> = vec![];
    for i in 0..rope_len {
        rope.push((0, 0));
    }
    for movement in movements {
        for _ in 0..movement.1 {
            match movement.0 {
                "U" => rope[0].1 += 1,
                "D" => rope[0].1 -= 1,
                "R" => rope[0].0 += 1,
                _ => rope[0].0 -= 1,
            }

            for i in 1..rope.len() {
                if i32::abs(rope[i - 1].0 - rope[i].0) == 2 {
                    if i32::abs(rope[i - 1].1 - rope[i].1) > 0 {
                        if rope[i - 1].1 > rope[i].1 {
                            rope[i].1 += 1
                        } else {
                            rope[i].1 -= 1
                        }
                    }
                    //move horizontally toward the rope[i-1]
                    if rope[i - 1].0 > rope[i].0 {
                        rope[i].0 += 1
                    } else {
                        rope[i].0 -= 1
                    }
                } else if i32::abs(rope[i - 1].1 - rope[i].1) == 2 {
                    if i32::abs(rope[i - 1].0 - rope[i].0) > 0 {
                        if rope[i - 1].0 > rope[i].0 {
                            rope[i].0 += 1
                        } else {
                            rope[i].0 -= 1
                        }
                    }
                    //move veritallcy toward the rope[i-1]
                    if rope[i - 1].1 > rope[i].1 {
                        rope[i].1 += 1
                    } else {
                        rope[i].1 -= 1
                    }
                }
            }

            hash_set.insert(*rope.last().unwrap());
        }
    }
    Some(hash_set.len())
}

pub fn part_one(input: &str) -> Option<usize> {
    move_rope(input, 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    move_rope(input, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(36));
    }
}
