advent_of_code::solution!(18);

use std::collections::{HashSet, VecDeque};

type Points = HashSet<(i32, i32, i32)>;

pub fn parse_input(input: &str) -> Points {
    let mut p = Points::new();
    input.lines().for_each(|l| {
        let nums: Vec<i32> = l.split(',').map(|n| n.parse::<i32>().unwrap()).collect();
        p.insert((nums[0], nums[1], nums[2]));
    });
    p
}
pub fn calculate_exposed_area(points: &Points) -> u32 {
    let mut count = 0;
    for (x, y, z) in points {
        for i in [-1, 1] {
            if !points.contains(&(x + i, *y, *z)) {
                count += 1
            }
            if !points.contains(&(*x, y + i, *z)) {
                count += 1
            }
            if !points.contains(&(*x, *y, z + i)) {
                count += 1
            }
        }
    }
    count
}

pub fn calculate_exposed_area_sans_inside(points: &Points) -> u32 {
    let mut air_points_queue: VecDeque<(i32, i32, i32)> = VecDeque::new();
    let max_x = points.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0 + 1;
    let max_y = points.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1 + 1;
    let max_z = points.iter().max_by(|a, b| a.2.cmp(&b.2)).unwrap().2 + 1;
    air_points_queue.push_back((-1, -1, -1));
    let mut count = 0;
    let mut visited = Points::new();
    while let Some((x, y, z)) = air_points_queue.pop_front() {
        visited.insert((x, y, z));
        for i in [-1, 1] {
            if points.contains(&(x + i, y, z)) {
                count += 1;
            } else if (0..=max_x).contains(&(x + i))
                && !air_points_queue.contains(&(x + i, y, z))
                && !visited.contains(&(x + i, y, z))
            {
                air_points_queue.push_back((x + i, y, z));
            }
            if points.contains(&(x, y + i, z)) {
                count += 1;
            } else if (0..=max_y).contains(&(y + i))
                && !air_points_queue.contains(&(x, y + i, z))
                && !visited.contains(&(x, y + i, z))
            {
                air_points_queue.push_back((x, y + i, z));
            }
            if points.contains(&(x, y, z + i)) {
                count += 1;
            } else if (0..=max_z).contains(&(z + i))
                && !air_points_queue.contains(&(x, y, z + i))
                && !visited.contains(&(x, y, z + i))
            {
                air_points_queue.push_back((x, y, z + i));
            }
        }
    }
    count
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(calculate_exposed_area(&parse_input(input)))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(calculate_exposed_area_sans_inside(&parse_input(input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(58));
    }
}
