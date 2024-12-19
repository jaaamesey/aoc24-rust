use hashbrown::HashSet;

pub fn part1() {
    let mut lines = include_str!("./test_input.txt").lines();
    let allowed_substrings = {
        let mut set = HashSet::new();
        lines.next().unwrap().split(", ").for_each(|curr| {
            set.insert(curr);
        });
        set
    };
    lines.next();

    lines.for_each(|line| {
        let mut char_iter = line.chars();
        let mut get_next_char = || char_iter.next();
        dbg!(try_filling_string(
            &allowed_substrings,
            &mut get_next_char,
            ""
        ));
    });

    fn try_filling_string(
        allowed_substrings: &HashSet<&str>,
        get_next_char: &mut dyn FnMut() -> Option<char>,
        current_substring: &str,
    ) -> bool {
        let next_char = get_next_char();
        let can_fill_substr = allowed_substrings.contains(current_substring);
        if let Some(next_char) = next_char {
            let mut new_substring = current_substring.to_string();
            new_substring.push(next_char);
            if can_fill_substr {
                let works_with_this_fill =
                    try_filling_string(allowed_substrings, get_next_char, "");
                if works_with_this_fill {
                    return true;
                }
            }
            let works_without_this_fill =
                try_filling_string(allowed_substrings, get_next_char, &new_substring);
            return works_without_this_fill;
        } else {
            return can_fill_substr;
        }
    }
}

pub fn part2() {}
