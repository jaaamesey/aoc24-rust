use hashbrown::HashSet;

pub fn part1() {
    let grid: Vec<Vec<u8>> = include_str!("./real_input.txt")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect();

    let directions: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let val = grid[y][x];
            if val != 0 {
                continue;
            }
            let mut nines_found = HashSet::<(usize, usize)>::new();
            find_nines_p1(&grid, &directions, (y, x), &mut nines_found);
            sum += nines_found.len();
        }
    }
    dbg!(sum);
}

fn find_nines_p1(
    grid: &Vec<Vec<u8>>,
    directions: &[(isize, isize)],
    (y, x): (usize, usize),
    nines_found: &mut HashSet<(usize, usize)>,
) {
    let height = grid.len();
    let width = grid[0].len();
    let neighbours = directions.iter().filter_map(|dir| {
        let ny = y.checked_add_signed(dir.0).unwrap_or(height);
        let nx = x.checked_add_signed(dir.1).unwrap_or(width);
        if ny < height && nx < width && grid[ny][nx] == grid[y][x] + 1 {
            return Some((ny, nx));
        }
        return None;
    });
    for pos in neighbours {
        if grid[pos.0][pos.1] == 9 {
            nines_found.insert(pos);
            continue;
        }
        find_nines_p1(grid, directions, pos, nines_found);
    }
}

pub fn part2() {
    let grid: Vec<Vec<u8>> = include_str!("./real_input.txt")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect();

    let directions: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let val = grid[y][x];
            if val != 0 {
                continue;
            }
            let mut paths_found = 0;
            find_nines_p2(&grid, &directions, &(y, x), &mut paths_found);
            sum += paths_found;
        }
    }
    dbg!(sum);
}

fn find_nines_p2(
    grid: &Vec<Vec<u8>>,
    directions: &[(isize, isize)],
    (y, x): &(usize, usize),
    paths_found: &mut usize,
) {
    let height = grid.len();
    let width = grid[0].len();
    let neighbours = directions.iter().filter_map(|(dy, dx)| {
        let ny = y.checked_add_signed(*dy).unwrap_or(height);
        let nx = x.checked_add_signed(*dx).unwrap_or(width);
        if ny < height && nx < width && grid[ny][nx] == grid[*y][*x] + 1 {
            return Some((ny, nx));
        }
        return None;
    });
    for pos in neighbours {
        if grid[pos.0][pos.1] == 9 {
            *paths_found += 1;
            continue;
        }
        find_nines_p2(grid, directions, &pos, paths_found);
    }
}
