use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

const NEIGHBOURS: [(i32, i32); 8] = [
    (-1, 0),  // N
    (-1, 1),  // NE
    (0, 1),   // E
    (1, 1),   // SE
    (1, 0),   // S
    (1, -1),  // SW
    (0, -1),  // W
    (-1, -1), // NW
];

pub fn execute_part_1() {
    let content = include_str!("input");
    let lines = content.lines().enumerate();
    let mut points = HashSet::new();
    let mut roll_count = 0;

    for (x, line) in lines {
        for (y, char) in line.chars().enumerate() {
            if char == '@' {
                points.insert(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    for point in points.iter() {
        let mut neighbour_count = 0;

        for neighbour in NEIGHBOURS.iter() {
            if points.contains(&Point {
                x: point.x + neighbour.0,
                y: point.y + neighbour.1,
            }) {
                neighbour_count += 1;
            }
        }

        if neighbour_count < 4 {
            roll_count += 1;
        }
    }

    println!("{roll_count}");
}

pub fn execute_part_2() {
    let content = include_str!("input");
    let lines = content.lines().enumerate();
    let mut points = HashSet::new();
    let mut roll_count = 0;

    for (x, line) in lines {
        for (y, char) in line.chars().enumerate() {
            if char == '@' {
                points.insert(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    loop {
        let mut points_to_remove = HashSet::new();

        for point in points.iter() {
            let mut neighbour_count = 0;

            for neighbour in NEIGHBOURS.iter() {
                if points.contains(&Point {
                    x: point.x + neighbour.0,
                    y: point.y + neighbour.1,
                }) {
                    neighbour_count += 1;
                }
            }

            if neighbour_count < 4 {
                roll_count += 1;
                points_to_remove.insert(Point {
                    x: point.x,
                    y: point.y,
                });
            }
        }

        if points_to_remove.is_empty() {
            break;
        }

        points.retain(|p| !points_to_remove.contains(p));
    }

    println!("{roll_count}");
}
