use lazy_regex::regex_captures;
use std::cmp;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn solve(data: Vec<String>) {
    let parsed_input: Vec<Game> = data.iter().map(parse_line).collect();

    println!("Day 2, part one: {}", part_one(&parsed_input));
    println!("Day 2, part two: {}", part_two(&parsed_input));
}

fn part_one(games: &[Game]) -> u32 {
    let games: Vec<&Game> = games
        .iter()
        .filter(|g| {
            g.rounds.iter().fold(true, |acc, curr| {
                acc && (curr.red <= MAX_RED && curr.green <= MAX_GREEN && curr.blue <= MAX_BLUE)
            })
        })
        .collect();

    games.iter().map(|g| g.id).sum()
}

fn part_two(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|g| {
            g.rounds.iter().fold(
                Round {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |acc, curr| Round {
                    red: cmp::max(acc.red, curr.red),
                    green: cmp::max(acc.green, curr.green),
                    blue: cmp::max(acc.blue, curr.blue),
                },
            )
        })
        .fold(0, |acc, curr| acc + curr.red * curr.green * curr.blue)
}

fn parse_line(line: &String) -> Game {
    let split = line.split(':').collect::<Vec<&str>>();
    let (game, rounds) = split.split_at(1);

    let id = game // getting the id like this is not good but its fun.
        .first()
        .expect("game id missing")
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<u32>()
        .expect("valid game id");

    let rounds = rounds
        .first()
        .expect(&format!("missing rounds for game {}", id))
        .split(';')
        .map(str::trim)
        .map(|r| {
            let (_, red) = regex_captures!(r#"(\d+) red"#, r).unwrap_or(("", "0"));
            let (_, green) = regex_captures!(r#"(\d+) green"#, r).unwrap_or(("", "0"));
            let (_, blue) = regex_captures!(r#"(\d+) blue"#, r).unwrap_or(("", "0"));

            let (red, green, blue) = (
                red.parse::<u32>().expect("valid red cube number"),
                green.parse::<u32>().expect("valid green cube number"),
                blue.parse::<u32>().expect("valid blue cube number"),
            );

            Round { red, green, blue }
        })
        .collect::<Vec<Round>>();

    Game { id, rounds }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_2_part_one() {
        let input = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .into_iter()
        .map(String::from)
        .map(|c| parse_line(&c))
        .collect::<Vec<Game>>();

        assert_eq!(part_one(&input), 8);
    }

    #[test]
    fn day_2_part_two() {
        let input = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .into_iter()
        .map(String::from)
        .map(|c| parse_line(&c))
        .collect::<Vec<Game>>();

        assert_eq!(part_two(&input), 2286);
    }
}
