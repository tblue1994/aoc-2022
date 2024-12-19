advent_of_code::solution!(15);

use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use regex::Regex;

type SensorList = HashMap<(i128, i128), i128>;
type PositionList = HashSet<(i128, i128)>;

pub fn calculate_distance(a: (i128, i128), b: (i128, i128)) -> i128 {
    i128::abs(a.0 - b.0) + i128::abs(a.1 - b.1)
}

pub fn parse_sensor_list(input: &str) -> (SensorList, PositionList) {
    let mut list = SensorList::new();
    let mut positions = PositionList::new();
    let s_regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    input.lines().for_each(|l| {
        let caps = s_regex.captures(l).unwrap();
        let sensor = (
            caps[1].parse::<i128>().unwrap(),
            caps[2].parse::<i128>().unwrap(),
        );
        let beacon = (
            caps[3].parse::<i128>().unwrap(),
            caps[4].parse::<i128>().unwrap(),
        );
        positions.insert(sensor);
        positions.insert(beacon);
        list.insert(sensor, calculate_distance(sensor, beacon));
    });

    (list, positions)
}

pub fn get_range_for_row(
    point: (i128, i128),
    distance: i128,
    row: i128,
    min_total: Option<i128>,
    max_total: Option<i128>,
) -> Option<RangeInclusive<i128>> {
    if ((point.1 - distance)..=(point.1 + distance)).contains(&row) {
        let min_x = (point.0 - distance) + i128::abs(point.1 - row);
        let max_x = (point.0 + distance) - i128::abs(point.1 - row);
        Some(max(min_x, min_total.unwrap_or(min_x))..=min(max_x, max_total.unwrap_or(max_x)))
    } else {
        None
    }
}

pub fn is_inside((x, y): (i128, i128), range: &RangeInclusive<i128>, row: i128) -> bool {
    range.contains(&x) && y == row
}

pub fn range_for_row(
    sensors: &SensorList,
    row: i128,
    min_x: Option<i128>,
    max_x: Option<i128>,
) -> Vec<RangeInclusive<i128>> {
    let mut ranges: Vec<RangeInclusive<i128>> = sensors
        .iter()
        .filter_map(|(s, d)| get_range_for_row(*s, *d, row, min_x, max_x))
        .collect();
    ranges.sort_by(|a, b| a.start().cmp(b.start()));
    let mut total_ranges: Vec<RangeInclusive<i128>> = vec![];
    let mut current_range = *ranges[0].start()..=*ranges[0].end();
    for range in ranges {
        if current_range.contains(range.start()) {
            current_range = *current_range.start()..=*max(current_range.end(), range.end())
        } else {
            total_ranges.push(current_range);
            current_range = range;
            break;
        }
    }
    total_ranges.push(current_range);
    total_ranges
}

pub fn part_one(input: &str) -> Option<usize> {
    let (sensors, positions) = parse_sensor_list(input);
    let row = if sensors.len() == 14 { 10 } else { 2000000 };

    let total_ranges = range_for_row(&sensors, row, None, None);
    if total_ranges.len() != 1 {
        panic!("something is wrong")
    }
    let total_range = *total_ranges[0].start()..=*(total_ranges[0].end());
    let r = *total_ranges[0].start()..=*(total_ranges[0].end());
    let objs = positions.iter().filter(|p| is_inside(**p, &r, row));

    Some(total_range.count() - objs.count())
}

pub fn part_two(input: &str) -> Option<u128> {
    let (sensors, _) = parse_sensor_list(input);
    let min = 0;
    let max = if sensors.len() == 14 { 20 } else { 4000000 };

    for y in min..=max {
        let total_ranges = range_for_row(&sensors, y, Some(min), Some(max));
        if total_ranges.len() == 2 {
            let missing = total_ranges[0].end() + 1;
            return Some((missing * 4000000 + y) as u128);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
