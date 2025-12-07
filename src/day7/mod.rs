use std::collections::HashSet;

pub fn execute_part_1() {
    let content = include_str!("input");
    let lines: Vec<_> = content.lines().collect();
    let mut count = 0;
    let mut active_indexes = HashSet::new();

    for line in lines {
        for (j, ch) in line.chars().enumerate() {
            if ch == 'S' {
                active_indexes.insert(j);
            }

            if ch == '^' && active_indexes.contains(&j) {
                active_indexes.remove(&j);
                active_indexes.extend([j - 1, j + 1]);
                count += 1;
            }
        }
    }

    println!("{}", count);
}

pub fn execute_part_2() {
}
