fn main() {
    // Part 1
    let input = include_str!("my_part1_input.txt");

    let mut sum = 0;

    for line in input.lines() {
        let mut digits = line.chars().filter_map(|c| c.to_digit(10));

        let first = digits.next().unwrap();

        let last = digits.last().unwrap_or(first);

        sum += first * 10 + last;
    }
    println!("Sum: {}", sum);

    ///////////////////////////////////////////////////////////////////////////////////////////////

    // Part 2
    let input = include_str!("my_part2_input.txt");

    let mut sum = 0;

    for line in input.lines() {
        let mut first_position = None;
        let mut first_digit = None;
        let mut last_position = None;
        let mut last_digit = None;

        for digit in DIGITS_AS_STR {
            if let Some(position) = line.find(digit) {
                if position <= first_position.unwrap_or(position) {
                    first_position = Some(position);
                    first_digit = Some(digit);
                }
            };

            if let Some(position) = line.rfind(digit) {
                if position >= last_position.unwrap_or(position) {
                    last_position = Some(position);
                    last_digit = Some(digit);
                }
            };
        }

        // dbg!(first_digit, last_digit);

        let first_digit = first_digit.map(digit_to_number).unwrap();
        let last_digit = last_digit.map(digit_to_number).unwrap();

        dbg!(first_digit, last_digit);

        sum += first_digit * 10 + last_digit;

        dbg!(sum);
    }

    println!("Part 2 Sum: {}", sum);
}

const DIGITS_AS_STR: [&str; 18] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
    "5", "6", "7", "8", "9",
];

fn digit_to_number(digit: &str) -> u32 {
    match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        n => n.parse::<u32>().unwrap(),
    }
}
