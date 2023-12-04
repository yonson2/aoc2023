// I tried to do this without a grid, all tests ended up passing but there is a bug somewhere.
// at the end, I gave up and used a 2d array.
// trait NotPeriod {
//     fn is_symbol(&self) -> bool;
// }
//
// impl NotPeriod for char {
//     fn is_symbol(&self) -> bool {
//         *self != '.'
//     }
// }
//
// #[derive(Debug, Clone)]
// enum Piece {
//     Number(char, Option<bool>),
//     Symbol(char),
//     Period,
// }
//
// pub fn solve(input: Vec<String>) {
//     let data = input
//         .into_iter()
//         .map(|l| parse_line(l))
//         .collect::<Vec<Vec<Piece>>>();
//
//     println!("Day 3, part one: {}", part_one(&data));
// }
//
// fn part_one(input: &[Vec<Piece>]) -> u32 {
//     let mut processed_schematic = Vec::new();
//     for (line_index, line) in input.iter().enumerate() {
//         // let check_left = i > 0; //BAD -> this should check the sub-index of v, not i.
//         // let check_right = i + 1 < v.len(); //BAD -> this should check the sub-index of v, not i.
//
//         let mut processed_schematic_line = Vec::new();
//         let check_above = line_index > 0;
//         let check_below = line_index + 1 < input.len();
//
//         for (i, v) in line.iter().enumerate() {
//             let check_left = i > 0;
//             let check_right = i + 1 < line.len();
//             let check_above_left = check_above && check_left;
//             let check_above_right = check_above && check_right;
//             let check_below_left = check_below && check_left;
//             let check_below_right = check_below && check_right;
//             match v {
//                 Piece::Number(n, _) => {
//                     if check_left {
//                         match line[i - 1] {
//                             Piece::Symbol(_) => {
//                                 processed_schematic_line.push(Piece::Number(n.clone(), Some(true)));
//                                 continue;
//                             }
//                             _ => (),
//                         };
//                     }
//
//                     if check_right {
//                         match line[i + 1] {
//                             Piece::Symbol(_) => {
//                                 processed_schematic_line.push(Piece::Number(n.clone(), Some(true)));
//                                 continue;
//                             }
//                             _ => (),
//                         };
//                     }
//
//                     if check_above {
//                         match input[line_index - 1][i] {
//                             Piece::Symbol(_) => {
//                                 processed_schematic_line.push(Piece::Number(n.clone(), Some(true)));
//                                 continue;
//                             }
//                             _ => (),
//                         }
//                     }
//
//                     if check_below {
//                         match input[line_index + 1][i] {
//                             Piece::Symbol(_) => {
//                                 processed_schematic_line.push(Piece::Number(n.clone(), Some(true)));
//                                 continue;
//                             }
//                             _ => (),
//                         }
//                     }
//
//                     if check_above_left {
//                         match input[line_index - 1][i - 1] {
//                             Piece::Symbol(_) => {
//                                 processed_schematic_line.push(Piece::Number(n.clone(), Some(true)));
//                                 continue;
//                             }
//                             _ => (),
//                         }
//                     }
//
//                     if check_above_right {
//                         match input[line_index - 1][i + 1] {
//                             Piece::Symbol(_) => {
//                                 processed_schematic_line.push(Piece::Number(n.clone(), Some(true)));
//                                 continue;
//                             }
//                             _ => (),
//                         }
//                     }
//
//                     if check_below_left {
//                         match input[line_index + 1][i - 1] {
//                             Piece::Symbol(_) => {
//                                 processed_schematic_line.push(Piece::Number(n.clone(), Some(true)));
//                                 continue;
//                             }
//                             _ => (),
//                         }
//                     }
//
//                     if check_below_right {
//                         match input[line_index + 1][i + 1] {
//                             Piece::Symbol(_) => {
//                                 processed_schematic_line.push(Piece::Number(n.clone(), Some(true)));
//                                 continue;
//                             }
//                             _ => (),
//                         }
//                     }
//                     processed_schematic_line.push(Piece::Number(n.clone(), Some(false)));
//                 }
//                 Piece::Symbol(x) => processed_schematic_line.push(Piece::Symbol(x.clone())),
//                 Piece::Period => processed_schematic_line.push(Piece::Period),
//             }
//         }
//         processed_schematic.push(processed_schematic_line.clone()); //TODO: maybe process the
//     }
//     // now take the numbers of the processed schematic.
//     processed_schematic
//         .into_iter()
//         .flat_map(|sl| get_schematic_line_numbers(sl))
//         .sum()
// }
//
// fn get_schematic_line_numbers(sl: Vec<Piece>) -> Vec<u32> {
//     let mut result = Vec::new();
//     let mut current_number = String::new();
//     let mut adjacent = false;
//
//     for (i, p) in sl.iter().enumerate() {
//         match p {
//             Piece::Number(n, maybe_a) => {
//                 current_number.push(n.clone());
//                 if let Some(true) = maybe_a {
//                     adjacent = true;
//                 }
//
//                 // last piece.
//                 if i + 1 == sl.len() {
//                     let n = current_number.parse().expect("valid number");
//                     result.push(n);
//                     current_number.clear();
//                     adjacent = false;
//                 }
//             }
//             _ => {
//                 if !current_number.is_empty() && adjacent {
//                     let n = current_number.parse().expect("valid number");
//                     result.push(n);
//                 }
//                 adjacent = false;
//                 current_number.clear();
//             }
//         }
//     }
//     result
// }
//
// fn parse_line(line: String) -> Vec<Piece> {
//     //TODO: ensure all have the same lenth?
//     line.chars()
//         .map(|c| match c {
//             c if c.is_numeric() => Piece::Number(c, None),
//             c if c == '.' => Piece::Period,
//             c => Piece::Symbol(c),
//         })
//         .collect()
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_day_3_part_one() {
//         let input = [
//             "467..114..",
//             "...*......",
//             "..35..633.",
//             "......#...",
//             "617*......",
//             ".....+.58.",
//             "..592.....",
//             "......755.",
//             "...$.*....",
//             ".664.598..",
//         ]
//         .into_iter()
//         .map(String::from)
//         .map(|l| parse_line(l))
//         .collect::<Vec<Vec<Piece>>>();
//
//         let input2 = [
//             "12.......*..",
//             "+.........34",
//             ".......-12..",
//             "..78........",
//             "***....60...",
//             "78.........9",
//             ".5.....23..$",
//             "8...90*12...",
//             "............",
//             "2.2......12.",
//             ".*.........*",
//             "1.1..503+.56",
//         ]
//         .into_iter()
//         .map(String::from)
//         .map(|l| parse_line(l))
//         .collect::<Vec<Vec<Piece>>>();
//
//         let input3 = [
//             "12.......*..",
//             "+.........34",
//             ".......-12..",
//             "..78........",
//             "..*....60...",
//             "78..........",
//             ".......23...",
//             "....90*12...",
//             "............",
//             "2.2......12.",
//             ".*.........*",
//             "1.1.......56",
//         ]
//         .into_iter()
//         .map(String::from)
//         .map(|l| parse_line(l))
//         .collect::<Vec<Vec<Piece>>>();
//
//         assert_eq!(part_one(&input), 4361);
//         assert_eq!(part_one(&input2), 925);
//         assert_eq!(part_one(&input3), 413);
//     }
// }
