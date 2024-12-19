advent_of_code::solution!(23);

use std::collections::{HashMap, HashSet, VecDeque};

type Point = (i32, i32);
pub fn parse_input(input: &str) -> HashSet<Point> {
    let mut points = HashSet::<Point>::new();
    input.lines().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                points.insert((j as i32, i as i32));
            }
        })
    });
    points
}

pub fn generate_neighbors(p: Point, round: usize) -> VecDeque<Vec<Point>> {
    //NW
    let nw = (p.0 - 1, p.1 - 1);
    //N
    let n = (p.0, p.1 - 1);
    //NE
    let ne = (p.0 + 1, p.1 - 1);
    //E
    let e = (p.0 + 1, p.1);
    //SE
    let se = (p.0 + 1, p.1 + 1);
    //S
    let s = (p.0, p.1 + 1);
    //SW
    let sw = (p.0 - 1, p.1 + 1);
    //W
    let w = (p.0 - 1, p.1);
    let mut neighbors: VecDeque<Vec<Point>> = VecDeque::from([
        vec![ne, n, nw],
        vec![se, s, sw],
        vec![sw, w, nw],
        vec![se, e, ne],
    ]);
    neighbors.rotate_left(round % 4);
    neighbors
}

pub fn propose_moves(
    current_positions: &HashSet<Point>,
    round: usize,
) -> HashMap<Point, Vec<Point>> {
    let mut proposed: HashMap<Point, Vec<Point>> = HashMap::new();
    for point in current_positions {
        // get neighboring points
        let neighbor_groups = generate_neighbors(*point, round);
        //check if stuck
        let mut stuck = true;
        for p in neighbor_groups.iter().flatten() {
            if current_positions.contains(p) {
                stuck = false;
                break;
            }
        }
        if stuck {
            proposed.insert(*point, vec![*point]);
            continue;
        }
        let mut moved = false;
        for g in neighbor_groups {
            let mut clear = true;
            for n in &g {
                if current_positions.contains(n) {
                    clear = false;
                    break;
                }
            }
            if clear {
                proposed
                    .entry(g[1])
                    .and_modify(|points| points.push(*point))
                    .or_insert_with(|| vec![*point]);
                moved = true;
                break;
            }
        }
        if !moved {
            proposed.insert(*point, vec![*point]);
        }
    }
    proposed
}

pub fn execute_moves(proposed_moves: &HashMap<Point, Vec<Point>>) -> HashSet<Point> {
    let mut points = HashSet::<Point>::new();
    for (k, v) in proposed_moves {
        if v.len() == 1 {
            points.insert(*k);
        } else {
            v.iter().for_each(|p| {
                points.insert(*p);
            });
        }
    }
    points
}

pub fn num_empty_spots(current_positions: &HashSet<Point>) -> u32 {
    let min_x = current_positions
        .iter()
        .min_by(|a, b| a.0.cmp(&b.0))
        .unwrap()
        .0;
    let max_x = current_positions
        .iter()
        .max_by(|a, b| a.0.cmp(&b.0))
        .unwrap()
        .0;
    let min_y = current_positions
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .1;
    let max_y = current_positions
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap()
        .1;
    (((max_x - min_x + 1) * (max_y - min_y + 1)) - current_positions.len() as i32) as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut current_positions = parse_input(input);
    for i in 0..10 {
        let proposed_moves = propose_moves(&current_positions, i);
        current_positions = execute_moves(&proposed_moves);
    }
    Some(num_empty_spots(&current_positions))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut current_positions = parse_input(input);
    let mut i = 0;
    loop {
        let proposed_moves = propose_moves(&current_positions, i);
        if proposed_moves
            .iter()
            .all(|(k, v)| v.len() == 1 && v[0] == *k)
        {
            break;
        }
        current_positions = execute_moves(&proposed_moves);
        i += 1;
    }
    Some(i as u32 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(20));
    }
}
