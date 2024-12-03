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
}

pub fn part2() {
    let input_str = include_str!("./real_input.txt").trim();
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|(do)(\(\))|(don't)(\(\))").unwrap();
    let mut enabled = true;
    let sum = regex
        .captures_iter(input_str)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            if a == "do" {
                enabled = true;
                return 0;
            }
            if a == "don't" {
                enabled = false;
                return 0;
            }
            if !enabled {
                return 0;
            }
            return a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
        })
        .sum::<i32>();
    dbg!(sum);
}

pub fn combined() {
    let input_str = include_str!("./real_input.txt").trim();
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|(do)(\(\))|(don't)(\(\))").unwrap();
    let mut enabled = true;
    let mut part_1_sum = 0;
    let part_2_sum = regex
        .captures_iter(input_str)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            if a == "do" {
                enabled = true;
                return 0;
            }
            if a == "don't" {
                enabled = false;
                return 0;
            }

            let res = a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
            part_1_sum += res;

            if !enabled {
                return 0;
            }

            return res;
        })
        .sum::<i32>();
    dbg!(part_1_sum);
    dbg!(part_2_sum);
}
