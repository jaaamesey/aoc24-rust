use std::collections::HashMap;

pub fn part1() {
    let input_str = include_str!("./real_input.txt");

    let row_strs = input_str.lines();

    let mut rows_a = Vec::<i32>::new();
    let mut rows_b = Vec::<i32>::new();
    for row_str in row_strs {
        let mut split = row_str.trim().split_whitespace();
        rows_a.push(split.next().unwrap().parse::<i32>().unwrap());
        rows_b.push(split.next().unwrap().parse::<i32>().unwrap());
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

pub fn part2() {
    let input_str = include_str!("./real_input.txt");

    let row_strs = input_str.lines();

    let mut rows_a = Vec::<i32>::new();
    let mut rows_b = Vec::<i32>::new();
    for row_str in row_strs {
        let mut split = row_str.trim().split_whitespace();
        rows_a.push(split.next().unwrap().parse::<i32>().unwrap());
        rows_b.push(split.next().unwrap().parse::<i32>().unwrap());
    }

    let mut b_counts = HashMap::<i32, i32>::new();
    for b in rows_b.iter() {
        let curr = b_counts.get(&b).unwrap_or(&0);
        b_counts.insert(*b, curr + 1);
    }

    let mut total_score = 0;
    for a in rows_a {
        let score = a * b_counts.get(&a).unwrap_or(&0);
        total_score += score;
    }

    let output = total_score;

    dbg!(output);
}
