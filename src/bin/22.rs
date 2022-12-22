pub enum Direction {
    Move(u32),
    Turn(char),
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Direction>) {
    let mut map: Vec<Vec<char>> = vec![];
    let mut directions: Vec<Direction> = vec![];
    let mut parts = input.split("\n\n");
    let map_str = parts.next().unwrap();
    let dirs = parts.next().unwrap();
    //parse map
    let max_len = map_str
        .lines()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .len();

    for l in map_str.lines() {
        let mut line: Vec<char> = l.chars().collect();
        while line.len() < max_len {
            line.push(' ')
        }
        map.push(line)
    }

    //parse dirs
    let mut distance_string = String::from("");
    for c in dirs.chars() {
        if c.is_numeric() {
            distance_string.push(c);
        } else {
            directions.push(Direction::Move(distance_string.parse::<u32>().unwrap()));
            directions.push(Direction::Turn(c));
            distance_string = String::from("");
        }
    }
    if !distance_string.is_empty() {
        directions.push(Direction::Move(distance_string.parse::<u32>().unwrap()));
    }

    (map, directions)
}

// gotta do y,x
pub fn move_on_map(map: Vec<Vec<char>>, directions: &[Direction]) -> u32 {
    let max_y = (map).len() as i32;
    let max_x = (map[0]).len() as i32;
    let mut current_dir: (i32, i32) = (1, 0);
    let mut first_space = 0;
    for (i, c) in (map[0]).iter().enumerate() {
        if *c == '.' {
            first_space = i;
            break;
        }
    }
    let mut position: (i32, i32) = (first_space as i32, 0);

    for dir in directions {
        if let Direction::Move(x) = dir {
            for _ in 0..*x {
                let mut stuck = false;
                (position, stuck) = advance(position, current_dir, &map, max_x, max_y);
                if stuck {
                    break;
                }
            }
        } else {
            current_dir = match dir {
                Direction::Turn('L') => match current_dir {
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    _ => panic!("Dir is effed"),
                },
                Direction::Turn('R') => match current_dir {
                    (1, 0) => (0, 1),
                    (0, -1) => (1, 0),
                    (-1, 0) => (0, -1),
                    (0, 1) => (-1, 0),
                    _ => panic!("Dir is effed"),
                },
                _ => panic!("wtf"),
            }
        }
    }

    let dir_value = match current_dir {
        (1, 0) => 0,
        (0, -1) => 1,
        (-1, 0) => 2,
        (0, 1) => 3,
        _ => panic!("Dir is effed"),
    };

    (((position.1 + 1) * 1000) + ((position.0 + 1) * 4) + dir_value) as u32
}

pub fn advance(
    position: (i32, i32),
    current_dir: (i32, i32),
    map: &[Vec<char>],
    max_x: i32,
    max_y: i32,
) -> ((i32, i32), bool) {
    let mut new_pos = (position.0 + current_dir.0, position.1 + current_dir.1);
    //handle x wraparound
    if !(0..max_x).contains(&new_pos.0) {
        if new_pos.0 < 0 {
            new_pos = (max_x - 1, new_pos.1);
        } else {
            new_pos = (0, new_pos.1);
        }
    }
    //handle y wraparound
    if !(0..max_y).contains(&new_pos.1) {
        if new_pos.1 < 0 {
            new_pos = (new_pos.0, max_y - 1);
        } else {
            new_pos = (new_pos.0, 0);
        }
    }
    match map[new_pos.1 as usize][new_pos.0 as usize] {
        '#' => (position, true),
        '.' => (new_pos, false),
        ' ' => {
            let (pos, stuck) = advance(new_pos, current_dir, map, max_x, max_y);
            if stuck {
                (position, true)
            } else {
                (pos, false)
            }
        }
        _ => panic!("oh shit"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, directions) = parse_input(input);
    Some(move_on_map(map, &directions))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), None);
    }
}
