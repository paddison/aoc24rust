use std::collections::HashSet;

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d10_test");
static INPUT: &str = include_str!("../data/d10");

struct Map {
    grid: Vec<u8>,
    width: usize,
    height: usize,
}

impl Map {
    fn get(&self, x: usize, y: usize) -> Option<u8> {
        if x >= self.width || y >= self.height {
            None
        } else {
            self.grid.get(y * self.width + x).copied()
        }
    }

    fn is_trailhead(&self, x: usize, y: usize) -> bool {
        self.get(x, y).map(|pos| pos == 0).unwrap_or(false)
    }
}

fn parse_input(input: &str) -> Map {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines.first().map(|line| line.len()).unwrap_or(0);

    let grid = lines
        .iter()
        .flat_map(|line| line.chars())
        .filter_map(|c| c.to_digit(10))
        .map(|n| n as u8)
        .collect();

    Map {
        grid,
        width,
        height,
    }
}

fn is_one_higher(current_height: u8, next_height: u8) -> bool {
    next_height.wrapping_sub(current_height) == 1
}

fn check_and_push_on_stack(
    map: &Map,
    x: usize,
    y: usize,
    current_height: u8,
    stack: &mut Vec<(usize, usize, u8)>,
) {
    if let Some(next_height) = map
        .get(x, y)
        .filter(|next_height| is_one_higher(current_height, *next_height))
    {
        stack.push((x, y, next_height))
    }
}

fn count_trails(map: &Map, x: usize, y: usize, is_part_1: bool) -> usize {
    let mut stack = vec![(x, y, 0)];
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut sum = 0;

    while let Some((x, y, current_height)) = stack.pop() {
        if current_height == 9 && !seen.contains(&(x, y)) {
            sum += 1;
            if is_part_1 {
                seen.insert((x, y));
            }
            continue;
        }

        for (x1, y1) in [
            (x.wrapping_sub(1), y),
            (x, y.wrapping_sub(1)),
            (x + 1, y),
            (x, y + 1),
        ] {
            check_and_push_on_stack(map, x1, y1, current_height, &mut stack);
        }
    }

    sum
}

fn solve(is_part_1: bool) -> usize {
    let map = parse_input(INPUT);
    let mut sum = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if map.is_trailhead(x, y) {
                let result = count_trails(&map, x, y, is_part_1);
                sum += result;
            }
        }
    }

    sum
}

pub fn solve_1() -> usize {
    solve(true)
}

pub fn solve_2() -> usize {
    solve(false)
}

#[test]
fn test_parse() {
    println!("{}", solve_1());
}
