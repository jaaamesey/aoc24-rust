use hashbrown::HashMap;

pub fn part1() {
    let input = include_str!("./real_input.txt");

    let res = input
        .lines()
        .map(|line| {
            let fragmented = {
                let mut vec = Vec::<Option<usize>>::new();
                for (i, c) in line.chars().enumerate() {
                    let is_free_space = i % 2 != 0;
                    for _ in 0..c.to_string().parse::<u8>().unwrap() {
                        if is_free_space {
                            vec.push(None);
                        } else {
                            vec.push(Some(i / 2));
                        }
                    }
                }
                vec
            };

            let mut defragmented = fragmented.clone();

            for i in 0..defragmented.len() {
                if defragmented[i].is_none() {
                    for j in (i..defragmented.len()).rev() {
                        if defragmented[j].is_some() {
                            defragmented[i] = defragmented[j];
                            defragmented[j] = None;
                            break;
                        }
                    }
                }
            }

            defragmented
                .iter()
                .enumerate()
                .map(|(i, c)| i * c.unwrap_or(0))
                .sum::<usize>()
        })
        .sum::<usize>();
    dbg!(res);
}

pub fn part2() {
    let input = include_str!("./real_input.txt");

    let res = input
        .lines()
        .map(|line| {
            let (fragmented, sizes, starting_positions) = {
                let mut vec = Vec::<Option<usize>>::new();
                let mut sizes = HashMap::<usize, u8>::new();
                let mut starting_positions = HashMap::<usize, usize>::new();
                for (i, c) in line.chars().enumerate() {
                    let is_free_space = i % 2 != 0;
                    let size = c.to_string().parse::<u8>().unwrap();
                    for _ in 0..size {
                        if is_free_space {
                            vec.push(None);
                        } else {
                            vec.push(Some(i / 2));
                            sizes.insert(i / 2, size);
                        }
                    }
                }
                for i in 0..vec.len() {
                    if let Some(file_id) = vec[i] {
                        if !starting_positions.contains_key(&file_id) {
                            starting_positions.insert(file_id, i);
                        }
                    }
                }
                (vec, sizes, starting_positions)
            };

            let mut defragmented = fragmented.clone();

            let mut files_to_insert = sizes.keys().collect::<Vec<_>>();
            files_to_insert.sort_by(|a, b| b.cmp(a));

            for file_id in files_to_insert.iter() {
                let file_size = usize::from(*sizes.get(*file_id).unwrap());
                let mut curr_blank_start: Option<usize> = None;
                for i in 0..defragmented.len() {
                    if defragmented[i] == Some(**file_id) {
                        break;
                    }
                    if curr_blank_start.is_none() && defragmented[i].is_none() {
                        curr_blank_start = Some(i);
                    }
                    if defragmented[i].is_some() {
                        curr_blank_start = None;
                    }
                    if curr_blank_start.is_some() {
                        let available_space = i - curr_blank_start.unwrap() + 1;
                        if file_size <= available_space {
                            // Remove file
                            let remove_start_pos = *starting_positions.get(*file_id).unwrap();
                            let remove_end_pos = remove_start_pos + file_size;
                            for i in remove_start_pos..remove_end_pos {
                                if defragmented[i] == Some(**file_id) {
                                    defragmented[i] = None;
                                }
                            }
                            // Add file
                            for j in curr_blank_start.unwrap()..i + 1 {
                                defragmented[j] = Some(**file_id);
                            }
                            break;
                        }
                    }
                }
            }

            defragmented
                .iter()
                .enumerate()
                .map(|(i, c)| i * c.unwrap_or(0))
                .sum::<usize>()
        })
        .sum::<usize>();
    dbg!(res);
}
