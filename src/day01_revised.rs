use aho_corasick::AhoCorasick;

const PATTERN: &[&str; 18] = &[
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

pub fn solve(data: Vec<String>) {
    println!(
        "Day 1 Revised, part one: {}",
        get_sum(&data, &PATTERN[0..9])
    );
    println!("Day 1 Revised, part two: {}", get_sum(&data, PATTERN));
}

fn get_sum(input: &[String], pattern: &[&str]) -> u32 {
    let ac = AhoCorasick::new(pattern).expect("Invalid pattern");

    input.iter().fold(0, |acc, curr| {
        let mut iter = ac.find_overlapping_iter(curr).peekable();
        let left = match iter
            .peek()
            .expect("must have a first digit")
            .pattern()
            .as_u32()
        {
            i if i < 9 => i + 1,
            i if i >= 9 => i - 8,
            _ => unreachable!("Should never happen."),
        };
        let right = match iter
            .last()
            .expect("must have a first digit")
            .pattern()
            .as_u32()
        {
            i if i < 9 => i + 1,
            i if i >= 9 => i - 8,
            _ => unreachable!(),
        };
        let number = format!("{}{}", left, right)
            .parse::<u32>()
            .expect("must be a number");

        acc + number
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_revised() {
        let input = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();

        assert_eq!(get_sum(&input, &PATTERN[0..9]), 142);
    }

    #[test]
    fn test_part_two_revised() {
        let input = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
            "onetwothreefour",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>();

        assert_eq!(get_sum(&input, PATTERN), 295);
    }
}
