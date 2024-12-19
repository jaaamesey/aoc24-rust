use core::panic;

use hashbrown::HashSet;

pub fn part1() {
    let mut lines = include_str!("./real_input.txt").lines();
    let allowed_substrings = {
        let mut set = HashSet::new();
        lines.next().unwrap().split(", ").for_each(|curr| {
            set.insert(curr);
        });
        set
    };
    lines.next();

    let count = lines
        .enumerate()
        .map(|(i, line)| {
            dbg!(i);
            try_filling_string(&allowed_substrings, &line.chars().collect::<Vec<_>>(), 0)
        })
        .filter(|r| *r)
        .count();

    dbg!(count);

    fn try_filling_string(
        allowed_substrings: &HashSet<&str>,
        line: &Vec<char>,
        mut i: usize,
    ) -> bool {
        let mut substr = line[i].to_string();
        while i < line.len() - 1 {
            if allowed_substrings.contains(substr.as_str()) {
                if try_filling_string(allowed_substrings, line, i + 1) {
                    return true;
                }
            }
            i += 1;
            substr.push(line[i]);
        }
        return allowed_substrings.contains(substr.as_str());
    }
}

pub fn part2() {}
