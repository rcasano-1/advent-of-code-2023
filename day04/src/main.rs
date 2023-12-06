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

fn part_2() {
    let input = include_str!("my_input.txt");
    let cards = parse(input);

    let mut queue = (0..cards.len()).collect::<Vec<_>>();
    let mut visited = 0;

    while let Some(i) = queue.pop() {
        // Keep track of how many cards we've processed
        visited += 1;

        let card = &cards[i];
        // Only add the next card if it has a winning number
        if card.wins == 0 {
            continue;
        }

        // Add a new card for each winning number
        for j in 0..card.wins as usize {
            queue.push(j + i + 1);
        }
    }

    println!("Part 2 Visited: {}", visited);
}

fn main() {
    part_1();
    part_2();
}
