pub fn part1() {
    let input = include_str!("./real_input.txt");

    let res = input
        .lines()
        .map(|line| {
            let fragmented = {
                let mut vec = Vec::<String>::new();
                for (i, c) in line.chars().enumerate() {
                    let is_free_space = i % 2 != 0;
                    for _ in 0..c.to_string().parse::<u8>().unwrap() {
                        if is_free_space {
                            vec.push(".".to_string());
                        } else {
                            vec.push((i / 2).to_string());
                        }
                    }
                }
                vec
            };

            let mut defragmented = fragmented.clone();

            for i in 0..defragmented.len() {
                if defragmented[i] == "." {
                    for j in (i..defragmented.len()).rev() {
                        if defragmented[j] != "." {
                            defragmented[i] = defragmented[j].clone();
                            defragmented[j] = ".".to_string();
                            break;
                        }
                    }
                }
            }

            defragmented
                .iter()
                .enumerate()
                .map(|(i, c)| i * c.to_string().parse::<usize>().unwrap_or(0))
                .sum::<usize>()
        })
        .sum::<usize>();
    dbg!(res);
}

pub fn part2() {}
