use hashbrown::{HashMap, HashSet};

pub fn part1() {
    let lines = include_str!("./real_input.txt").lines();

    let mut positions_by_frequency: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for (y, line) in lines.clone().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                positions_by_frequency
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push((isize::try_from(y).unwrap(), isize::try_from(x).unwrap()))
            }
        }
    }

    let grid_height = isize::try_from(lines.clone().count()).unwrap();
    let grid_width = isize::try_from(lines.clone().next().unwrap().chars().count()).unwrap();

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for frequency_positions in positions_by_frequency.values() {
        for position in frequency_positions {
            for other_position in frequency_positions {
                if position == other_position {
                    continue;
                }
                let offset_y = other_position.0 - position.0;
                let offset_x = other_position.1 - position.1;
                {
                    let antinode_y = position.0 - offset_y;
                    let antinode_x = position.1 - offset_x;
                    if antinode_y >= 0
                        && antinode_y < grid_height
                        && antinode_x >= 0
                        && antinode_x < grid_width
                    {
                        antinodes.insert((antinode_y, antinode_x));
                    }
                }
            }
        }
    }

    dbg!(antinodes.len());

    for (y, line) in lines.enumerate() {
        println!(
            "{}",
            line.chars()
                .enumerate()
                .map(|(x, _)| if antinodes.contains(&(y as isize, x as isize)) {
                    '#'
                } else {
                    '.'
                })
                .collect::<String>()
        );
    }
}

pub fn part2() {
    let lines = include_str!("./real_input.txt").lines();

    let mut positions_by_frequency: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for (y, line) in lines.clone().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                positions_by_frequency
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push((isize::try_from(y).unwrap(), isize::try_from(x).unwrap()))
            }
        }
    }

    let grid_height = isize::try_from(lines.clone().count()).unwrap();
    let grid_width = isize::try_from(lines.clone().next().unwrap().chars().count()).unwrap();

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for frequency_positions in positions_by_frequency.values() {
        for position in frequency_positions {
            for other_position in frequency_positions {
                if position == other_position {
                    continue;
                }
                let offset_y = other_position.0 - position.0;
                let offset_x = other_position.1 - position.1;
                let mut i = 0;
                loop {
                    let antinode_y = position.0 - i * offset_y;
                    let antinode_x = position.1 - i * offset_x;
                    if antinode_y >= 0
                        && antinode_y < grid_height
                        && antinode_x >= 0
                        && antinode_x < grid_width
                    {
                        antinodes.insert((antinode_y, antinode_x));
                    } else {
                        break;
                    }
                    i += 1;
                }
            }
        }
    }

    dbg!(antinodes.len());
}
