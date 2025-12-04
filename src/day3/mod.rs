#[derive(Debug)]
struct Pairs {
    high: u8,
    low: u8,
}

pub fn execute_part_1() {
    let content = include_str!("input");
    let mut pairs = Vec::<Pairs>::new();

    for line in content.lines() {
        let mut nums = line.chars().peekable();
        let mut pair = Pairs {
            high: u8::MIN,
            low: u8::MIN,
        };

        while let Some(curr) = nums.next() {
            let curr = curr.to_digit(10).unwrap() as u8;

            if let Some(&next) = nums.peek() {
                let next = next.to_digit(10).unwrap() as u8;

                if curr > pair.high {
                    pair.high = curr;
                    pair.low = u8::MIN;
                }

                if next > pair.low {
                    pair.low = next;
                }
            } else {
                if curr > pair.low {
                    pair.low = curr;
                }
            }
        }

        pairs.push(pair);
    }

    let result = pairs
        .iter()
        .fold(0u32, |acc, p| acc + ((p.high as u32) * 10 + (p.low as u32)));

    println!("{:?}", result);
}

pub fn execute_part_2() {
    let content = include_str!("input");
    let mut result = 0;

    for line in content.lines() {
        let nums: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();

        let mut slots = [0u8; 12];
        let mut input_pos = 0;

        for result_pos in 0..slots.len() {
            let remaining_needed = slots.len() - result_pos;
            let remaining_available = nums.len() - input_pos;
            let can_look_ahead = remaining_available - remaining_needed;
            let mut best_digit = nums[input_pos];
            let mut best_pos = input_pos;

            for i in 0..=can_look_ahead {
                if nums[input_pos + i] > best_digit {
                    best_digit = nums[input_pos + i];
                    best_pos = input_pos + i;
                }
            }

            slots[result_pos] = best_digit;
            input_pos = best_pos + 1;
        }

        result += slots.iter().fold(0usize, |acc, &s| acc * 10 + s as usize);
    }

    println!("{}", result);
}
