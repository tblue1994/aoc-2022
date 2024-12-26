advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<String> {
    let mut snafu_sum = 0;
    for line in input.lines() {
        snafu_sum += snafu_to_base_10(line);
    }
    let snafu = base_10_to_snafu(snafu_sum);
    assert_eq!(snafu_sum, snafu_to_base_10(&snafu));
    Some(snafu)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

pub fn snafu_to_base_10(line: &str) -> i64 {
    let mut snafu_value = 0;
    for (i, c) in line.chars().rev().enumerate() {
        let base = match c {
            '=' => -2,
            '-' => -1,
            x @ ('0' | '1' | '2') => x.to_digit(10).unwrap() as i64,
            _ => panic!("Not Snafu Character"),
        };
        snafu_value += base * 5_i64.pow(i as u32)
    }
    snafu_value
}

pub fn base_10_to_snafu(val: i64) -> String {
    let mut result = vec![];
    let mut x = val;

    loop {
        let m = (x) % 5 as i64;
        x = x / 5 as i64;

        // will panic if you use a bad radix (< 2 or > 36).
        let c = match m {
            n @ (0 | 1 | 2) => std::char::from_digit(n as u32, 5).unwrap(),
            3 => {
                x += 1;
                '='
            }
            4 => {
                x += 1;
                '-'
            }
            _ => panic!("o shit"),
        };

        result.push(c);
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("2=-1=0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
