pub fn execute_part_1() {
    let content = include_str!("input");
    let mut dial = 50;
    let mut zero_count = 0;

    for line in content.lines() {
        let (direction, count) = line.split_at(1);
        let count: i32 = count.parse().unwrap();

        if direction == "L" {
            dial = (dial - count).rem_euclid(100);
        } else {
            dial = (dial + count).rem_euclid(100);
        }

        if dial == 0 {
            zero_count += 1;
        }
    }

    println!("{zero_count}");
}

pub fn execute_part_2() {
    let content = include_str!("input");
    let mut dial = 50;
    let mut zero_count = 0;

    for line in content.lines() {
        let (direction, count) = line.split_at(1);
        let count: i32 = count.parse().unwrap();

        if direction == "L" {
            zero_count += ((dial - 1) as i32).div_euclid(100) - (dial - count - 1).div_euclid(100);
            dial = (dial - count).rem_euclid(100);
        } else {
            zero_count += (dial + count) / 100 - dial / 100;
            dial = (dial + count).rem_euclid(100);
        }
    }

    println!("{zero_count}");
}
