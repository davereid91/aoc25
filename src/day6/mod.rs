pub fn execute_part_1() {
    let content = include_str!("input");
    let lines: Vec<_> = content.lines().collect();
    let operator_line = lines.last().unwrap();
    let data_lines = &lines[..lines.len() - 1];

    let mut operands_with_offset = Vec::new();
    for (i, ch) in operator_line.char_indices() {
        if !ch.is_whitespace() {
            operands_with_offset.push((ch, i));
        }
    }

    let mut result = 0;

    for i in 0..operands_with_offset.len() {
        let start = operands_with_offset[i].1;
        let end = if i + 1 < operands_with_offset.len() {
            operands_with_offset[i + 1].1
        } else {
            operator_line.len()
        };

        let operator = operands_with_offset[i].0;
        let mut column_sum = 0;
        let mut column_product = 1;

        for line in data_lines {
            let value = line[start..end].trim().parse::<usize>().unwrap_or(0);
            column_sum += value;
            column_product *= value;
        }

        result += match operator {
            '+' => column_sum,
            '*' => column_product,
            _ => 0,
        };
    }

    println!("{result}");
}

pub fn execute_part_2() {}
