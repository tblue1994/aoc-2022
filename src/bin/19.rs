use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(19);

pub struct Blueprint {
    ore: u64,
    clay: u64,
    obsidian: (u64, u64),
    geode: (u64, u64),
}

pub fn part_one(input: &str) -> Option<u64> {
    let blueprints = parse(input);
    let mut quality_level_sum = 0;
    for (i, blueprint) in blueprints.iter().enumerate() {
        let mut shortcuts = HashMap::new();
        let geode_num =
            find_max_geodes(blueprint, (0, 0, 0, 0), (1, 0, 0, 0), 0, 24, &mut shortcuts);
        quality_level_sum += geode_num * (i + 1) as u64;
    }
    Some(quality_level_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let blueprints = parse(input);
    let mut quality_level_prod = 1;
    let max = blueprints.len().min(3);
    for blueprint in &blueprints[0..max] {
        let mut shortcuts = HashMap::new();
        let geode_num =
            find_max_geodes(blueprint, (0, 0, 0, 0), (1, 0, 0, 0), 0, 32, &mut shortcuts);
        quality_level_prod *= geode_num;
    }
    Some(quality_level_prod)
}

pub fn parse(input: &str) -> Vec<Blueprint> {
    let mut blueprints = vec![];
    let re = Regex::new(r"Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian").unwrap();
    for line in input.lines() {
        let (_, [ore, clay, o_ore, o_clay, g_ore, g_obsidian]) =
            re.captures_iter(line).map(|c| c.extract()).next().unwrap();
        blueprints.push(Blueprint {
            ore: ore.parse().unwrap(),
            clay: clay.parse().unwrap(),
            obsidian: (o_ore.parse().unwrap(), o_clay.parse().unwrap()),
            geode: (g_ore.parse().unwrap(), g_obsidian.parse().unwrap()),
        })
    }
    blueprints
}

pub fn find_max_geodes(
    blueprint: &Blueprint,
    current_resources: (u64, u64, u64, u64),
    current_robots: (u64, u64, u64, u64),
    time: u32,
    max_time: u32,
    shortcuts: &mut HashMap<((u64, u64, u64, u64), (u64, u64, u64, u64)), u64>,
) -> u64 {
    if time >= max_time {
        return current_resources.3;
    }

    if shortcuts.contains_key(&(current_resources, current_robots)) {
        return *shortcuts.get(&(current_resources, current_robots)).unwrap();
    }

    //determine new robots
    let mut outcomes = vec![];
    let mut bought_high_value = false;

    //geode robot
    if blueprint.geode.0 <= current_resources.0 && blueprint.geode.1 <= current_resources.2 {
        outcomes.push(find_max_geodes(
            blueprint,
            (
                current_resources.0 + current_robots.0 - blueprint.geode.0,
                current_resources.1 + current_robots.1,
                current_resources.2 + current_robots.2 - blueprint.geode.1,
                current_resources.3 + current_robots.3,
            ),
            (
                current_robots.0,
                current_robots.1,
                current_robots.2,
                current_robots.3 + 1,
            ),
            time + 1,
            max_time,
            shortcuts,
        ));
        bought_high_value = true;
    }

    //obsidian robot
    if blueprint.obsidian.0 <= current_resources.0
        && blueprint.obsidian.1 <= current_resources.1
        && current_robots.2 < blueprint.geode.1
    {
        outcomes.push(find_max_geodes(
            blueprint,
            (
                current_resources.0 + current_robots.0 - blueprint.obsidian.0,
                current_resources.1 + current_robots.1 - blueprint.obsidian.1,
                current_resources.2 + current_robots.2,
                current_resources.3 + current_robots.3,
            ),
            (
                current_robots.0,
                current_robots.1,
                current_robots.2 + 1,
                current_robots.3,
            ),
            time + 1,
            max_time,
            shortcuts,
        ));
        bought_high_value = true;
    }

    if !bought_high_value {
        //clay robot
        if blueprint.clay <= current_resources.0 && current_robots.1 < blueprint.obsidian.1 {
            outcomes.push(find_max_geodes(
                blueprint,
                (
                    current_resources.0 + current_robots.0 - blueprint.clay,
                    current_resources.1 + current_robots.1,
                    current_resources.2 + current_robots.2,
                    current_resources.3 + current_robots.3,
                ),
                (
                    current_robots.0,
                    current_robots.1 + 1,
                    current_robots.2,
                    current_robots.3,
                ),
                time + 1,
                max_time,
                shortcuts,
            ))
        }

        //ore robot
        if blueprint.ore <= current_resources.0
            && current_robots.0
                < ((blueprint.obsidian.0.max(blueprint.geode.0)).max(blueprint.clay))
                    .max(blueprint.ore)
        {
            outcomes.push(find_max_geodes(
                blueprint,
                (
                    current_resources.0 + current_robots.0 - blueprint.ore,
                    current_resources.1 + current_robots.1,
                    current_resources.2 + current_robots.2,
                    current_resources.3 + current_robots.3,
                ),
                (
                    current_robots.0 + 1,
                    current_robots.1,
                    current_robots.2,
                    current_robots.3,
                ),
                time + 1,
                max_time,
                shortcuts,
            ))
        }

        //do nothing
        outcomes.push(find_max_geodes(
            blueprint,
            (
                current_resources.0 + current_robots.0,
                current_resources.1 + current_robots.1,
                current_resources.2 + current_robots.2,
                current_resources.3 + current_robots.3,
            ),
            (
                current_robots.0,
                current_robots.1,
                current_robots.2,
                current_robots.3,
            ),
            time + 1,
            max_time,
            shortcuts,
        ));
    }

    let best = *outcomes.iter().max().unwrap();
    shortcuts.insert((current_resources, current_robots), best);
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3348));
    }
}
