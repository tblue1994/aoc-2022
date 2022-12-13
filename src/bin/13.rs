use std::cmp::Ordering;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum Packet {
    Term(u8),
    Nest(Vec<Packet>),
}
impl PartialEq for Packet {
    fn eq(&self, rhs: &Self) -> bool {
        self.cmp(rhs) == Ordering::Equal
    }
}
impl Eq for Packet {}
impl PartialOrd for Packet {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
impl Ord for Packet {
    fn cmp(&self, rhs: &Self) -> Ordering {
        use Packet::*;
        match (self, rhs) {
            (Term(l), Term(r)) => l.cmp(r),
            (Nest(l), Nest(r)) => l.cmp(r),
            (Term(l), Nest(r)) => [Term(*l)][..].cmp(r),
            (Nest(l), Term(r)) => l.as_slice().cmp(&[Term(*r)]),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let sum_indexes = input
        .split("\n\n")
        .map(|p| {
            let mut lines = p.lines();
            (
                serde_json::from_str::<Packet>(lines.next().unwrap()).unwrap(),
                serde_json::from_str::<Packet>(lines.next().unwrap()).unwrap(),
            )
        })
        .enumerate()
        .filter(|(_, (a, b))| a < b)
        .map(|(i, _)| i + 1)
        .sum::<usize>();
    Some(sum_indexes)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut new_packages: String = "[[2]]\n[[6]]\n".to_string();
    new_packages.push_str(input);

    let mut packages: Vec<Packet> = new_packages
        .replace("\n\n", "\n")
        .lines()
        .map(|p| serde_json::from_str::<Packet>(p).unwrap())
        .collect();
    packages.sort();
    let mut total = 1;
    for i in 0..packages.len() {
        if packages[i] == serde_json::from_str::<Packet>("[[2]]").unwrap()
            || packages[i] == serde_json::from_str::<Packet>("[[6]]").unwrap()
        {
            total *= i + 1
        }
    }
    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
