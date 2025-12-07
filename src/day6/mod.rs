pub fn execute_part_1() {
    let content = include_str!("input");
    let lines: Vec<_> = content.lines().collect();
    let mut reversed = lines.iter().rev();
    let operator_line = reversed.next().unwrap();

    let mut positions = Vec::new();
    for (i, ch) in operator_line.chars().enumerate() {
        if !ch.is_whitespace() {
            positions.push((i, ch));
        }
    }

    let mut rows = vec![];
    for line in lines.iter().take(lines.len() - 1) {
        let mut row = Vec::new();

        for i in 0..positions.len() {
            let start = positions[i].0;
            let end = if i + 1 < positions.len() {
                positions[i + 1].0
            } else {
                line.len()
            };

            let value = line[start..end].trim().parse().unwrap_or(0);
            row.push(value);
        }

        rows.push(row);
    }

    let mut result: usize = 0;

    for (i, (_, operator)) in positions.iter().enumerate() {
        let columns: Vec<usize> = rows.iter().map(|row| row[i] as usize).collect();

        match operator {
            '+' => result += columns.iter().sum::<usize>(),
            '*' => result += columns.iter().product::<usize>(),
            _ => {}
        }
    }

    println!("{result}");
}

pub fn execute_part_2() {}
