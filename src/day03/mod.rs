extern crate hashbrown;
use regex::Regex;

pub fn part1() {
    let input_str = include_str!("./real_input.txt").trim();
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let sum = regex
        .captures_iter(input_str)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            return a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
        })
        .sum::<i32>();
    dbg!(sum);
    // let chars = input_str.chars();
    // let mut total = 0;
    //
    // let expected_prefix = "mul(";
    // let mut expression = "".to_string();
    // let mut expression_num_a = "".to_string();
    // let mut expression_num_b = "".to_string();
    // let mut on_num_b = false;
    //
    // for char in chars {
    //     expression.push(char);
    //     if !expected_prefix.starts_with(&expression) {
    //         expression = char.to_string();
    //         expression_num_a = "".to_string();
    //         expression_num_b = "".to_string();
    //         on_num_b = false;
    //         continue;
    //     }
    //     if expression.starts_with(expected_prefix) {
    //         dbg!(&expression, &char);
    //         if char == ',' {
    //             if on_num_b {
    //                 expression = char.to_string();
    //                 expression_num_a = "".to_string();
    //                 expression_num_b = "".to_string();
    //                 on_num_b = false;
    //                 continue;
    //             }
    //             on_num_b = true;
    //             continue;
    //         }
    //         if char == ')' {
    //             if !on_num_b {
    //                 expression = char.to_string();
    //                 expression_num_a = "".to_string();
    //                 expression_num_b = "".to_string();
    //                 on_num_b = false;
    //                 continue;
    //             }
    //             dbg!("YOO!");
    //             total += expression_num_a.parse::<i32>().unwrap()
    //                 * expression_num_b.parse::<i32>().unwrap();
    //             continue;
    //         }
    //         if !char.is_numeric() {
    //             expression = char.to_string();
    //             expression_num_a = "".to_string();
    //             expression_num_b = "".to_string();
    //             on_num_b = false;
    //             continue;
    //         }
    //         if on_num_b {
    //             expression_num_b.push(char);
    //         } else {
    //             expression_num_a.push(char);
    //         }
    //     }
    // }

    // let expected_mul = "mul(";
    // let mut maybe_mul: String = "".to_string();
    // let mut in_mul = false;
    // let mut stuff_in_mul: String = "".to_string();
    // let mut total = 0;
    // for char in chars {
    //     if in_mul {
    //         if char == ')' {
    //             let maybe_numbers = stuff_in_mul
    //                 .split(',')
    //                 .map(|s| s.parse::<i32>())
    //                 .collect::<Vec<_>>();
    //
    //             in_mul = false;
    //             stuff_in_mul = "".to_string();
    //             maybe_mul = "".to_string();
    //
    //             if maybe_numbers.len() != 2 || maybe_numbers.iter().any(|n| n.is_err()) {
    //                 dbg!("NOPE!", &maybe_numbers);
    //             } else {
    //                 dbg!(&maybe_numbers);
    //                 total += maybe_numbers
    //                     .iter()
    //                     .fold(1, |acc, next| acc * next.clone().unwrap());
    //             }
    //
    //             continue;
    //         }
    //
    //         if char != ',' && !char.is_numeric() {
    //             in_mul = false;
    //             stuff_in_mul = "".to_string();
    //             maybe_mul = "".to_string();
    //             dbg!("HEY");
    //         }
    //         stuff_in_mul.push(char);
    //         continue;
    //     }
    //     maybe_mul.push(char);
    //     if !expected_mul.starts_with(&maybe_mul) {
    //         maybe_mul = "".to_string();
    //         maybe_mul.push(char);
    //     }
    //     if maybe_mul == expected_mul {
    //         maybe_mul = "".to_string();
    //         in_mul = true;
    //     }
    // }
}

pub fn part2() {}
