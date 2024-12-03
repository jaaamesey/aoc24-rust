extern crate hashbrown;

pub fn part1() {
    let input_str = include_str!("./real_input.txt");

    let row_strs = input_str.lines();

    let mut rows_a = Vec::<i32>::new();
    let mut rows_b = Vec::<i32>::new();
    for row_str in row_strs {
        let mut split = row_str
            .trim()
            .split_whitespace()
            .map(|v| v.parse::<i32>().unwrap());
        rows_a.push(split.next().unwrap());
        rows_b.push(split.next().unwrap());
    }

    rows_a.sort();
    rows_b.sort();

    let output = rows_a
        .iter()
        .enumerate()
        .map(|(i, a)| (a - rows_b[i]).abs())
        .sum::<i32>();

    dbg!(output);
}

pub fn part2() {}
