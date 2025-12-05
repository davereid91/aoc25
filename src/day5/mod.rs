pub fn execute_part_1() {
    let content = include_str!("input");
    let (ranges_str, id_str) = content.split_once("\n\n").unwrap();
    let mut ranges = vec![];
    let mut fresh_count = 0;

    for range in ranges_str.lines() {
        let (low, high) = range.split_once('-').unwrap();
        let low: usize = low.parse().unwrap();
        let high: usize = high.parse().unwrap();
        ranges.push(low..=high);
    }

    for id in id_str.lines() {
        let id: usize = id.parse().unwrap();

        for range in ranges.iter() {
            if range.contains(&id) {
                fresh_count += 1;
                break;
            }
        }
    }

    println!("{fresh_count}");
}

pub fn execute_part_2() {
    let content = include_str!("input");
    let (ranges_str, _) = content.split_once("\n\n").unwrap();
    let mut ranges = vec![];

    for range in ranges_str.lines() {
        let (low, high) = range.split_once('-').unwrap();
        let low: usize = low.parse().unwrap();
        let high: usize = high.parse().unwrap();

        ranges.push(low..=high);
    }

    ranges.sort_by_key(|r| *r.start());

    let mut merged = vec![ranges[0].clone()];

    for range in ranges.into_iter().skip(1) {
        let last = merged.pop().unwrap();

        if *range.start() <= *last.end() {
            merged.push(*last.start()..=(*last.end().max(range.end())));
        } else {
            merged.push(last);
            merged.push(range);
        }
    }

    let result: usize = merged.iter().map(|r| r.end() - r.start() + 1).sum();

    println!("{}", result);
}
