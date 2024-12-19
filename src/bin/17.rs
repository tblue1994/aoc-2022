advent_of_code::solution!(17);

use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

type Moves = Vec<bool>;
type Point = (i128, i128);

pub fn parse_moves(input: &str) -> Moves {
    input.chars().map(|c| c == '>').collect()
}

pub fn drop_rocks(num_rocks: u128, moves: &Moves) -> u128 {
    let shapes: Vec<Vec<Point>> = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (1, 1), (0, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];
    let mut hash: HashMap<(usize, usize, i128), (u128, i128)> = HashMap::new();
    let mut max_y = 0;
    let mut move_index = 0;
    let mut filled_blocks: HashSet<Point> = HashSet::new();
    let mut height_diff: u128 = 0;
    let mut batches = 0;
    let mut i = 0;
    let mut found = false;
    while i < num_rocks {
        let shape = &shapes[(i % shapes.len() as u128) as usize];
        let mut placed_shape: Vec<Point> =
            shape.iter().map(|(x, y)| (x + 2, y + max_y + 4)).collect();
        let mut stuck = false;
        while !stuck {
            let m = moves[move_index];
            move_index = (move_index + 1) % moves.len();
            (placed_shape, _) =
                move_shape((if m { 1 } else { -1 }, 0), &placed_shape, &filled_blocks);
            (placed_shape, stuck) = move_shape((0, -1), &placed_shape, &filled_blocks);
        }
        for p in placed_shape {
            filled_blocks.insert(p);
            max_y = max(max_y, p.1)
        }
        i += 1;

        if let Some((prev_i, prev_height)) = hash.insert(
            (
                (i as usize) % shapes.len(),
                move_index % moves.len(),
                get_top(
                    &filled_blocks
                        .iter()
                        .filter(|(_, y)| *y == max_y)
                        .map(|p| p.to_owned())
                        .collect::<Vec<Point>>(),
                ),
            ),
            (i, max_y),
        ) {
            if !found {
                height_diff = (max_y - prev_height) as u128;
                let rocks_left = num_rocks - i;
                let rocks_diff = i - prev_i;
                batches = rocks_left / rocks_diff;
                i += batches * rocks_diff;
                found = true;
            }
        }
    }
    (max_y as u128 + (height_diff * batches)) as u128
}

pub fn get_top(points: &[Point]) -> i128 {
    let mut num = 0;
    for (x, _) in points {
        num |= 2_i128.pow(*x as u32)
    }
    num
}

pub fn move_shape(
    (d_x, d_y): Point,
    shape: &[Point],
    filled: &HashSet<Point>,
) -> (Vec<Point>, bool) {
    let mut new_loc: Vec<Point> = vec![];
    let mut stuck = false;
    for (x, y) in shape {
        let new_point = (x + d_x, y + d_y);
        new_loc.push(new_point);
        stuck = filled.contains(&new_point) || !(0..7).contains(&new_point.0) || new_point.1 < 1;
        if stuck {
            break;
        }
    }
    (if stuck { shape.to_owned() } else { new_loc }, stuck)
}

pub fn part_one(input: &str) -> Option<u128> {
    Some(drop_rocks(2022, &parse_moves(input)))
}

pub fn part_two(input: &str) -> Option<u128> {
    Some(drop_rocks(1000000000000, &parse_moves(input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
