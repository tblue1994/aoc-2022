const THEIR_ROCK: &str = "A";
const THEIR_PAPER: &str = "B";

const MY_ROCK: &str = "X";
const MY_PAPER: &str = "Y";

const WIN_VALUE: u32 = 6;
const TIE_VALUE: u32 = 3;
const LOSS_VALUE: u32 = 0;

pub fn determine_sign_value(mine: &str) -> u32 {
    match mine {
        MY_ROCK => 1,
        MY_PAPER => 2,
        _ => 3,
    }
}

pub fn determine_winner_value(theirs: &str, mine: &str) -> u32 {
    match theirs {
        THEIR_ROCK => match mine {
            MY_ROCK => TIE_VALUE,
            MY_PAPER => WIN_VALUE,
            _ => LOSS_VALUE,
        },
        THEIR_PAPER => match mine {
            MY_ROCK => LOSS_VALUE,
            MY_PAPER => TIE_VALUE,
            _ => WIN_VALUE,
        },
        _ => match mine {
            MY_ROCK => WIN_VALUE,
            MY_PAPER => LOSS_VALUE,
            _ => TIE_VALUE,
        },
    }
}

pub fn determine_round_value(theirs: &str, mine: &str) -> u32 {
    determine_sign_value(mine) + determine_winner_value(theirs, mine)
}

pub fn determine_my_sign(theirs: &str, outcome: &str) -> &'static str {
    const WIN: &str = "Z";
    const DRAW: &str = "Y";
    const SCISSORS: &str = "Z";
    match outcome {
        WIN => match theirs {
            THEIR_ROCK => MY_PAPER,
            THEIR_PAPER => SCISSORS,
            _ => MY_ROCK,
        },
        DRAW => match theirs {
            THEIR_ROCK => MY_ROCK,
            THEIR_PAPER => MY_PAPER,
            _ => SCISSORS,
        },
        _ => match theirs {
            THEIR_ROCK => SCISSORS,
            THEIR_PAPER => MY_ROCK,
            _ => MY_PAPER,
        },
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let values: Vec<&str> = input.lines().collect();
    let mut total: u32 = 0;

    for round_string in values {
        let v: Vec<&str> = round_string.split(' ').collect();
        let theirs = v[0];
        let mine = v[1];
        total += determine_round_value(theirs, mine)
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let values: Vec<&str> = input.lines().collect();
    let mut total: u32 = 0;

    for round_string in values {
        let v: Vec<&str> = round_string.split(' ').collect();
        let theirs = v[0];
        let outcome = v[1];
        let mine = determine_my_sign(theirs, outcome);
        total += determine_round_value(theirs, mine)
    }
    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
