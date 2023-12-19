use bitvec::{bitvec, vec::BitVec};
use itertools::Itertools;
use nd_vec::{vector, Vec2};

type Pos = Vec2<usize>;
struct Galaxies {
    galaxies: Vec<Pos>,
    rows: BitVec,
    cols: BitVec,
}

fn parse(input: &str) -> Galaxies {
    let lines = input.lines().collect_vec();
    let mut galaxies = Vec::new();

    let mut rows = bitvec![0; lines[0].len()];
    let mut cols = bitvec![0; lines.len()];

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(vector![x, y]);
                rows.set(y, true);
                cols.set(x, true);
            }
        }
    }

    Galaxies {
        galaxies,
        rows,
        cols,
    }
}

impl Galaxies {
    fn expand(&mut self, mut multiplier: usize) {
        multiplier -= 1;

        for (y, _) in self.rows.iter().enumerate().rev().filter(|x| !x.1.as_ref()) {
            for pos in self.galaxies.iter_mut().filter(|pos| pos.y() > y) {
                *pos += vector!(0, multiplier);
            }
        }

        for (x, _) in self.cols.iter().enumerate().rev().filter(|x| !x.1.as_ref()) {
            for pos in self.galaxies.iter_mut().filter(|pos| pos.x() > x) {
                *pos += vector!(multiplier, 0);
            }
        }
    }

    fn total_distance(&self) -> usize {
        self.galaxies
            .iter()
            .map(|x| x.num_cast::<isize>().unwrap())
            .tuple_combinations()
            .map(|(a, b)| a.manhattan_distance(&b) as usize)
            .sum()
    }
}

fn part_1() {
    let (input, expected_total) = (include_str!("input.txt"), Some(374));

    let mut galaxies = parse(input);
    galaxies.expand(2);
    println!("Part 1: {}", galaxies.total_distance());

    if let Some(expected_total) = expected_total {
        assert_eq!(galaxies.total_distance(), expected_total);
    }
}

fn part_2() {
    let (input, expected_total) = (include_str!("input.txt"), Some(82000210));

    let mut galaxies = parse(input);
    galaxies.expand(1000000);
    println!("Part 2: {}", galaxies.total_distance());

    if let Some(expected_total) = expected_total {
        assert_eq!(galaxies.total_distance(), expected_total);
    }
}

fn main() {
    part_1();
    part_2();
}
