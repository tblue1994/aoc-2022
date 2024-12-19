advent_of_code::solution!(6);

use std::collections::HashSet;

pub fn get_chars_unique_length_index(input: &str, num_unique: usize) -> Option<usize> {
    let mut index = 0;
    for i in (num_unique - 1)..input.len() {
        let set: HashSet<char> = HashSet::from_iter(
            input
                .chars()
                .skip(i - (num_unique - 1))
                .take(num_unique)
                .collect::<Vec<char>>(),
        );
        if set.len() == num_unique {
            index = i + 1;
            break;
        }
    }
    Some(index)
}

pub fn part_one(input: &str) -> Option<usize> {
    get_chars_unique_length_index(input, 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    get_chars_unique_length_index(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(19));
    }
}
