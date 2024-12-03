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

pub fn part2() {}
