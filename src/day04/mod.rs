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

pub fn part2() {}
