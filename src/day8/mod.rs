use std::{collections::HashSet, fmt};

#[derive(Debug)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3D {
    fn distance_to(&self, other: &Point3D) -> i64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)).isqrt()
    }
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}

impl From<&str> for Point3D {
    fn from(s: &str) -> Self {
        let nums: Vec<i64> = s
            .split(',')
            .map(|v| v.trim().parse::<i64>().unwrap())
            .collect();

        Self {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        }
    }
}

#[derive(Debug)]
struct Edge {
    a: usize,
    b: usize,
    d2: i64,
}

pub fn execute_part_1() {
    let content = include_str!("input");
    let points: Vec<Point3D> = content.lines().map(|l| l.into()).collect();
    let mut edges = vec![];

    for (i, curr_point) in points.iter().enumerate() {
        for (j, next_point) in points.iter().enumerate().skip(i + 1) {
            edges.push(Edge {
                a: i,
                b: j,
                d2: curr_point.distance_to(next_point),
            });
        }
    }

    edges.sort_by(|e1, e2| e1.d2.cmp(&e2.d2));

    for edge in edges {
    }

}

pub fn execute_part_2() {}
