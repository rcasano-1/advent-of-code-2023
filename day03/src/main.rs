use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

fn read_schematic(input: &str) -> Vec<Vec<char>> {
    let mut updated_schematic = Vec::new();
    let mut lines = input.lines();

    // read first line so we know how long all lines are
    // and can make a margin of dots where the first and last lines
    // are just dots.
    let first_line = lines.next().unwrap();
    let top_and_bottom_margin = vec!['.'; first_line.len() + 2];

    // add the margin at the top of the schematic
    updated_schematic.push(top_and_bottom_margin.clone());

    // add margin of one char, dot, to the left and right of first line
    let padded_first_line = once('.')
        .chain(first_line.chars())
        .chain(once('.'))
        .collect();
    updated_schematic.push(padded_first_line);

    // add margin to all other lines
    for line in lines {
        let padded_line = once('.').chain(line.chars()).chain(once('.')).collect();
        updated_schematic.push(padded_line);
    }

    // add the margin at the bottom of the schematic
    updated_schematic.push(top_and_bottom_margin);
    updated_schematic
}

#[allow(dead_code)]
fn print_schematic(schematic: &Vec<Vec<char>>) {
    for row in schematic {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

// direction to look for adjacent symbols
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    None,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::None => (0, 0),
            Direction::N => (1, 0),
            Direction::NE => (1, 1),
            Direction::E => (0, 1),
            Direction::SE => (-1, 1),
            Direction::S => (-1, 0),
            Direction::SW => (-1, -1),
            Direction::W => (0, -1),
            Direction::NW => (1, -1),
        }
    }

    fn next(&self) -> Direction {
        match self {
            Direction::None => Direction::N,
            Direction::N => Direction::NE,
            Direction::NE => Direction::E,
            Direction::E => Direction::SE,
            Direction::SE => Direction::S,
            Direction::S => Direction::SW,
            Direction::SW => Direction::W,
            Direction::W => Direction::NW,
            Direction::NW => Direction::None,
        }
    }
}

fn is_adjacent_to_symbol(schematic: &[Vec<char>], row: usize, col: usize) -> bool {
    fn is_symbol(c: char) -> bool {
        c != '.' && !c.is_ascii_digit()
    }

    // Start looking above current position, N direction, and look clockwise until we are back at N.
    let mut direction = Direction::N;
    while direction != Direction::None {
        let (row_offset, col_offset) = direction.offset();
        let neighbor_row = row as i32 + row_offset;
        let neighbor_col = col as i32 + col_offset;

        if is_symbol(schematic[neighbor_row as usize][neighbor_col as usize]) {
            return true;
        }

        direction = direction.next();
    }
    false
}

fn find_part_numbers(schematic: &Vec<Vec<char>>) -> Vec<u32> {
    let mut part_nums = Vec::new();
    let mut num_buf = String::new();
    let mut is_part_number = false;

    for row in 0..schematic.len() {
        for col in 0..schematic[row].len() {
            let cur_char = schematic[row][col];
            if cur_char.is_ascii_digit() {
                num_buf.push(schematic[row][col]);
                is_part_number |= is_adjacent_to_symbol(schematic, row, col);
            } else {
                // we are at the end of a number.
                // if it's a part number, we'll add it to the list
                // and then go looking for the next number
                if is_part_number {
                    part_nums.push(num_buf.parse::<u32>().unwrap());
                }
                num_buf.clear();
                is_part_number = false;
            }
        }
    }
    part_nums
}

fn part_1() {
    let input = include_str!("my_input.txt");
    let schematic = &read_schematic(input);

    let sum: u32 = find_part_numbers(schematic).iter().sum();
    println!("Part 1 Sum: {}", sum);
}

fn find_adjacent_gear_symbol(
    schematic: &[Vec<char>],
    row: usize,
    col: usize,
) -> Option<(usize, usize)> {
    fn is_gear_symbol(c: char) -> bool {
        c == '*'
    }

    let mut direction = Direction::N;
    while direction != Direction::None {
        let (row_offset, col_offset) = direction.offset();
        let neighbor_row = (row as i32 + row_offset) as usize;
        let neighbor_col = (col as i32 + col_offset) as usize;

        if is_gear_symbol(schematic[neighbor_row][neighbor_col]) {
            return Some((neighbor_row, neighbor_col));
        }

        direction = direction.next();
    }
    None
}

fn find_gear_part_nums(schematic: &Vec<Vec<char>>) -> Vec<(u32, u32)> {
    let mut gear_part_nums = Vec::new();
    let mut gear_loc_to_part_nums = HashMap::<(usize, usize), Vec<_>>::new();

    let mut num_buf = String::new();
    let mut is_part_number = false;

    let mut gear_locs = HashSet::new();

    for row in 0..schematic.len() {
        for col in 0..schematic[row].len() {
            let cur_char = schematic[row][col];
            if cur_char.is_ascii_digit() {
                num_buf.push(schematic[row][col]);
                is_part_number |= is_adjacent_to_symbol(schematic, row, col);
                if let Some(gear_loc) = find_adjacent_gear_symbol(schematic, row, col) {
                    gear_locs.insert(gear_loc);
                }
            } else {
                // we are at the end of a number.
                // if it's a gear, we'll add it to the list
                // and then go looking for the next number
                if is_part_number && !gear_locs.is_empty() {
                    let part_num = num_buf.parse::<u32>().unwrap();
                    gear_locs.iter().for_each(|loc| {
                        gear_loc_to_part_nums
                            .entry(*loc)
                            .or_default()
                            .push(part_num);
                    });
                }
                num_buf.clear();
                gear_locs.clear();
                is_part_number = false;
            }
        }
    }
    gear_loc_to_part_nums.iter().for_each(|(_, part_nums)| {
        if part_nums.len() == 2 {
            gear_part_nums.push((part_nums[0], part_nums[1]));
        }
        if part_nums.len() > 2 {
            println!("found a gear with more than 2 part nums: {:?}", part_nums);
        }
    });
    gear_part_nums
}

fn part_2() {
    let input = include_str!("my_input.txt");
    let schematic = read_schematic(input);

    let gear_part_nums = find_gear_part_nums(&schematic);
    let gear_ratios = gear_part_nums
        .iter()
        .map(|(a, b)| (*a as u64) * (*b as u64));

    let sum = gear_ratios.sum::<u64>();
    println!("Part 2 Sum: {}", sum);
}

fn main() {
    part_1();
    part_2();
}
