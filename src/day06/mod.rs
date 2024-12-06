use hashbrown::HashSet;

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
}
