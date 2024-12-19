advent_of_code::solution!(14);

use std::collections::HashSet;

type Pos = (i32, i32);
type State = HashSet<Pos>;

pub fn build_initial_state(input: &str) -> State {
    let mut state = State::new();
    let ledges: Vec<Vec<Pos>> = input
        .lines()
        .map(|l| {
            l.split(" -> ")
                .map(|s| {
                    let mut coord = s.split(',').map(|c| c.parse::<i32>().unwrap());
                    (coord.next().unwrap(), coord.next().unwrap())
                })
                .collect::<Vec<Pos>>()
        })
        .collect();

    for ledge in ledges {
        for i in 0..ledge.len() - 1 {
            let current = ledge[i];
            let next = ledge[i + 1];
            let x_diff = i32::abs(current.0 - next.0);
            let y_diff = i32::abs(current.1 - next.1);
            let modifier = if current.0 > next.0 {
                (-1, 0)
            } else if current.0 < next.0 {
                (1, 0)
            } else if current.1 > next.1 {
                (0, -1)
            } else {
                (0, 1)
            };

            for x in 0..=x_diff {
                for y in 0..=y_diff {
                    state.insert(((x * modifier.0) + current.0, (y * modifier.1) + current.1));
                }
            }
        }
    }

    state
}

pub fn rain_sand(mut state: State, include_floor: bool) -> usize {
    let initial = state.len();
    let max_y = state.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let mut full = false;
    while !full {
        let mut sand_location = (500, 0);
        loop {
            let next_locations = vec![
                (sand_location.0, sand_location.1 + 1),
                (sand_location.0 - 1, sand_location.1 + 1),
                (sand_location.0 + 1, sand_location.1 + 1),
            ];
            let mut stuck = true;
            for location in next_locations {
                if state.contains(&location) || (include_floor && location.1 == max_y + 2) {
                    continue;
                } else {
                    sand_location = location;
                    stuck = false;
                    break;
                }
            }
            if stuck {
                state.insert(sand_location);
                if sand_location == (500, 0) {
                    full = true
                }

                break;
            }
            if !include_floor && sand_location.1 > max_y {
                full = true;
                break;
            }
        }
    }
    state.len() - initial
}

pub fn part_one(input: &str) -> Option<usize> {
    let state = build_initial_state(input);
    Some(rain_sand(state, false))
}

pub fn part_two(input: &str) -> Option<usize> {
    let state = build_initial_state(input);
    Some(rain_sand(state, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(93));
    }
}
