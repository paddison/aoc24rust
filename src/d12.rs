use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d12_test");
static INPUT: &str = include_str!("../data/d12");

const RIGHT: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const UP: usize = 3;
const NUMBER_OF_DIRECTIONS: usize = 4;

const DIRECTIONS: [(usize, usize); NUMBER_OF_DIRECTIONS] = [
    (1, 0),          // right
    (0, usize::MAX), // down
    (usize::MAX, 0), // left
    (0, 1),          // up
];

#[derive(Debug)]
struct Region {
    positions: Vec<(usize, usize)>,
    tag: char,
}

impl Region {
    fn determine_price(&self) -> usize {
        let mut bla: HashMap<_, _> = self.positions.iter().copied().map(|p| (p, 4)).collect();
        let mut seen = HashSet::new();

        for position in &self.positions {
            seen.insert(position);
            for next in get_next_positions(position) {
                if !seen.contains(&next) && bla.contains_key(&next) {
                    bla.entry(*position).and_modify(|v| *v -= 1);
                    bla.entry(next).and_modify(|v| *v -= 1);
                }
            }
        }

        bla.values().sum::<usize>() * self.positions.len()
    }

    fn determine_price_surrounding(&self) -> usize {
        // Since we're parsing from top left to top right, the first position will always be on the
        // top right.

        // determine the circumference by "walking" along the edges of the region.
        // for the top sides, check if up is available:
        //  - yes: change direction to up
        //  - no: check if right is available:
        //    - yes: continue walking, go to first step. increase counter
        //    - no: change direction to down.
        //  when rotating counter clockwise go to the next tile, and increment
        //  when rotating clockwise change state and increment
        let start = self.positions.first().unwrap();
        let lookup: HashSet<_> = self.positions.iter().copied().collect();
        let mut current = start;
        let mut current_direction = RIGHT;
        let mut circumference = 1;

        loop {
            // check if rotating counter clockwise is possible
            if let Some(counter_clockwise) = lookup.get(&add_positions(
                current,
                &DIRECTIONS[current_direction + NUMBER_OF_DIRECTIONS % NUMBER_OF_DIRECTIONS],
            )) {
                current_direction += NUMBER_OF_DIRECTIONS % NUMBER_OF_DIRECTIONS;
                current = counter_clockwise;
            } else if let Some(straight) =
                lookup.get(&add_positions(current, &DIRECTIONS[current_direction]))
            {
                // try to go normally
                current = straight;
                circumference += 1;
            } else {
                // rotate counter clockwise
                current_direction += 1 % NUMBER_OF_DIRECTIONS;
                circumference += 1;
            };

            if current == start {
                break;
            }
        }

        circumference
    }
}

fn add_positions(position: &(usize, usize), direction: &(usize, usize)) -> (usize, usize) {
    (
        position.0.wrapping_sub(direction.0),
        position.1.wrapping_add(direction.1),
    )
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_next_positions(pos: &(usize, usize)) -> [(usize, usize); 4] {
    [
        (pos.0 + 1, pos.1),
        (pos.0.wrapping_sub(1), pos.1),
        (pos.0, pos.1 + 1),
        (pos.0, pos.1.wrapping_sub(1)),
    ]
}

fn dfs(next: (usize, usize), seen: &mut HashSet<(usize, usize)>, map: &[Vec<char>]) -> Region {
    let mut queue = vec![next];
    let tag = map.get(next.1).unwrap().get(next.0).copied().unwrap();
    let mut positions = Vec::new();

    while let Some(position) = queue.pop() {
        if !seen.contains(&position) {
            seen.insert(position);
            positions.push(position);

            for next_pos in get_next_positions(&position) {
                if let Some(next_tag) = map.get(next_pos.1).and_then(|p| p.get(next_pos.0)).copied()
                {
                    if next_tag == tag {
                        queue.push(next_pos);
                    }
                }
            }
        }
    }

    Region { positions, tag }
}

fn determine_regions(map: Vec<Vec<char>>) -> Vec<Region> {
    let height = map.len();
    let width = map.first().map(|row| row.len()).unwrap_or(0);
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut regions = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let position = (x, y);
            if !seen.contains(&position) {
                regions.push(dfs(position, &mut seen, &map));
            }
        }
    }

    regions
}

pub fn solve_1() -> usize {
    determine_regions(parse_input(INPUT))
        .into_iter()
        .map(|region| region.determine_price())
        .sum()
}

#[test]
fn test() {
    println!("{}", solve_1());
}
