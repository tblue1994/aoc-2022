pub fn get_item_value(item: char) -> u32 {
    let utf8_value = item as u32;
    if utf8_value > 96 {
        utf8_value - 96
    } else {
        utf8_value - 64 + 26
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let rucksacks: Vec<&str> = input.lines().collect();
    let mut total: u32 = 0;
    for rucksack in rucksacks {
        let (compartment1, compartment2) = rucksack.split_at(rucksack.len() / 2);
        let mut misplaced_item: Option<char> = None;
        for c in compartment1.chars() {
            if compartment2.contains(c) {
                misplaced_item = Some(c);
                break;
            }
        }
        total += get_item_value(misplaced_item.unwrap())
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rucksacks: Vec<&str> = input.lines().collect();
    let mut total: u32 = 0;
    for i in (0..rucksacks.len() - 1).step_by(3) {
        let mut misplaced_item: Option<char> = None;
        for c in rucksacks[i].chars() {
            if rucksacks[i + 1].contains(c) && rucksacks[i + 2].contains(c) {
                misplaced_item = Some(c);
                break;
            }
        }
        total += get_item_value(misplaced_item.unwrap())
    }
    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
