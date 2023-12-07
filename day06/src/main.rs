// #[derive(Debug)]
// struct Race {
//     time: u64,
//     distance: u64,
// }

use core::time;
use std::iter::zip;

fn ways_to_win(race_time: u64, record_distance: u64) -> u64 {
    let a = ((race_time * race_time - 4 * record_distance) as f64).sqrt();
    let x1 = (((race_time as f64 - a) / 2.0) + 1.0).floor();
    let x2 = (((race_time as f64 + a) / 2.0) - 1.0).ceil();
    (x2 - x1 + 1.0) as u64
}

fn part_1() {
    fn parse_line(line: &str) -> Vec<u64> {
        line.split(':')
            .nth(1)
            .expect("Should have parsed times or distances")
            .split_whitespace()
            .flat_map(|n| n.parse().ok())
            .collect()
    }

    fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
        let mut lines = input.lines();
        let times = parse_line(lines.next().expect("Should have parsed times"));
        let distances = parse_line(lines.next().expect("Should have parsed distances"));
        (times, distances)
    }

    let input = include_str!("my_input.txt");

    let (times, distances) = parse_input(input);

    let races = zip(times.iter(), distances.iter());

    let product = races.map(|(t, d)| ways_to_win(*t, *d)).product::<u64>();

    println!("Part 1 Product: {}", product);
}

fn part_2() {
    fn parse_line(line: &str) -> u64 {
        let (_title, nums) = line
            .split_once(':')
            .expect("Should have parsed times or distances");
        nums.replace(' ', "")
            .parse()
            .expect("Should have found a single number")
    }

    fn parse_input(input: &str) -> (u64, u64) {
        let mut lines = input.lines();
        let times = parse_line(lines.next().expect("Should have parsed times"));
        let distances = parse_line(lines.next().expect("Should have parsed distances"));
        (times, distances)
    }

    let input = include_str!("my_input.txt");

    let (time, distance) = parse_input(input);

    let ways_to_win = ways_to_win(time, distance);

    println!("Part 2 Ways to Win: {}", ways_to_win);
}

fn main() {
    part_1();
    part_2();
}
