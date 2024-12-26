use regex::Regex;

advent_of_code::solution!(22);

pub const UP: (isize, isize) = (0, -1);
pub const DOWN: (isize, isize) = (0, 1);
pub const LEFT: (isize, isize) = (-1, 0);
pub const RIGHT: (isize, isize) = (1, 0);

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
                let stuck;
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
    let mut map: Vec<Vec<char>> = vec![];
    let mut directions: Vec<&str> = vec![];
    for line in input.lines() {
        if line.contains(".") {
            map.push(line.chars().collect());
        }

        if line.contains("L") {
            //make regex
            let re = Regex::new(r"(\d+|L|R)").unwrap();
            for (_, [dir]) in re.captures_iter(input).map(|c| c.extract()) {
                directions.push(dir);
            }
        }
    }
    let mut location = (50, 0);
    let mut direction = RIGHT;
    println!("{:?}, {:?}", location, direction);
    for dir in directions {
        println!("----");
        println!("{}", dir);
        (location, direction) = follow_direction(location, direction, dir, &map);
        println!("{:?}, {:?}", location, direction);
    }

    let dir_value = match direction {
        RIGHT => 0,
        LEFT => 2,
        DOWN => 1,
        UP => 3,
        _ => panic!("Dir is effed"),
    };
    Some(((1000 * (location.1 + 1)) + (4 * (location.0 + 1)) + dir_value) as u32)
}
pub fn follow_direction(
    location: (isize, isize),
    direction: (isize, isize),
    dir: &str,
    map: &Vec<Vec<char>>,
) -> ((isize, isize), (isize, isize)) {
    if dir.parse::<u32>().is_ok() {
        advance_cube(location, direction, dir.parse().unwrap(), map)
    } else {
        (location, turn(direction, dir))
    }
}

pub fn turn(direction: (isize, isize), dir: &str) -> (isize, isize) {
    match dir {
        "L" => match direction {
            LEFT => DOWN,
            UP => LEFT,
            RIGHT => UP,
            DOWN => RIGHT,
            _ => panic!("Dir is effed"),
        },
        "R" => match direction {
            LEFT => UP,
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            _ => panic!("Dir is effed"),
        },
        _ => panic!("wtf"),
    }
}

pub fn advance_cube(
    location: (isize, isize),
    direction: (isize, isize),
    dir: u32,
    map: &Vec<Vec<char>>,
) -> ((isize, isize), (isize, isize)) {
    if dir == 0 {
        return (location, direction);
    }
    //handle special cases
    let (new_loc, new_dir) = special_case(
        location.0 + direction.0,
        location.1 + direction.1,
        direction,
    );
    //check map for walls
    if map[new_loc.1 as usize][new_loc.0 as usize] == ' ' {
        panic!("in empty space");
    }

    if map[new_loc.1 as usize][new_loc.0 as usize] == '#' {
        return (location, direction);
    }

    advance_cube(new_loc, new_dir, dir - 1, map)
}

pub fn special_case(
    x: isize,
    y: isize,
    direction: (isize, isize),
) -> ((isize, isize), (isize, isize)) {
    if y == -1 && direction == UP {
        //Top of 2
        if (50..100).contains(&x) {
            //left of 6
            println!("Top 2 -> Left 6");
            return ((0, x + 100), RIGHT);
        }
        //Top of 1
        if (100..150).contains(&x) {
            //bottom of 6
            println!("Top 1 -> Bottom 6");
            return ((x - 100, 199), UP);
        }
    }
    if y == 50 && direction == DOWN {
        //bottom of 1
        if (100..150).contains(&x) {
            //right of 3
            println!("Bottom 1 -> right 3");
            return ((99, x - 50), LEFT);
        }
    }
    //top of 5
    if y == 99 && direction == UP {
        //left of 3
        if (0..50).contains(&x) {
            println!("top 5 -> left 3");
            return ((50, x + 50), RIGHT);
        }
    }
    if y == 150 && direction == DOWN {
        //bottom 4
        if (50..100).contains(&x) {
            //right 6
            println!("bottom 4 -> right 6");
            return ((49, x + 100), LEFT);
        }
    }
    //bottom of 6
    if y >= 200 && direction == DOWN {
        //top of 1
        println!("bottom 6 -> top 1");
        return ((x + 100, 0), DOWN);
    }

    if x == -1 && direction == LEFT {
        //left of 6
        if (150..200).contains(&y) {
            //top of 2
            println!("left 6 -> top 2");
            return ((y - 100, 0), DOWN);
        }
        //left of 5
        if (100..150).contains(&y) {
            println!("left 5 -> left 2");
            return ((50, 149 - y), RIGHT);
        }
    }
    if x == 49 && direction == LEFT {
        //left of 2
        if (0..50).contains(&y) {
            //upside down left of 5
            println!("left 2 -> left 5");
            return ((0, 149 - y), RIGHT);
        }
        //left of 3
        if (50..100).contains(&y) {
            //Top of 5
            println!("left 3 -> top 5");
            return ((y - 50, 100), DOWN);
        }
    }
    if x == 50 && direction == RIGHT {
        //right 6
        if (150..200).contains(&y) {
            //bottom 4
            println!("right 6 -> bottom 4");
            return ((y - 100, 149), UP);
        }
    }
    if x == 100 && direction == RIGHT {
        // right of 3
        if (50..100).contains(&y) {
            //bottom 1
            println!("right 3 -> bottom 1");
            return ((y + 50, 49), UP);
        }
        //right of 4
        if (100..150).contains(&y) {
            //inverse right of 1
            println!("right 4 -> right 1");
            return ((149, 149 - y), LEFT);
        }
    }
    //right 1
    if x >= 150 && direction == RIGHT {
        //inverse right of 4
        println!("right 1 -> right 4");
        return ((99, 149 - y), LEFT);
    }

    return ((x, y), direction);
}

pub fn test_one(_input: &str) -> Option<u32> {
    None
}
pub fn test_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(test_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(test_two(&input), None);
    }
}
