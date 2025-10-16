use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d12_test");
static INPUT: &str = include_str!("../data/d12");

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
