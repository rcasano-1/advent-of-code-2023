// 12 red cubes, 13 green cubes, and 14 blue cubes
const MAX_CUBES: [u32; 3] = [12, 13, 14];

fn main() {
    // Part 1
    let input = include_str!("my_input.txt");

    let sum = parse(input)
        .iter()
        .enumerate()
        .filter(|(_, games)| games.iter().all(|game| game.is_possible()))
        .map(|x| x.0 + 1)
        .sum::<usize>();

    println!("Part 1: {}", sum);

    ////////////////////////////////////////////////////////////////////////////////////////////////

    // Part 2
    let sum2 = parse(input)
        .iter()
        .map(|games| {
            let mut max = CubeSet::default();
            for game in games {
                max = max.max(game);
            }
            max.red * max.green * max.blue
        })
        .sum::<u32>();

    println!("Part 2: {}", sum2);
}

fn parse(input: &str) -> Vec<Vec<CubeSet>> {
    input
        .lines()
        .map(|line| {
            let cubes = line.split_once(':').unwrap().1;

            let mut sets = Vec::new();
            for game_set in cubes.split(';') {
                let mut cubes = CubeSet::default();
                for i in game_set.split(',') {
                    let mut iter = i.split_whitespace();
                    let count = iter.next().unwrap().parse::<u32>().unwrap();
                    let color = iter.next().unwrap();

                    match color {
                        "red" => cubes.red += count,
                        "green" => cubes.green += count,
                        "blue" => cubes.blue += count,
                        _ => unreachable!(),
                    }
                }
                sets.push(cubes);
            }

            sets
        })
        .collect()
}

#[derive(Debug, Default)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    fn max(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn is_possible(&self) -> bool {
        self.red <= MAX_CUBES[0] && self.green <= MAX_CUBES[1] && self.blue <= MAX_CUBES[2]
    }
}
