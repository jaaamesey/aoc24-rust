pub fn part1() {
    let input = include_str!("./real_input.txt");

    let rules = input
        .lines()
        .filter(|l| l.find('|').is_some())
        .map(|l| {
            l.trim()
                .split_once('|')
                .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
                .unwrap()
        })
        .collect::<Vec<_>>();

    let updates = input
        .lines()
        .filter(|l| !l.find('|').is_some() && l.trim().len() > 0)
        .map(|l| {
            l.trim()
                .split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for update in updates {
        let is_valid = update.iter().enumerate().all(|(index_in_update, page)| {
            let mut numbers_that_must_come_after = rules.iter().filter_map(|(before, after)| {
                if &before == &page && update.iter().any(|n| n == after) {
                    Some(after)
                } else {
                    None
                }
            });
            let slice = &update[index_in_update..update.len()];
            return numbers_that_must_come_after.all(|number_that_must_come_after| {
                slice.iter().any(|n| n == number_that_must_come_after)
            });
        });
        if is_valid {
            sum += update[update.len() / 2];
        }
    }
    dbg!(sum);
}

pub fn part2() {}
