use hashbrown::HashMap;

pub fn do_the_thing(iterations: usize) {
    let mut stones: HashMap<usize, usize> = HashMap::new();
    include_str!("./real_input.txt")
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .for_each(|stone| *stones.entry(stone).or_insert(0) += 1);

    for _ in 0..iterations {
        let prev_stones = stones.clone();
        for (stone_label, stone_quantity) in prev_stones {
            let digits = stone_label.to_string();
            *stones.entry(stone_label).or_insert(0) -= stone_quantity;
            if stone_label == 0 {
                *stones.entry(1).or_insert(0) += stone_quantity;
            } else if digits.len() % 2 == 0 {
                let (left_str, right_str) = digits.split_at(digits.len() / 2);
                if left_str.len() != right_str.len() {
                    panic!();
                }
                let left = left_str.parse::<usize>().unwrap();
                let right = right_str.parse::<usize>().unwrap();

                *stones.entry(left).or_insert(0) += stone_quantity;
                *stones.entry(right).or_insert(0) += stone_quantity;
            } else {
                *stones.entry(stone_label * 2024).or_insert(0) += stone_quantity;
            }
        }
    }

    dbg!(stones.values().sum::<usize>());
}

pub fn part1() {
    do_the_thing(25);
}
pub fn part2() {
    do_the_thing(75);
}
