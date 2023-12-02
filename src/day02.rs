use lazy_regex::regex_captures;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn solve(data: Vec<String>) {
    println!("Day 2, part one: {}", part_one(&data));
    println!("Day 2, part two: {}", part_two(&data));
}

fn part_one(input: &[String]) -> u32 {
    let games: Vec<Game> = input
        .into_iter()
        .map(parse_input)
        .filter(|g| {
            g.rounds.iter().fold(true, |acc, curr| {
                if !acc {
                    return acc;
                }

                if curr.red > MAX_RED || curr.green > MAX_GREEN || curr.blue > MAX_BLUE {
                    return false;
                }

                acc
            })
        })
        .collect();

    games.iter().map(|g| g.id).sum()
}

fn part_two(input: &[String]) -> u32 {
    input
        .into_iter()
        .map(parse_input)
        .map(|g| {
            let minimum_round = g.rounds.iter().fold(
                Round {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |mut acc, curr| {
                    if curr.red > acc.red {
                        acc.red = curr.red
                    }
                    if curr.blue > acc.blue {
                        acc.blue = curr.blue
                    }
                    if curr.green > acc.green {
                        acc.green = curr.green
                    }
                    acc
                },
            );

            minimum_round
        })
        .map(|r| r.red * r.green * r.blue)
        .sum()
}

fn parse_input(line: &String) -> Game {
    let split = line.split(":").collect::<Vec<&str>>();
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
        .split(";")
        .map(str::trim)
        .map(|r| {
            let mut round = Round {
                red: 0,
                green: 0,
                blue: 0,
            };

            let red = regex_captures!(r#"(\d+) red"#, r);
            let green = regex_captures!(r#"(\d+) green"#, r);
            let blue = regex_captures!(r#"(\d+) blue"#, r);

            if let Some((_, red_cubes)) = red {
                round.red = red_cubes.parse::<u32>().expect("valid cube number");
            }

            if let Some((_, green_cubes)) = green {
                round.green = green_cubes.parse::<u32>().expect("valid cube number");
            }

            if let Some((_, blue_cubes)) = blue {
                round.blue = blue_cubes.parse::<u32>().expect("valid cube number");
            }

            round
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
        .collect::<Vec<String>>();

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
        .collect::<Vec<String>>();

        assert_eq!(part_two(&input), 2286);
    }
}
