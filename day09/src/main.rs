struct Sequence {
    values: Vec<i64>,
}

impl Sequence {
    fn derive(&self) -> Vec<Vec<i64>> {
        let mut derived = vec![self.values.clone()];

        while !derived.last().unwrap().iter().all(|&v| v == 0) {
            let last = derived.last().unwrap();
            let mut next = Vec::new();

            for i in 1..last.len() {
                next.push(last[i] - last[i - 1]);
            }

            derived.push(next);
        }
        derived
    }

    fn predict(&self) -> i64 {
        self.derive().iter().filter_map(|v| v.last()).sum()
    }

    fn reverse(mut self) -> Self {
        self.values.reverse();
        self
    }
}

fn parse(input: &str) -> Vec<Sequence> {
    let mut seq_vec = Vec::new();

    for line in input.lines() {
        let values = line
            .split_whitespace()
            .map(|v| v.parse::<i64>().unwrap())
            .collect();
        seq_vec.push(Sequence { values });
    }
    seq_vec
}

fn part_1() {
    let input = include_str!("my_input.txt");
    let seq_vec = parse(input);

    let mut sum = 0;
    for seq in seq_vec {
        sum += seq.predict();
    }
    println!("Part 1: {}", sum);
}

fn part_2() {
    let input = include_str!("my_input.txt");
    let seq_vec = parse(input);

    let mut sum = 0;
    for seq in seq_vec {
        sum += seq.reverse().predict();
    }
    println!("Part 2: {}", sum);
}

fn main() {
    part_1();
    part_2();
}
