#[derive(Debug)]
struct Robot {
    pos: (isize, isize),
    vel: (isize, isize),
}

pub fn part1() {
    let mut lines = include_str!("./real_input.txt").lines();
    let (height_str, width_str) = lines.next().unwrap().split_once(",").unwrap();
    let height = height_str.parse::<isize>().unwrap();
    let width = width_str.parse::<isize>().unwrap();

    let mut robots: Vec<Robot> = lines
        .map(|line| {
            let (mut pos_str, mut vel_str) = line.split_once(" ").unwrap();
            pos_str = &pos_str[2..];
            vel_str = &vel_str[2..];
            let (pos_x_str, pos_y_str) = pos_str.split_once(",").unwrap();
            let (vel_x_str, vel_y_str) = vel_str.split_once(",").unwrap();
            Robot {
                pos: (pos_y_str.parse().unwrap(), pos_x_str.parse().unwrap()),
                vel: (vel_y_str.parse().unwrap(), vel_x_str.parse().unwrap()),
            }
        })
        .collect();

    for _ in 0..100 {
        robots.iter_mut().for_each(|r| {
            r.pos.0 = (r.pos.0 + r.vel.0).rem_euclid(height);
            r.pos.1 = (r.pos.1 + r.vel.1).rem_euclid(width);
        });
    }

    let mut quadrant_a_count = 0;
    let mut quadrant_b_count = 0;
    let mut quadrant_c_count = 0;
    let mut quadrant_d_count = 0;

    for robot in robots {
        let (y, x) = robot.pos;
        if y < height / 2 {
            if x < width / 2 {
                quadrant_a_count += 1;
            } else if x > width / 2 {
                quadrant_b_count += 1;
            }
        } else if y > height / 2 {
            if x < width / 2 {
                quadrant_c_count += 1;
            } else if x > width / 2 {
                quadrant_d_count += 1;
            }
        }
    }

    dbg!(
        quadrant_a_count,
        quadrant_b_count,
        quadrant_c_count,
        quadrant_d_count
    );
    dbg!(quadrant_a_count * quadrant_b_count * quadrant_c_count * quadrant_d_count);
}

pub fn part2() {}
