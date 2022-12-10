pub fn part_one(input: &str) -> Option<i32> {
    let instrustions: Vec<(&str, i32)> = input
        .lines()
        .map(|s| s.split(' ').collect::<Vec<&str>>())
        .map(|a| {
            if a.len() == 2 {
                (a[0], a[1].parse::<i32>().unwrap())
            } else {
                (a[0], 0)
            }
        })
        .collect();

    let mut value: i32 = 1;
    let mut iteration_check = 20;
    let mut signal_strength = 0;
    let mut cycle_no = 1;
    let mut instruction_index = 0;
    while cycle_no < 220 {
        let instr = instrustions[instruction_index];
        match instr.0 {
            "noop" => cycle_no += 1,
            _ => cycle_no += 2,
        }
        instruction_index += 1;

        if cycle_no > iteration_check {
            signal_strength += value * iteration_check;
            iteration_check += 40
        }

        value += instr.1;
    }

    Some(signal_strength)
}

pub fn part_two(input: &str) -> Option<u32> {
    let instrustions: Vec<(&str, i32)> = input
        .lines()
        .map(|s| s.split(' ').collect::<Vec<&str>>())
        .map(|a| {
            if a.len() == 2 {
                (a[0], a[1].parse::<i32>().unwrap())
            } else {
                (a[0], 0)
            }
        })
        .collect();

    let mut value: i32 = 1;
    let mut instruction_index = 0;
    let mut buffer: Vec<char> = vec![];
    while buffer.len() < 240 {
        let instr = instrustions[instruction_index];
        let times = match instr.0 {
            "noop" => 1,
            _ => 2,
        };
        instruction_index += 1;

        for _ in 0..times {
            if (buffer.len() % 40) as i32 >= value - 1 && (buffer.len() % 40) as i32 <= value + 1 {
                buffer.push('█')
            } else {
                buffer.push('.')
            }
        }

        value += instr.1;
    }

    println!("{}", buffer[0..40].iter().collect::<String>());
    println!("{}", buffer[40..80].iter().collect::<String>());
    println!("{}", buffer[80..120].iter().collect::<String>());
    println!("{}", buffer[120..160].iter().collect::<String>());
    println!("{}", buffer[160..200].iter().collect::<String>());
    println!("{}", buffer[200..240].iter().collect::<String>());

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
