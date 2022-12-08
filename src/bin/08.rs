use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<usize> {
    let lines: Vec<&str> = input.lines().collect();
    let mut forest: Vec<Vec<u32>> = vec![];
    for line in lines {
        forest.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect())
    }
    let mut hash: HashSet<(usize, usize)> = HashSet::new();
    //forwards horizontally
    for x in 0..forest.len() {
        let mut current_tree = forest[x][0];
        for y in 0..forest[0].len() {
            if x == 0 || y == 0 {
                hash.insert((x, y));
            } else if forest[x][y] > current_tree {
                hash.insert((x, y));
                current_tree = forest[x][y]
            }
            if current_tree == 9 {
                break;
            }
        }
    }
    //backwards horizontally
    for x in (0..forest.len()).rev() {
        let mut current_tree = forest[x][forest[0].len() - 1];
        for y in (0..forest[0].len()).rev() {
            if x == forest.len() - 1 || y == forest[0].len() - 1 {
                hash.insert((x, y));
            } else if forest[x][y] > current_tree {
                hash.insert((x, y));
                current_tree = forest[x][y]
            }
            if current_tree == 9 {
                break;
            }
        }
    }
    //forwards veritcally
    for y in 0..forest[0].len() {
        let mut current_tree = forest[0][y];
        for x in 0..forest.len() {
            if x == 0 || y == 0 {
                hash.insert((x, y));
            } else if forest[x][y] > current_tree {
                hash.insert((x, y));
                current_tree = forest[x][y]
            }
            if current_tree == 9 {
                break;
            }
        }
    }
    //backwards vertically
    for y in (0..forest[0].len()).rev() {
        let mut current_tree = forest[forest.len() - 1][y];
        for x in (0..forest.len()).rev() {
            if x == forest.len() - 1 || y == forest[0].len() - 1 {
                hash.insert((x, y));
            } else if forest[x][y] > current_tree {
                hash.insert((x, y));
                current_tree = forest[x][y]
            }
            if current_tree == 9 {
                break;
            }
        }
    }
    Some(hash.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut forest: Vec<Vec<u32>> = vec![];
    for line in lines {
        forest.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect())
    }
    let mut max_tree_score = 0;
    let vectors: Vec<(i32, i32)> = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];
    for x in 1..forest.len() - 1 {
        for y in 1..forest[0].len() - 1 {
            let mut current_tree_score = 1;
            for vec in &vectors {
                let mut distance = 1;
                let mut current_x = x as i32 + vec.0;
                let mut current_y = y as i32 + vec.1;
                loop {
                    if forest[x][y] <= forest[current_x as usize][current_y as usize]
                        || current_x == 0
                        || current_y == 0
                        || current_x == (forest.len() - 1) as i32
                        || current_y == (forest[0].len() - 1) as i32
                    {
                        break;
                    } else {
                        distance += 1;
                        current_x += vec.0;
                        current_y += vec.1;
                    }
                }
                current_tree_score *= distance
            }
            if current_tree_score > max_tree_score {
                max_tree_score = current_tree_score
            }
        }
    }
    Some(max_tree_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
