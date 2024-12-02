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
    for report in reports.iter() {
        let mut report_direction: i8 = 0;
        let mut is_main_report_safe = true;
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
                let any_variants_safe = report
                    .iter()
                    .enumerate()
                    .map(|(i, _n)| {
                        report
                            .iter()
                            .enumerate()
                            .filter_map(|(j, &n)| if j != i { Some(n) } else { None })
                            .collect::<Vec<_>>()
                    })
                    .any(|variant| {
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
                    });
                if any_variants_safe {
                    num_safe_reports += 1;
                }
                is_main_report_safe = false;
                break;
            }
        }

        if is_main_report_safe {
            num_safe_reports += 1;
            continue;
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
