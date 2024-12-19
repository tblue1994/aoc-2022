advent_of_code::solution!(7);

use regex::Regex;

#[derive(Debug, Default)]
struct ArenaTree<T>
where
    T: PartialEq,
{
    arena: Vec<Node<T>>,
}

impl<T> ArenaTree<T>
where
    T: PartialEq,
{
    fn node(&mut self, val: T, name: &str) -> usize {
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(Node::new(idx, val, name));
        idx
    }
}

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq,
{
    idx: usize,
    val: T,
    name: String,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, val: T, name: &str) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
            name: name.to_string(),
        }
    }
}

pub(crate) fn build_file_system(input: &str) -> ArenaTree<Option<u32>> {
    let instructions: Vec<&str> = input.lines().collect();
    let mut file_system = ArenaTree::<Option<u32>>::default();
    let mut current_directory = file_system.node(None, "/");
    let change_dir_regex = Regex::new(r"\$ cd (.+)").unwrap();
    let dir_name_regex = Regex::new(r"dir (.+)").unwrap();
    let file_name_regex = Regex::new(r"(\d+) (.+)").unwrap();
    for instr in instructions {
        if instr == "$ cd /" {
            current_directory = 0
        } else if instr == "$ cd .." {
            current_directory = file_system.arena[current_directory].parent.unwrap()
        } else if change_dir_regex.is_match(instr) {
            let cap = change_dir_regex.captures(instr).unwrap();

            current_directory = *file_system.arena[current_directory]
                .children
                .iter()
                .find(|n| file_system.arena[**n].name == cap[1])
                .unwrap()
        } else if dir_name_regex.is_match(instr) {
            let cap = dir_name_regex.captures(instr).unwrap();
            let dir_node = file_system.node(None, &cap[1]);
            file_system.arena[current_directory].children.push(dir_node);
            file_system.arena[dir_node].parent = Some(current_directory);
        } else if file_name_regex.is_match(instr) {
            let cap = file_name_regex.captures(instr).unwrap();
            let file_node = file_system.node(Some(cap[1].parse::<u32>().unwrap()), &cap[2]);
            file_system.arena[current_directory]
                .children
                .push(file_node);
            file_system.arena[file_node].parent = Some(current_directory);
        }
    }

    for ind in (0..file_system.arena.len()).rev() {
        if file_system.arena[ind].val.is_none() && !file_system.arena[ind].children.is_empty() {
            let mut value = 0;
            for i in file_system.arena[ind].children.iter() {
                value += file_system.arena[*i].val.unwrap();
            }
            file_system.arena[ind].val = Some(value)
        }
    }
    file_system
}

pub fn part_one(input: &str) -> Option<u32> {
    let file_system = build_file_system(input);
    let mut small_dir_total = 0;

    for node in file_system.arena {
        if !node.children.is_empty() && node.val.unwrap() <= 100000 {
            small_dir_total += node.val.unwrap()
        }
    }

    Some(small_dir_total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let file_system = build_file_system(input);
    const TOTAL_MEMORY: u32 = 70000000;
    const UPDATE_MEMORY: u32 = 30000000;
    let current_memory = file_system.arena[0].val.unwrap();
    let target_memory = UPDATE_MEMORY - (TOTAL_MEMORY - current_memory);
    let mut small_dir_total = current_memory;

    for node in file_system.arena {
        if !node.children.is_empty()
            && node.val.unwrap() >= target_memory
            && node.val.unwrap() <= small_dir_total
        {
            small_dir_total = node.val.unwrap()
        }
    }

    Some(small_dir_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
