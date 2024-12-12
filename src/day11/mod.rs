use hashbrown::HashMap;

pub fn do_the_thing(iterations: usize) {
    let mut stones: HashMap<usize, usize> = HashMap::new();
    include_str!("./real_input.txt")
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .for_each(|stone| *stones.entry(stone).or_insert(0) += 1);

    let mut memoized = HashMap::<usize, (usize, Option<usize>)>::new();

    for _ in 0..iterations {
        let mut new_stones: HashMap<usize, usize> = HashMap::with_capacity(stones.len() + 50);
        for (&stone_label, &stone_quantity) in stones.iter() {
            if let Some((left, right)) = memoized.get(&stone_label) {
                *new_stones.entry(*left).or_insert(0) += stone_quantity;
                if let Some(right) = right {
                    *new_stones.entry(*right).or_insert(0) += stone_quantity;
                }
                continue;
            }
            let digits = stone_label.to_string();
            if stone_label == 0 {
                *new_stones.entry(1).or_insert(0) += stone_quantity;
                memoized.insert(stone_label, (1, None));
            } else if digits.len() % 2 == 0 {
                let (left_str, right_str) = digits.split_at(digits.len() / 2);
                let left = left_str.parse::<usize>().unwrap();
                let right = right_str.parse::<usize>().unwrap();
                memoized.insert(stone_label, (left, Some(right)));
                *new_stones.entry(left).or_insert(0) += stone_quantity;
                *new_stones.entry(right).or_insert(0) += stone_quantity;
            } else {
                let new_label = stone_label * 2024;
                *new_stones.entry(new_label).or_insert(0) += stone_quantity;
                memoized.insert(stone_label, (new_label, None));
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
