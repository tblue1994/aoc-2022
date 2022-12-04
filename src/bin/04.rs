pub fn solve(input: &str, f: fn(u32, u32, u32, u32) -> bool) -> Option<u32> {
    let assignment_pairs: Vec<&str> = input.lines().collect();
    let mut total: u32 = 0;

    for pair_string in assignment_pairs {
        let nums: Vec<&str> = pair_string.split(&[',', '-'][..]).collect();

        if f(
            nums[0].parse().unwrap(),
            nums[1].parse().unwrap(),
            nums[2].parse().unwrap(),
            nums[3].parse().unwrap(),
        ) {
            total += 1
        }
    }

    Some(total)
}

pub fn is_all_overlap(start1: u32, end1: u32, start2: u32, end2: u32) -> bool {
    (start1 <= start2 && end1 >= end2) || (start2 <= start1 && end2 >= end1)
}

pub fn is_any_overlap(start1: u32, end1: u32, start2: u32, end2: u32) -> bool {
    let range1 = start1..(end1 + 1);
    let range2 = start2..(end2 + 1);
    range1.contains(&start2)
        || range1.contains(&end2)
        || range2.contains(&start1)
        || range2.contains(&end1)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, is_all_overlap)
}
pub fn part_two(input: &str) -> Option<u32> {
    solve(input, is_any_overlap)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
