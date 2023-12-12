use std::collections::HashMap;

#[cfg(windows)]
const BLANK_LINE: &str = "\r\n\r\n";

#[cfg(not(windows))]
const BLANK_LINE: &str = "\n\n";

#[derive(Debug)]
struct Map<'a> {
    // Char array of 'L's and 'R's representing the directions
    instructions: &'a [u8],
    // Node => (Left, Right)
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Map<'a> {
    fn get(&self, pos: &'a str, i: usize) -> &'a str {
        let (left, right) = self.nodes.get(pos).unwrap();
        match self.instructions[i % self.instructions.len()] as char {
            'L' => left,
            'R' => right,
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Map {
    let (instructions, node_list) = input.split_once(BLANK_LINE).unwrap();

    let mut nodes = HashMap::new();

    for node in node_list.lines() {
        let (id, children) = node.split_once(" = ").unwrap();

        let children = children
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .unwrap();

        nodes.insert(id, children);
    }

    Map {
        instructions: instructions.as_bytes(),
        nodes,
    }
}

/// Start at `"AAA"` and follow the instructions until you reach `"ZZZ"`.
fn part_1() {
    let input = include_str!("my_input.txt");
    let map = parse(input);

    let mut i = 0;
    let mut cur_pos = "AAA";

    loop {
        cur_pos = map.get(cur_pos, i);
        i += 1;

        if cur_pos == "ZZZ" {
            break;
        }
    }

    println!("Part 1 - Steps: {}", i);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

/// Get the cycle length for each starting position (ends with `"A"`), this is the number of positions you need to get from `"AAA"` to `"ZZZ"`.
///
/// Calculate the least common multiple (LCM) of all cycle lengths to get the number of steps needed to get from `"AAA"` to `"ZZZ"` for all starting positions.
fn part_2() {
    let input = include_str!("my_input.txt");
    let map = parse(input);

    let mut pos = Vec::new();
    for &id in map.nodes.keys() {
        if id.ends_with('A') {
            pos.push(id);
        }
    }

    let mut cycle_lengths = Vec::new();

    for mut pos in pos {
        let mut cycle_len = 0;
        let mut i = 0;

        loop {
            pos = map.get(pos, i);
            i += 1;

            cycle_len += 1;
            if pos.ends_with('Z') {
                cycle_lengths.push(cycle_len);
                break;
            }
        }
    }

    let steps = cycle_lengths.into_iter().reduce(lcm).unwrap();

    println!("Part 2 - Steps: {}", steps);
}

fn main() {
    part_1();
    part_2();
}
