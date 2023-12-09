pub fn solve(input: Vec<String>) {
    let data = parse_input(input);
    println!("Day 9, part one: {}", part_one(data.clone()));
    println!("Day 9, part two: {}", part_two(data));
}

fn part_one(data: Vec<Vec<isize>>) -> isize {
    data.into_iter().map(get_next_value).sum()
}

fn part_two(data: Vec<Vec<isize>>) -> isize {
    data.into_iter()
        .map(|l| get_next_value(l.into_iter().rev().collect()))
        .sum()
}

fn get_next_value(seq: Vec<isize>) -> isize {
    let diff: Vec<isize> = seq.windows(2).map(|a| a[1] - a[0]).collect();
    let &last_value = seq.last().expect("a value");
    match diff.iter().all(|&n| n == 0) {
        true => last_value,
        false => last_value + get_next_value(diff),
    }
}

fn parse_input(input: Vec<String>) -> Vec<Vec<isize>> {
    input
        .iter()
        .map(|s| {
            s.split(' ')
                .map(|x| x.parse::<isize>().expect("valid number"))
                .collect()
        })
        .collect::<Vec<Vec<isize>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_9_part_one() {
        let input = ["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"]
            .into_iter()
            .map(String::from)
            .collect();

        let data = parse_input(input);

        assert_eq!(part_one(data), 114);
    }

    #[test]
    fn test_day_9_part_two() {
        let input = ["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"]
            .into_iter()
            .map(String::from)
            .collect();

        let data = parse_input(input);

        assert_eq!(part_two(data), 2);
    }
}
