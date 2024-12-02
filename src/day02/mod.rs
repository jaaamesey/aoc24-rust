extern crate hashbrown;

pub fn part1() {
    let input_str = include_str!("./real_input.txt");

    let report_strs = input_str.lines();

    let reports = report_strs
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut num_safe_reports = 0;
    for report in reports {
        let mut report_direction: i8 = 0;
        let mut is_safe = true;
        for (i, item) in report.iter().enumerate() {
            if i == 0 {
                continue;
            }
            let prev = report[i - 1];
            let difference_from_prev = item - prev;
            let direction = sign(difference_from_prev);
            if report_direction == 0 {
                report_direction = direction;
            }
            if direction != report_direction
                || difference_from_prev.abs() < 1
                || difference_from_prev.abs() > 3
            {
                is_safe = false;
                break;
            }
        }
        if is_safe {
            num_safe_reports += 1;
        }
    }

    dbg!(num_safe_reports);
}

pub fn part2() {
    let input_str = include_str!("./real_input.txt");

    let report_strs = input_str.lines();

    let reports = report_strs
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut num_safe_reports = 0;
    for report in reports {
        let mut report_direction: i8 = 0;
        let mut variants_to_retry: Vec<Vec<i32>> = Vec::new();
        for (i, item) in report.iter().enumerate() {
            if i == 0 {
                continue;
            }
            let prev = report[i - 1];
            let difference_from_prev = item - prev;
            let direction = sign(difference_from_prev);
            if report_direction == 0 {
                report_direction = direction;
            }
            if direction != report_direction
                || difference_from_prev.abs() < 1
                || difference_from_prev.abs() > 3
            {
                for (i, _item) in report.iter().enumerate() {
                    variants_to_retry.push(
                        report
                            .iter()
                            .enumerate()
                            .filter_map(|(j, &n)| if j != i { Some(n) } else { None })
                            .collect(),
                    );
                }
                break;
            }
        }

        if variants_to_retry.is_empty() {
            num_safe_reports += 1;
            continue;
        }

        let any_variants_safe = variants_to_retry
            .iter()
            .filter(|variant| {
                let mut report_direction: i8 = 0;
                for (i, item) in variant.iter().enumerate() {
                    if i == 0 {
                        continue;
                    }
                    let prev = variant[i - 1];
                    let difference_from_prev = item - prev;
                    let direction = sign(difference_from_prev);
                    if report_direction == 0 {
                        report_direction = direction;
                    }
                    if direction != report_direction
                        || difference_from_prev.abs() < 1
                        || difference_from_prev.abs() > 3
                    {
                        return false;
                    }
                }
                return true;
            })
            .count();

        if any_variants_safe > 0 {
            num_safe_reports += 1;
        }
    }

    dbg!(num_safe_reports);
}

fn sign(n: i32) -> i8 {
    if n == 0 {
        return 0;
    }
    if n > 0 {
        return 1;
    }
    return -1;
}
