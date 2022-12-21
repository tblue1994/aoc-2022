pub struct Node {
    name: String,
    value: Option<u128>,
    operator: Option<char>,
    sub_nodes: Option<Vec<String>>,
}

pub fn parse_input(input: &str) -> Vec<Node> {
    let mut nodes = vec![];
    input.lines().for_each(|l| {
        let mut s = l.split(": ");
        let node_name = s.next().unwrap();
        let stuff = s.next().unwrap();
        let back: Vec<&str> = stuff.split(' ').collect();
        let node: Node = if back.len() == 1 {
            //is number
            Node {
                name: node_name.to_string(),
                value: Some(back[0].parse::<u128>().unwrap()),
                operator: None,
                sub_nodes: None,
            }
        } else {
            //is parent
            Node {
                name: node_name.to_string(),
                value: None,
                operator: Some(back[1].chars().next().unwrap()),
                sub_nodes: Some(vec![back[0].to_owned(), back[2].to_owned()]),
            }
        };

        nodes.push(node)
    });
    nodes
}
pub fn get_value1(name: String, nodes: &[Node]) -> u128 {
    let node = nodes.iter().find(|n| n.name == name).unwrap();
    if let Some(v) = node.value {
        v
    } else {
        let mut values = node
            .sub_nodes
            .as_ref()
            .unwrap()
            .iter()
            .map(|s| get_value1(s.to_owned(), nodes));
        match node.operator {
            Some('*') => values.product(),
            Some('+') => values.sum(),
            Some('/') => values.next().unwrap() / values.next().unwrap(),
            Some('-') => values.next().unwrap() - values.next().unwrap(),
            _ => panic!("uh oh"),
        }
    }
}

pub fn get_value2(name: String, nodes: &[Node], missing: String) -> Option<u128> {
    if name == missing {
        return None;
    }

    let node = nodes.iter().find(|n| n.name == name).unwrap();
    if let Some(v) = node.value {
        Some(v)
    } else {
        let values: Vec<Option<u128>> = node
            .sub_nodes
            .as_ref()
            .unwrap()
            .iter()
            .map(|s| get_value2(s.to_owned(), nodes, missing.to_owned()))
            .collect();
        if values.iter().any(|v| v.is_none()) {
            return None;
        }
        let val = match node.operator {
            Some('*') => values[0].unwrap() * values[1].unwrap(),
            Some('+') => values[0].unwrap() + values[1].unwrap(),
            Some('/') => values[0].unwrap() / values[1].unwrap(),
            Some('-') => values[0].unwrap() - values[1].unwrap(),
            _ => panic!("uh oh"),
        };
        Some(val)
    }
}

pub fn solve_for(missing: String, root_name: String, nodes: &[Node]) -> u128 {
    let root = nodes.iter().find(|n| n.name == root_name).unwrap();
    let mut values = root
        .sub_nodes
        .as_ref()
        .unwrap()
        .iter()
        .map(|s| get_value2(s.to_owned(), nodes, missing.to_owned()));
    let first = values.next().unwrap();
    let second = values.next().unwrap();
    let subs = root.sub_nodes.as_ref().unwrap();
    if first.is_none() {
        solve_for_w_total(missing, subs[0].to_owned(), nodes, second.unwrap())
    } else {
        solve_for_w_total(missing, subs[1].to_owned(), nodes, first.unwrap())
    }
}

pub fn solve_for_w_total(missing: String, root_name: String, nodes: &[Node], total: u128) -> u128 {
    if missing == root_name {
        return total;
    }

    let root = nodes.iter().find(|n| n.name == root_name).unwrap();
    if let Some(v) = root.value {
        return v;
    }
    let mut values = root
        .sub_nodes
        .as_ref()
        .unwrap()
        .iter()
        .map(|s| get_value2(s.to_owned(), nodes, missing.to_owned()));
    let first = values.next().unwrap();
    let second = values.next().unwrap();
    let subs = root.sub_nodes.as_ref().unwrap();
    let new_total = calculate_total(
        root.operator,
        total,
        first.unwrap_or_else(|| second.unwrap()),
        first.is_none(),
    );
    if first.is_none() {
        solve_for_w_total(missing, subs[0].to_owned(), nodes, new_total)
    } else {
        solve_for_w_total(missing, subs[1].to_owned(), nodes, new_total)
    }
}

pub fn calculate_total(op: Option<char>, total: u128, sub_val: u128, is_first: bool) -> u128 {
    match op {
        Some('*') => total / sub_val,
        Some('+') => total - sub_val,
        Some('/') => {
            if is_first {
                total * sub_val
            } else {
                sub_val / total
            }
        }
        Some('-') => {
            if is_first {
                total + sub_val
            } else {
                sub_val - total
            }
        }
        _ => panic!("uh oh"),
    }
}

pub fn part_one(input: &str) -> Option<u128> {
    let nodes = parse_input(input);
    Some(get_value1("root".to_string(), &nodes))
}

pub fn part_two(input: &str) -> Option<u128> {
    let nodes = parse_input(input);
    Some(solve_for("humn".to_string(), "root".to_string(), &nodes))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
