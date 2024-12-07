use radix_fmt::radix;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;

pub fn part1() {
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
                if result > test {
                    return None;
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

pub fn part2() {
    #[derive(Debug)]
    enum Operator {
        ADD,
        MULTIPLY,
        CONCATENATE,
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
        .par_bridge()
        .filter_map(|(test, numbers)| {
            let (max_permutation, max_permutation_str) = {
                let mut str = String::new();
                for _ in 0..numbers.len() {
                    str.push('2');
                }
                (u64::from_str_radix(&str, 3).unwrap(), str)
            };
            let mut current_permutation: u64 = 0;
            loop {
                let permutation = {
                    let mut vec = vec![0; max_permutation_str.len()];
                    let mut i = vec.len() - 1;
                    let mut n = current_permutation;
                    loop {
                        if i == 0 {
                            break;
                        }
                        vec[i] = n % 3;
                        n /= 3;
                        i -= 1;
                    }
                    vec
                };
                let mut operators = {
                    permutation.iter().enumerate().map(|(i, c)| {
                        if *c == 0 || i == 0 {
                            Operator::ADD
                        } else if *c == 1 {
                            Operator::MULTIPLY
                        } else {
                            Operator::CONCATENATE
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
                        Operator::CONCATENATE => {
                            result = (result.to_string() + &n.to_string())
                                .parse::<u64>()
                                .unwrap();
                        }
                    };
                    if result > test {
                        break;
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
