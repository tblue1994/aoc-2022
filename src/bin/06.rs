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

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
