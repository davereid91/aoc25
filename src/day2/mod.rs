pub fn execute_part_1() {
    let content = include_str!("input");
    let potential_ranges = content.split_inclusive(',');
    let mut actual_ranges = Vec::new();
    let mut count = 0;

    for range in potential_ranges {
        let range = &range[0..range.len() - 1];
        let (l, r) = range.split_once('-').unwrap();
        let num_l: usize = l.parse().unwrap();
        let num_r: usize = r.parse().unwrap();
        actual_ranges.push(num_l..=num_r);
    }

    for range in actual_ranges {
        for x in range {
            let x_str = x.to_string();
            let (x_l, x_r) = x_str.split_at(x_str.len() / 2);

            if x_l == x_r {
                count += x;
            }
        }
    }

    println!("{count}");
}

pub fn execute_part_2() {
    let content = include_str!("input");
    let potential_ranges = content.split_inclusive(',');
    let mut actual_ranges = Vec::new();
    let mut count = 0;

    for range in potential_ranges {
        let range = &range[0..range.len() - 1];
        let (l, r) = range.split_once('-').unwrap();
        let num_l: usize = l.parse().unwrap();
        let num_r: usize = r.parse().unwrap();
        actual_ranges.push(num_l..=num_r);
    }

    for range in actual_ranges {
        for x in range {
            if is_invalid_id(x) {
                count += x;
            }
        }
    }

    println!("{count}");
}

fn is_invalid_id(id: usize) -> bool {
    let x_str = id.to_string();
    let len = x_str.len();

    for pattern_len in 1..=len / 2 {
        if len % pattern_len == 0 {
            let pattern = &x_str[0..pattern_len];
            let repetitions = len / pattern_len;

            if pattern.repeat(repetitions) == x_str {
                return true;
            }
        }
    }

    false
}
