extern crate rayon;
use hashbrown::HashSet;
use rayon::iter::{ParallelBridge, ParallelIterator};

pub fn part1() {
    let (board, starting_pos) = {
        let mut starting_pos: Option<(usize, usize)> = None;
        let vec: Vec<Vec<char>> = include_str!("./real_input.txt")
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == '^' {
                            starting_pos = Some((y, x));
                            return '.';
                        } else {
                            return c;
                        }
                    })
                    .collect()
            })
            .collect();

        (vec, starting_pos.unwrap())
    };

    let board_width = board[0].len();
    let board_height = board.len();

    let directions: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir_index = 0;
    let mut pos = starting_pos;

    let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();

    loop {
        let (y, x) = pos;
        let (dy, dx) = directions[dir_index];

        visited_positions.insert((y, x));

        if let Some(row) = board.get(y.checked_add_signed(dy).unwrap_or(board_height)) {
            if let Some(&c) = row.get(x.checked_add_signed(dx).unwrap_or(board_width)) {
                if c == '#' {
                    dir_index = (dir_index + 1) % directions.len();
                    continue;
                }
            }
        }

        let new_y = y.checked_add_signed(dy).unwrap_or(board_height);
        let new_x = x.checked_add_signed(dx).unwrap_or(board_width);
        if new_y > board_height - 1 || new_x > board_width - 1 {
            break;
        }

        pos = (new_y, new_x);
    }

    dbg!(visited_positions.len());
}

pub fn part2() {
    let (board, starting_pos) = {
        let mut starting_pos: Option<(usize, usize)> = None;
        let vec: Vec<Vec<char>> = include_str!("./real_input.txt")
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == '^' {
                            starting_pos = Some((y, x));
                            return '.';
                        } else {
                            return c;
                        }
                    })
                    .collect()
            })
            .collect();

        (vec, starting_pos.unwrap())
    };

    let board_width = board[0].len();
    let board_height = board.len();

    let directions: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir_index = 0;
    let mut pos = starting_pos;

    let mut possible_new_obstacle_locations: HashSet<(usize, usize)> = HashSet::new();

    loop {
        let (y, x) = pos;
        let (dy, dx) = directions[dir_index];

        possible_new_obstacle_locations.insert((y, x));

        let new_y = y.checked_add_signed(dy).unwrap_or(board_height);
        let new_x = x.checked_add_signed(dx).unwrap_or(board_width);

        if let Some(row) = board.get(new_y) {
            if let Some(&c) = row.get(new_x) {
                if c == '#' {
                    dir_index = (dir_index + 1) % directions.len();
                    continue;
                }
            }
        }

        if new_y > board_height - 1 || new_x > board_width - 1 {
            break;
        }

        pos = (new_y, new_x);
    }

    let count = possible_new_obstacle_locations
        .iter()
        .par_bridge()
        .filter(|(new_obstacle_y, new_obstacle_x)| {
            let mut dir_index = 0;
            let mut pos = starting_pos;
            let mut visited_positions_and_dir_indexes: HashSet<((usize, usize), usize)> =
                HashSet::new();
            loop {
                let (y, x) = pos;
                let (dy, dx) = directions[dir_index];

                visited_positions_and_dir_indexes.insert(((y, x), dir_index));

                let new_y = y.checked_add_signed(dy).unwrap_or(board_height);
                let new_x = x.checked_add_signed(dx).unwrap_or(board_width);

                if let Some(row) = board.get(new_y) {
                    if let Some(&c) = row.get(new_x) {
                        if c == '#' || (new_y == *new_obstacle_y && new_x == *new_obstacle_x) {
                            dir_index = (dir_index + 1) % directions.len();
                            continue;
                        }
                    }
                }

                if visited_positions_and_dir_indexes.contains(&((new_y, new_x), dir_index)) {
                    return true;
                }

                if new_y > board_height - 1 || new_x > board_width - 1 {
                    return false;
                }

                pos = (new_y, new_x);
            }
        })
        .count();

    dbg!(count);
}
