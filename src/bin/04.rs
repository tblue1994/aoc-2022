advent_of_code::solution!(4);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(4));
    }
}
