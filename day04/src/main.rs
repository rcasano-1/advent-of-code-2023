struct Card {
    wins: u8,
}

fn parse(input: &str) -> Vec<Card> {
    let mut cards = Vec::new();

    for line in input.lines() {
        // Split the line and throw away the card number, then split on the winning numbers and the scratch numbers
        let (_, line) = line.split_once(": ").unwrap();
        let (winning_nums, scratched) = line.split_once(" | ").unwrap();

        let parse = |s: &str| {
            s.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<u8>>()
        };

        let winning_nums = parse(winning_nums);
        let scratched = parse(scratched);

        cards.push(Card {
            wins: scratched
                .iter()
                .filter(|matching_num| winning_nums.contains(matching_num))
                .count() as u8,
        });
    }
    cards
}

fn part_1() {
    let input = include_str!("my_input.txt");

    let cards = parse(input);

    // How else can I do this without using pow?
    let sum = cards
        .iter()
        .filter(|c| c.wins > 0)
        .map(|c| 2u32.pow(c.wins.saturating_sub(1) as u32))
        .sum::<u32>();

    println!("Part 1 Sum: {}", sum);
}

fn main() {
    part_1();
}
