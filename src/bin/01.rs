advent_of_code::solution!(1);

pub fn build_calorie_counts(input: &str) -> Vec<i32> {
    let values: Vec<&str> = input.lines().collect();
    let mut calorie_counts: Vec<i32> = Vec::new();
    let mut current_calories = 0;
    for calorie_value in values {
        if calorie_value.is_empty() {
            calorie_counts.push(current_calories);
            current_calories = 0;
        } else {
            current_calories += calorie_value.parse::<i32>().unwrap();
        }
    }
    // push last value
    calorie_counts.push(current_calories);
    calorie_counts.sort_by(|a, b| b.cmp(a));
    calorie_counts
}

pub fn part_one(input: &str) -> Option<i32> {
    let calorie_counts = build_calorie_counts(input);
    Some(calorie_counts[0])
}

pub fn part_two(input: &str) -> Option<i32> {
    let calorie_counts = build_calorie_counts(input);
    let highest = &calorie_counts[..3];
    Some(highest.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(45000));
    }
}
