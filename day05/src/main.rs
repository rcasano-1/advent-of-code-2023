use rayon::{iter::ParallelIterator, slice::ParallelSlice};

#[cfg(windows)]
const BLANK_LINE: &str = "\r\n\r\n";

#[cfg(not(windows))]
const BLANK_LINE: &str = "\n\n";

#[derive(Debug, Clone)]
struct Range {
    end: u64,
    start: u64,
    length: u64,
}

impl Range {
    fn start_contains(&self, value: u64) -> bool {
        self.start <= value && value < self.start + self.length
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn map(&self, value: u64) -> u64 {
        for range in &self.ranges {
            if range.start_contains(value) {
                return range.end + value - range.start;
            }
        }
        value
    }
}

#[derive(Debug)]
struct AlmanacParseResult {
    maps: Vec<Map>,
    seeds: Vec<u64>,
}

fn parse_almanac(input: &str) -> AlmanacParseResult {
    let mut maps = Vec::new();

    let mut map_sections = input.split(BLANK_LINE);

    let seeds = map_sections
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    for map_section in map_sections.filter(|s| !s.is_empty()) {
        let lines = map_section.lines();
        let mut ranges = Vec::new();

        for line in lines.skip(1) {
            let mut parts = line.split_whitespace();

            let end = parts.next().unwrap().parse().unwrap();
            let start = parts.next().unwrap().parse().unwrap();
            let length = parts.next().unwrap().parse().unwrap();

            ranges.push(Range { start, end, length });
        }

        maps.push(Map { ranges });
    }

    AlmanacParseResult { maps, seeds }
}

fn part_1() {
    let input = include_str!("my_input.txt");

    let almanac = parse_almanac(input);

    let mut min = u64::MAX;
    for mut seed in almanac.seeds {
        for map in &almanac.maps {
            seed = map.map(seed);
        }
        min = min.min(seed);
    }

    println!("Part 1 Min: {}", min);
}

fn part_2() {
    let input = include_str!("my_input.txt");

    let almanac = parse_almanac(input);

    let min = almanac
        .seeds
        .par_chunks_exact(2)
        .map(|seed| {
            let mut min = u64::MAX;
            for mut seed in seed[0]..=(seed[0] + seed[1]) {
                for map in &almanac.maps {
                    seed = map.map(seed);
                }

                min = min.min(seed);
            }
            min
        })
        .min()
        .unwrap();

    println!("Part 2 Min: {}", min);
}

fn main() {
    part_1();
    part_2();
}
