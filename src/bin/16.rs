use std::{
    cmp::min,
    collections::{HashMap, HashSet},
};

use regex::{Captures, Regex};

type DistanceGraph = HashMap<u8, HashMap<u8, i32>>;

#[derive(Debug, Clone)]
pub struct Node {
    name: u8,
    value: i32,
    neighbors: Vec<u8>,
}

pub fn parse_nodes(input: &str) -> (Vec<Node>, HashMap<String, u8>) {
    let l_regex =
        Regex::new(r"Valve (.+) has flow rate=(.+); tunnels* leads* to valves* (.+)").unwrap();
    let mut xx_to_id = HashMap::<String, u8>::new();
    let mut lines = input
        .lines()
        .map(|l| l_regex.captures(l).unwrap())
        .collect::<Vec<Captures>>();

    lines.sort_by(|a, b| {
        b[2].parse::<i32>()
            .unwrap()
            .cmp(&a[2].parse::<i32>().unwrap())
    });
    lines.iter().enumerate().for_each(|(i, caps)| {
        xx_to_id.insert(caps[1].to_string(), i as u8);
    });

    (
        lines
            .iter()
            .map(|caps| Node {
                name: *xx_to_id.get(&caps[1]).unwrap(),
                value: caps[2].parse::<i32>().unwrap(),
                neighbors: caps[3]
                    .split(", ")
                    .map(|s| *xx_to_id.get(s).unwrap())
                    .collect(),
            })
            .collect::<Vec<Node>>(),
        xx_to_id,
    )
}

pub fn find_path(
    current_node: u8,
    time: i32,
    flow: i32,
    nodes: &[Node],
    distance_graph: &DistanceGraph,
    visited: &HashSet<u8>,
) -> (i32, HashSet<u8>) {
    let distances = distance_graph.get(&current_node).unwrap();
    return nodes
        .iter()
        .filter(|n| {
            n.value > 0 && !visited.contains(&n.name) && distances.get(&n.name).unwrap() <= &time
        })
        .map(|n| {
            let distance = distances.get(&n.name).unwrap();
            let time_left = time - distance - 1;
            if time_left <= 2 {
                return (flow, visited.clone());
            }
            let value = time_left * n.value;
            let mut up_visited = visited.clone();
            up_visited.insert(n.name);
            find_path(
                n.name,
                time_left,
                flow + value,
                nodes,
                distance_graph,
                &up_visited,
            )
        })
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap_or((flow, visited.clone()));
}

pub fn find_path2(
    elephant: (u8, i32),
    person: (u8, i32),
    flow: i32,
    nodes: &[Node],
    distance_graph: &DistanceGraph,
    visited: &HashSet<u8>,
) -> i32 {
    let elephant_distances = distance_graph.get(&elephant.0).unwrap();
    let person_distances = distance_graph.get(&person.0).unwrap();
    return nodes
        .iter()
        .filter(|n| {
            n.value > 0
                && !visited.contains(&n.name)
                && (elephant_distances.get(&n.name).unwrap() <= &elephant.1
                    || person_distances.get(&n.name).unwrap() <= &person.1)
        })
        .flat_map(|n| {
            let e_distance = elephant_distances.get(&n.name).unwrap();
            let e_time_left = elephant.1 - e_distance - 1;
            let p_distance = person_distances.get(&n.name).unwrap();
            let p_time_left = person.1 - p_distance - 1;
            if e_time_left <= 2 && p_time_left <= 2 {
                return vec![flow];
            }
            let e_value = e_time_left * n.value;
            let p_value = p_time_left * n.value;
            let mut up_visited = visited.clone();
            up_visited.insert(n.name);

            vec![
                if e_time_left > 2 {
                    find_path2(
                        (n.name, e_time_left),
                        person,
                        flow + e_value,
                        nodes,
                        distance_graph,
                        &up_visited,
                    )
                } else {
                    flow
                },
                if p_time_left > 2 {
                    find_path2(
                        elephant,
                        (n.name, p_time_left),
                        flow + p_value,
                        nodes,
                        distance_graph,
                        &up_visited,
                    )
                } else {
                    flow
                },
            ]
        })
        .max()
        .unwrap_or(flow);
}

pub fn find_distances_to_valves(nodes: &[Node]) -> DistanceGraph {
    let mut distance_graph = DistanceGraph::new();
    let valves: Vec<&Node> = nodes.iter().filter(|n| n.value > 0).collect();
    for node in nodes {
        let mut distances = HashMap::<u8, i32>::new();
        for valve in &valves {
            if valve.name != node.name {
                let distance = find_shortest_path(node, valve, nodes);
                distances.insert(valve.name, distance);
            }
        }
        distance_graph.insert(node.name, distances);
    }
    distance_graph
}

pub fn find_shortest_path(start: &Node, destination: &Node, nodes: &[Node]) -> i32 {
    let mut distances = HashMap::<u8, i32>::new();
    let mut distance = 0;
    let mut current = start.name;
    distances.insert(current, distance);
    for node in nodes {
        distances.insert(node.name, i32::MAX);
    }
    let mut visited = HashSet::<u8>::new();
    visited.insert(current);

    while current != destination.name {
        let node = nodes.iter().find(|n| n.name == current).unwrap();
        //update values
        for n in &node.neighbors {
            if !visited.contains(n) {
                distances
                    .entry(*n)
                    .and_modify(|v| *v = min(distance + 1, *v));
            }
        }
        //select next node
        let (found, found_distance) = distances
            .iter()
            .filter(|n| !visited.contains(n.0))
            .min_by(|a, b| a.1.cmp(b.1))
            .unwrap();
        visited.insert(found.to_owned());
        distance = *found_distance;
        current = found.to_owned();
    }
    distance
}

pub fn part_one(input: &str) -> Option<i32> {
    let (nodes, id_map) = parse_nodes(input);
    let distance_graph = find_distances_to_valves(&nodes);
    let visited = HashSet::<u8>::new();
    let half_nodes: Vec<Node> = nodes.iter().filter(|n| n.value > 0).cloned().collect();
    Some(
        find_path(
            *id_map.get("AA").unwrap(),
            30,
            0,
            &half_nodes,
            &distance_graph,
            &visited,
        )
        .0,
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let (nodes, id_map) = parse_nodes(input);
    let distance_graph = find_distances_to_valves(&nodes);
    let visited = HashSet::<u8>::new();
    let half_nodes: Vec<Node> = nodes.iter().filter(|n| n.value > 0).cloned().collect();

    // Some(find_path2(
    //     (*id_map.get("AA").unwrap(), 26),
    //     (*id_map.get("AA").unwrap(), 26),
    //     0,
    //     &half_nodes,
    //     &distance_graph,
    //     &visited,
    // ))
    let (person_visited, path) = find_path(
        *id_map.get("AA").unwrap(),
        26,
        0,
        &half_nodes,
        &distance_graph,
        &visited,
    );
    let (e_visited, _) = find_path(
        *id_map.get("AA").unwrap(),
        26,
        0,
        &half_nodes,
        &distance_graph,
        &path,
    );
    Some(person_visited + e_visited)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
