advent_of_code::solution!(12);

use std::collections::HashMap;

pub fn can_step(current: char, destination: char) -> bool {
    destination <= (current as u8 + 1) as char
}

pub fn can_step2(current: char, destination: char) -> bool {
    current <= (destination as u8 + 1) as char
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut elevation_map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut starting_point: (usize, usize) = (usize::MAX, usize::MAX);
    let mut ending_point: (usize, usize) = (usize::MAX, usize::MAX);
    let mut unvisted: HashMap<(usize, usize), u32> = HashMap::new();
    for x in 0..elevation_map.len() {
        for y in 0..elevation_map[0].len() {
            if elevation_map[x][y] == 'S' {
                starting_point = (x, y);
                elevation_map[x][y] = 'a';
            } else if elevation_map[x][y] == 'E' {
                ending_point = (x, y);
                elevation_map[x][y] = 'z';
                unvisted.insert((x, y), u32::MAX);
            } else {
                unvisted.insert((x, y), u32::MAX);
            }
        }
    }

    let mut current_point = starting_point;
    let mut current_value = 0;
    loop {
        if current_point == ending_point {
            break;
        }
        let mut next_points = vec![];
        if current_point.0 != 0
            && unvisted.contains_key(&(current_point.0 - 1, current_point.1))
            && can_step(
                elevation_map[current_point.0][current_point.1],
                elevation_map[current_point.0 - 1][current_point.1],
            )
        {
            next_points.push((current_point.0 - 1, current_point.1))
        }
        if current_point.0 != elevation_map.len() - 1
            && unvisted.contains_key(&(current_point.0 + 1, current_point.1))
            && can_step(
                elevation_map[current_point.0][current_point.1],
                elevation_map[current_point.0 + 1][current_point.1],
            )
        {
            next_points.push((current_point.0 + 1, current_point.1))
        }
        if current_point.1 != 0
            && unvisted.contains_key(&(current_point.0, current_point.1 - 1))
            && can_step(
                elevation_map[current_point.0][current_point.1],
                elevation_map[current_point.0][current_point.1 - 1],
            )
        {
            next_points.push((current_point.0, current_point.1 - 1))
        }
        if current_point.1 != elevation_map[0].len() - 1
            && unvisted.contains_key(&(current_point.0, current_point.1 + 1))
            && can_step(
                elevation_map[current_point.0][current_point.1],
                elevation_map[current_point.0][current_point.1 + 1],
            )
        {
            next_points.push((current_point.0, current_point.1 + 1))
        }

        for point in next_points {
            unvisted.entry(point).and_modify(|a| {
                if current_value + 1 < *a {
                    *a = current_value + 1
                }
            });
        }

        let min_point = unvisted.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap();
        current_point = *min_point.0;
        current_value = *min_point.1;

        unvisted.remove(&current_point);
    }
    Some(current_value)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elevation_map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut ending_point: (usize, usize) = (usize::MAX, usize::MAX);
    let mut unvisted: HashMap<(usize, usize), u32> = HashMap::new();
    for x in 0..elevation_map.len() {
        for y in 0..elevation_map[0].len() {
            if elevation_map[x][y] == 'S' {
                elevation_map[x][y] = 'a';
                unvisted.insert((x, y), u32::MAX);
            } else if elevation_map[x][y] == 'E' {
                ending_point = (x, y);
                elevation_map[x][y] = 'z';
            } else {
                unvisted.insert((x, y), u32::MAX);
            }
        }
    }

    let mut current_point = ending_point;
    let mut current_value = 0;
    loop {
        if elevation_map[current_point.0][current_point.1] == 'a' {
            break;
        }
        let mut next_points = vec![];
        if current_point.0 != 0
            && unvisted.contains_key(&(current_point.0 - 1, current_point.1))
            && can_step2(
                elevation_map[current_point.0][current_point.1],
                elevation_map[current_point.0 - 1][current_point.1],
            )
        {
            next_points.push((current_point.0 - 1, current_point.1))
        }
        if current_point.0 != elevation_map.len() - 1
            && unvisted.contains_key(&(current_point.0 + 1, current_point.1))
            && can_step2(
                elevation_map[current_point.0][current_point.1],
                elevation_map[current_point.0 + 1][current_point.1],
            )
        {
            next_points.push((current_point.0 + 1, current_point.1))
        }
        if current_point.1 != 0
            && unvisted.contains_key(&(current_point.0, current_point.1 - 1))
            && can_step2(
                elevation_map[current_point.0][current_point.1],
                elevation_map[current_point.0][current_point.1 - 1],
            )
        {
            next_points.push((current_point.0, current_point.1 - 1))
        }
        if current_point.1 != elevation_map[0].len() - 1
            && unvisted.contains_key(&(current_point.0, current_point.1 + 1))
            && can_step2(
                elevation_map[current_point.0][current_point.1],
                elevation_map[current_point.0][current_point.1 + 1],
            )
        {
            next_points.push((current_point.0, current_point.1 + 1))
        }

        for point in next_points {
            unvisted.entry(point).and_modify(|a| {
                if current_value + 1 < *a {
                    *a = current_value + 1
                }
            });
        }

        let min_point = unvisted.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap();
        current_point = *min_point.0;
        current_value = *min_point.1;

        unvisted.remove(&current_point);
    }
    Some(current_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(29));
    }
}
