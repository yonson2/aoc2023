// Unwraps scattered over because we know our input doesn't contain things we don't expect.
// (handle errors when working working with production code).

use std::collections::HashMap;

pub fn solve(data: Vec<String>) {
    println!("Day 1, part one: {}", part_one(&data));
    println!("Day 1, part two: {}", part_two(&data));
}

fn part_one(input: &[String]) -> u32 {
    input
        .into_iter()
        .map(|l| {
            l.chars()
                .filter_map(|a| a.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|l| {
            format!("{}{}", l.first().unwrap(), l.last().unwrap())
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

fn part_two(input: &[String]) -> u32 {
    let correct_input: Vec<String> = input
        .into_iter()
        .map(|l| replace_text_with_numeric_form(l.clone()))
        .collect();

    part_one(&correct_input)
}

// for each string, find all the occurrences of each replacement and then replace the one with
// the lowest index. Repeat this until no finds.
fn replace_text_with_numeric_form(text: String) -> String {
    let replacements: HashMap<&str, &str> = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut finds = Vec::new();
    for (spelled_digit, _) in replacements.clone() {
        if let Some(find_index) = text.find(spelled_digit) {
            finds.push((spelled_digit, find_index))
        }
    }
    if finds.is_empty() {
        return text;
    } else {
        finds.sort_by(|a, b| a.1.cmp(&b.1));
        let word_to_replace = finds[0].0;
        let new_text = text.replacen(word_to_replace, replacements[word_to_replace], 1);
        return replace_text_with_numeric_form(new_text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();

        assert_eq!(part_one(&input), 142);
    }

    #[test]
    fn test_part_two() {
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

        assert_eq!(part_two(&input), 295);
    }

    #[test]
    fn test_replace() {
        let input = String::from("eightwothree");
        assert_eq!(replace_text_with_numeric_form(input), String::from("8wo3"));
    }
}
