pub fn part1() {
    let board = include_str!("./real_input.txt")
        .lines()
        .map(|str| str.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let directions: [(isize, isize); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    let mut num_instances = 0;
    for (y, row) in board.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c != 'X' {
                continue;
            }
            num_instances += directions
                .iter()
                .filter(|(dy, dx)| {
                    "XMAS".chars().enumerate().all(|(i, target_char)| {
                        let by = y.checked_add_signed(*dy * (i as isize));
                        let bx = x.checked_add_signed(*dx * (i as isize));
                        if by.is_none()
                            || bx.is_none()
                            || by.unwrap() > board.len() - 1
                            || bx.unwrap() > row.len() - 1
                        {
                            return false;
                        }
                        return board[by.unwrap()][bx.unwrap()] == target_char;
                    })
                })
                .count();
        }
    }
    dbg!(num_instances);
}

pub fn part2() {
    let board = include_str!("./real_input.txt")
        .lines()
        .map(|str| str.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let directions: [(isize, isize); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    let mut diagonal_centers = board
        .iter()
        .map(|row| row.iter().map(|_| 0).collect())
        .collect::<Vec<Vec<usize>>>();
    let target_chars = "MAS".chars().enumerate().collect::<Vec<_>>();
    for (y, row) in board.iter().enumerate() {
        for x in row
            .iter()
            .enumerate()
            .filter_map(|(i, &c)| if c == 'M' { Some(i) } else { None })
        {
            for (dy, dx) in directions {
                let mut diagonal_center: Option<(usize, usize)> = None;
                for &(i, target_char) in target_chars.iter() {
                    let by = y.checked_add_signed(dy * (i as isize));
                    let bx = x.checked_add_signed(dx * (i as isize));
                    if by.is_none()
                        || bx.is_none()
                        || by.unwrap() > board.len() - 1
                        || bx.unwrap() > row.len() - 1
                        || board[by.unwrap()][bx.unwrap()] != target_char
                    {
                        diagonal_center = None;
                        break;
                    }
                    if i == 1 {
                        diagonal_center = Some((by.unwrap(), bx.unwrap()))
                    };
                }
                if let Some((y, x)) = diagonal_center {
                    diagonal_centers[y][x] += 1;
                }
            }
        }
    }
    let num_crosses = diagonal_centers
        .iter()
        .map(|row| row.iter().filter(|&&count| count > 1).count())
        .sum::<usize>();
    dbg!(num_crosses);
}
