pub fn part1() {
    // 1111 = 15
    // 4^2 - 1?
    // 111 = 7
    // 3^2 - 1? No
    #[derive(Debug)]
    enum Operator {
        ADD,
        MULTIPLY,
    }

    let lines = include_str!("./real_input.txt").lines().map(|l| {
        let (test_str, numbers_str) = l.split_once(": ").unwrap();
        let test = test_str.parse::<u64>().unwrap();
        let numbers = numbers_str
            .split(' ')
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        (test, numbers)
    });
    let res = lines
        .filter_map(|(test, numbers)| {
            let (max_permutation, max_permutation_str) = {
                let mut str = String::new();
                for _ in 0..numbers.len() {
                    str.push('1');
                }
                (u64::from_str_radix(&str, 2).unwrap(), str)
            };
            let mut current_permutation: u64 = 0;
            loop {
                let permutation_str = &format!(
                    "{:0len$b}",
                    current_permutation,
                    len = max_permutation_str.len()
                );
                let mut operators = {
                    permutation_str.chars().enumerate().map(|(i, c)| {
                        if c == '0' || i == 0 {
                            Operator::ADD
                        } else {
                            Operator::MULTIPLY
                        }
                    })
                };

                let mut result = 0;
                // TODO: subtract by 1
                for n in numbers.iter() {
                    let operator = operators.next().unwrap();
                    match operator {
                        Operator::ADD => result += n,
                        Operator::MULTIPLY => result *= n,
                    }
                }

                if result == test {
                    return Some(test);
                }

                current_permutation += 1;

                if current_permutation > max_permutation {
                    return None;
                }
            }
        })
        .sum::<u64>();
    dbg!(res);
}

pub fn part2() {}
