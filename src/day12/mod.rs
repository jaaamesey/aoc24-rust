use hashbrown::HashSet;

pub fn part1() {
    let grid = include_str!("./real_input.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let directions: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut regions = Vec::<HashSet<(usize, usize)>>::new();

    let mut visited = HashSet::<(usize, usize)>::new();

    fn search_region(
        directions: &[(isize, isize)],
        visited: &mut HashSet<(usize, usize)>,
        in_region: &mut HashSet<(usize, usize)>,
        grid: &Vec<Vec<char>>,
        target_plot_type: char,
        pos: &(usize, usize),
    ) {
        let (y, x) = *pos;
        if y >= grid.len() || x >= grid[0].len() {
            return;
        }
        if in_region.contains(pos) {
            return;
        }
        let plot_type = grid[y][x];
        if plot_type != target_plot_type {
            return;
        }
        in_region.insert((y, x));
        visited.insert((y, x));
        for (dy, dx) in directions {
            search_region(
                directions,
                visited,
                in_region,
                grid,
                target_plot_type,
                &(
                    y.checked_add_signed(*dy).unwrap_or(grid.len()),
                    x.checked_add_signed(*dx).unwrap_or(grid[0].len()),
                ),
            );
        }
    }

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if visited.contains(&(y, x)) {
                continue;
            }

            let mut in_region = HashSet::<(usize, usize)>::new();
            let target_plot_type = grid[y][x];
            let pos = (y, x);
            search_region(
                &directions,
                &mut visited,
                &mut in_region,
                &grid,
                target_plot_type,
                &pos,
            );
            regions.push(in_region);

            visited.insert((y, x));
        }
    }

    let sum = regions
        .iter()
        .map(|r| {
            let area = r.len();
            let mut perimeter = 0;
            for (oy, ox) in r {
                for (dy, dx) in directions {
                    let y = oy.checked_add_signed(dy).unwrap_or(grid.len());
                    let x = ox.checked_add_signed(dx).unwrap_or(grid[0].len());
                    if y >= grid.len() || x >= grid[0].len() {
                        perimeter += 1;
                    } else if grid[y][x] != grid[*oy][*ox] {
                        perimeter += 1;
                    }
                }
            }
            return area * perimeter;
        })
        .sum::<usize>();

    dbg!(sum);
}

pub fn part2() {
    let grid = include_str!("./real_input.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let directions: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let mut regions = Vec::<HashSet<(usize, usize)>>::new();

    let mut visited = HashSet::<(usize, usize)>::new();

    fn search_region(
        directions: &[(isize, isize)],
        visited: &mut HashSet<(usize, usize)>,
        in_region: &mut HashSet<(usize, usize)>,
        grid: &Vec<Vec<char>>,
        target_plot_type: char,
        pos: &(usize, usize),
    ) {
        let (y, x) = *pos;
        if y >= grid.len() || x >= grid[0].len() {
            return;
        }
        if in_region.contains(pos) {
            return;
        }
        let plot_type = grid[y][x];
        if plot_type != target_plot_type {
            return;
        }
        in_region.insert((y, x));
        visited.insert((y, x));
        for (dy, dx) in directions {
            search_region(
                directions,
                visited,
                in_region,
                grid,
                target_plot_type,
                &(
                    y.checked_add_signed(*dy).unwrap_or(grid.len()),
                    x.checked_add_signed(*dx).unwrap_or(grid[0].len()),
                ),
            );
        }
    }

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if visited.contains(&(y, x)) {
                continue;
            }

            let mut in_region = HashSet::<(usize, usize)>::new();
            let target_plot_type = grid[y][x];
            let pos = (y, x);
            search_region(
                &directions,
                &mut visited,
                &mut in_region,
                &grid,
                target_plot_type,
                &pos,
            );
            regions.push(in_region);

            visited.insert((y, x));
        }
    }

    let corner_checks: [[(isize, isize); 2]; 4] = [
        [(1, 0), (0, 1)],
        [(0, -1), (1, 0)],
        [(-1, 0), (0, -1)],
        [(0, 1), (-1, 0)],
    ];

    let sum = regions
        .iter()
        .map(|r| {
            let (initial_y, initial_x) = r.iter().next().unwrap();
            let target_type = grid[*initial_y][*initial_x];
            let area = r.len();
            let mut perimeter = HashSet::<(usize, usize)>::new();
            for (oy, ox) in r {
                for (dy, dx) in directions {
                    let y = oy.checked_add_signed(dy).unwrap_or(grid.len());
                    let x = ox.checked_add_signed(dx).unwrap_or(grid[0].len());
                    if y >= grid.len() || x >= grid[0].len() {
                        //perimeter += 1;
                    } else if grid[y][x] != grid[*oy][*ox] {
                        perimeter.insert((y, x));
                    }
                }
            }
            let num_exterior_corners = r
                .iter()
                .map(|(oy, ox)| {
                    corner_checks
                        .iter()
                        .filter(|corner_check| {
                            let corner_passed = corner_check.iter().all(|(dy, dx)| {
                                let y = oy.checked_add_signed(*dy).unwrap_or(grid.len());
                                let x = ox.checked_add_signed(*dx).unwrap_or(grid[0].len());
                                if y >= grid.len() || x >= grid[0].len() {
                                    return true;
                                } else if !r.contains(&(y, x)) {
                                    return true;
                                }
                                return false;
                            });
                            return corner_passed;
                        })
                        .count()
                })
                .sum::<usize>();

            let mut coords = Vec::new();
            for y in 0..grid.len() + 2 {
                for x in 0..grid[0].len() + 2 {
                    let y = (y as isize) - 1;
                    let x = (x as isize) - 1;
                    if !r.contains(&(y as usize, x as usize)) {
                        coords.push((y, x))
                    }
                }
            }

            let num_interior_corners = coords
                .iter()
                .map(|(oy, ox)| {
                    corner_checks
                        .iter()
                        .filter(|corner_check| {
                            corner_check.iter().all(|(dy, dx)| {
                                let y = oy + (*dy);
                                let x = ox + (*dx);
                                if !r.contains(&(y as usize, x as usize)) {
                                    return false;
                                }
                                return true;
                            })
                        })
                        .count()
                })
                .sum::<usize>();
            return area * (num_exterior_corners + num_interior_corners);
        })
        .sum::<usize>();

    dbg!(sum);
}
