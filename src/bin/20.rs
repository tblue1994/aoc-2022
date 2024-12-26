advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<i128> {
    let mut file_bits: Vec<(i128, usize)> = vec![];
    for (i, line) in input.lines().enumerate() {
        file_bits.push((line.parse().unwrap(), i));
    }
    let file_bits_slice = file_bits.as_mut_slice();

    mix(file_bits_slice);

    get_answer(file_bits_slice)
}

pub fn part_two(input: &str) -> Option<i128> {
    let mut file_bits: Vec<(i128, usize)> = vec![];
    for (i, line) in input.lines().enumerate() {
        file_bits.push(((line.parse::<i128>().unwrap() * 811589153), i));
    }
    let file_bits_slice = file_bits.as_mut_slice();

    for _ in 0..10 {
        mix(file_bits_slice);
        // println!("{}", i);
        // println!("{:?}", file_bits_slice);
    }

    get_answer(file_bits_slice)
}

pub fn mix(file_bits_slice: &mut [(i128, usize)]) {
    // println!("{:?}", file_bits_slice);
    for i in 0..file_bits_slice.len() {
        let mut index = file_bits_slice.iter().position(|x| x.1 == i).unwrap();
        let t = file_bits_slice[index];

        // let mut change = 1;
        // if t.0 < 0 {
        //     change = -1;
        // }
        let times_to_run = t.0.rem_euclid((file_bits_slice.len() - 1) as i128);

        for _ in 0..times_to_run {
            let new_index = (index as i32 + 1).rem_euclid(file_bits_slice.len() as i32) as usize;
            file_bits_slice.swap(new_index, index);
            index = new_index;
        }
        // println!("{:?}", file_bits_slice);
    }
}

pub fn get_answer(file_bits_slice: &[(i128, usize)]) -> Option<i128> {
    let zero_pos = file_bits_slice.iter().position(|x| x.0 == 0).unwrap();
    Some(
        file_bits_slice[(zero_pos + 1000) % file_bits_slice.len()].0
            + file_bits_slice[(zero_pos + 2000) % file_bits_slice.len()].0
            + file_bits_slice[(zero_pos + 3000) % file_bits_slice.len()].0,
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1623178306));
    }
}
