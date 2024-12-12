use hashbrown::HashMap;

pub fn do_the_thing(iterations: usize) {
    let mut stones: HashMap<usize, usize> = HashMap::new();
    include_str!("./real_input.txt")
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .for_each(|stone| *stones.entry(stone).or_insert(0) += 1);

    for _ in 0..iterations {
        let mut new_stones: HashMap<usize, usize> = HashMap::with_capacity(stones.len() + 50);
        for (&stone_label, &stone_quantity) in stones.iter() {
            let digits = stone_label.to_string();
            if stone_label == 0 {
                *new_stones.entry(1).or_insert(0) += stone_quantity;
            } else if digits.len() % 2 == 0 {
                let (left_str, right_str) = digits.split_at(digits.len() / 2);
                let left = left_str.parse::<usize>().unwrap();
                let right = right_str.parse::<usize>().unwrap();
                *new_stones.entry(left).or_insert(0) += stone_quantity;
                *new_stones.entry(right).or_insert(0) += stone_quantity;
            } else {
                *new_stones.entry(stone_label * 2024).or_insert(0) += stone_quantity;
            }
        }
        stones = new_stones;
    }

    dbg!(stones.values().sum::<usize>());
}

pub fn part1() {
    do_the_thing(25);
}
pub fn part2() {
    do_the_thing(75);
}
