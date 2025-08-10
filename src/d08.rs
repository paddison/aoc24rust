use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d08_test");
static INPUT: &str = include_str!("../data/d08");

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn is_contained_in(&self, width: i32, height: i32) -> bool {
        self.x < width && self.x >= 0 && self.y < height && self.y >= 0
    }

    fn dist(&self, other: &Self) -> Point {
        Point {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }

    fn get_antinodes(&self, other: &Self, width: i32, height: i32, limit: i32) -> Vec<Point> {
        let mut antinodes = Vec::new();
        let dist = self.dist(other);
        if limit > 2 {
            antinodes.push(*self);
            antinodes.push(*other);
        }

        for i in 1..limit {
            let dist = Point {
                x: i * dist.x,
                y: i * dist.y,
            };
            let p = Point {
                x: self.x - dist.x,
                y: self.y - dist.y,
            };

            if !p.is_contained_in(width, height) {
                break;
            }

            antinodes.push(p)
        }

        antinodes
    }
}

type Antennas = HashMap<char, Vec<Point>>;

fn parse_input(input: &str) -> (i32, i32, Antennas) {
    assert_eq!(input[input.len() - 1..], *"\n");
    let width = input
        .find('\n')
        .expect("input contains no newline characters");

    let height = input.len() / (width + 1);

    let mut antennas = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, id) in line.chars().enumerate() {
            if id != '.' {
                let points: &mut Vec<Point> = antennas.entry(id).or_default();
                points.push(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    (width as i32, height as i32, antennas)
}

fn get_antinodes(antennas: Antennas, width: i32, height: i32, limit: i32) -> Vec<Point> {
    let mut antinodes = Vec::new();

    for points in antennas.values() {
        for (i, a) in points.iter().enumerate().take(points.len() - 1) {
            for b in &points[i + 1..] {
                let mut antinodes_found = a.get_antinodes(b, width, height, limit);
                antinodes.append(&mut antinodes_found);
                let mut antinodes_found = b.get_antinodes(a, width, height, limit);
                antinodes.append(&mut antinodes_found);
            }
        }
    }

    antinodes
}

fn get_number_of_unique_antinodes(
    antennas: Antennas,
    width: i32,
    height: i32,
    limit: i32,
) -> usize {
    get_antinodes(antennas, width, height, limit)
        .into_iter()
        .collect::<HashSet<Point>>()
        .len()
}

pub fn solve_1() -> usize {
    let (width, height, antennas) = parse_input(INPUT);
    get_number_of_unique_antinodes(antennas, width, height, 2)
}

pub fn solve_2() -> usize {
    let (width, height, antennas) = parse_input(INPUT);
    get_number_of_unique_antinodes(antennas, width, height, width.max(height) + 1)
}
