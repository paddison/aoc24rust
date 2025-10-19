use std::{
    collections::{HashMap, HashSet},
    ops::Add,
    usize,
};

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d12_test");
static INPUT: &str = include_str!("../data/d12");

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn rotate_clockwise(&self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn rotate_counter_clockwise(&self) -> Self {
        match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        }
    }
}

impl Into<Point<isize>> for Direction {
    fn into(self) -> Point<isize> {
        match self {
            Direction::Right => Point { x: 1, y: 0 },
            Direction::Down => Point { x: 0, y: 1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Up => Point { x: 0, y: -1 },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Point<isize> {
    fn move_in_direction(self, direction: Direction) -> Self {
        self + direction.into()
    }
}

impl TryFrom<(usize, usize)> for Point<isize> {
    type Error = ();

    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        if value.0 > isize::MAX as usize || value.1 > isize::MAX as usize {
            Err(())
        } else {
            Ok(Self::new(value.0 as isize, value.1 as isize))
        }
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Region {
    positions: Vec<Point<isize>>,
    tag: char,
}

impl Region {
    fn determine_price(&self) -> usize {
        let mut price_per_position: HashMap<_, _> =
            self.positions.iter().copied().map(|p| (p, 4)).collect();
        let mut seen = HashSet::new();

        for position in &self.positions {
            seen.insert(position);
            for next in get_adjacent_positions(position) {
                if !seen.contains(&next) && price_per_position.contains_key(&next) {
                    price_per_position.entry(*position).and_modify(|v| *v -= 1);
                    price_per_position.entry(next).and_modify(|v| *v -= 1);
                }
            }
        }

        price_per_position.values().sum::<usize>() * self.positions.len()
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
        let lookup: HashSet<_> = self.positions.iter().copied().collect();
        let mut number_of_sides = 0;
        let mut seen = HashSet::new();

        for start_position in self.positions.iter().copied() {
            for start_direction in is_edge_position(&start_position, &lookup)
                .into_iter()
                .filter_map(|dir| dir)
            {
                if !seen.contains(&(start_position, start_direction)) {
                    number_of_sides += determine_number_of_edges(
                        start_direction,
                        start_position,
                        &mut seen,
                        &lookup,
                    );
                }
            }
        }

        number_of_sides * self.positions.len()
    }
}

fn determine_number_of_edges(
    mut current_direction: Direction,
    mut current_position: Point<isize>,
    seen: &mut HashSet<(Point<isize>, Direction)>,
    lookup: &HashSet<Point<isize>>,
) -> usize {
    let mut number_of_sides = 0;

    while seen.insert((current_position, current_direction)) {
        let direction_counter_clockwise = current_direction.rotate_counter_clockwise();
        let counter_clockwise = current_position.move_in_direction(direction_counter_clockwise);

        if lookup.contains(&counter_clockwise) {
            current_direction = direction_counter_clockwise;
            current_position = counter_clockwise;
            number_of_sides += 1;
        } else {
            let forward = current_position.move_in_direction(current_direction);

            if lookup.contains(&forward) {
                current_position = forward;
            } else {
                current_direction = current_direction.rotate_clockwise();
                number_of_sides += 1;
            }
        }
    }

    number_of_sides
}

fn is_edge_position(
    position: &Point<isize>,
    lookup: &HashSet<Point<isize>>,
) -> [Option<Direction>; 4] {
    let mut possible_directions = [None, None, None, None];
    for (i, direction) in [
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Up,
    ]
    .into_iter()
    .enumerate()
    {
        if !lookup.contains(&position.move_in_direction(direction)) {
            possible_directions[i] = Some(direction.rotate_clockwise());
        }
    }

    possible_directions
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn determine_regions(map: Vec<Vec<char>>) -> Vec<Region> {
    let height = map.len();
    let width = map.first().map(|row| row.len()).unwrap_or(0);
    let mut seen: HashSet<Point<isize>> = HashSet::new();
    let mut regions = Vec::new();

    for y in 0..height {
        for x in 0..width {
            match (x, y).try_into() {
                Ok(position) if !seen.contains(&position) => {
                    regions.push(depth_first_search(position, &mut seen, &map));
                }
                _ => continue,
            }
        }
    }

    regions
}

fn depth_first_search(
    start_position: Point<isize>,
    seen: &mut HashSet<Point<isize>>,
    map: &[Vec<char>],
) -> Region {
    let mut queue = vec![start_position];
    let tag = map
        .get(start_position.y as usize)
        .unwrap()
        .get(start_position.x as usize)
        .copied()
        .unwrap();
    let mut positions = Vec::new();

    while let Some(current_position) = queue.pop() {
        if !seen.contains(&current_position) {
            seen.insert(current_position);
            positions.push(current_position);

            for next_position in get_adjacent_positions(&current_position) {
                if let Some(next_tag) = map
                    .get(next_position.y as usize)
                    .and_then(|p| p.get(next_position.x as usize))
                    .copied()
                {
                    if next_tag == tag {
                        queue.push(next_position);
                    }
                }
            }
        }
    }

    Region { positions, tag }
}

fn get_adjacent_positions(position: &Point<isize>) -> [Point<isize>; 4] {
    [
        position.move_in_direction(Direction::Right),
        position.move_in_direction(Direction::Down),
        position.move_in_direction(Direction::Left),
        position.move_in_direction(Direction::Up),
    ]
}

pub fn solve_1() -> usize {
    determine_regions(parse_input(INPUT))
        .into_iter()
        .map(|region| region.determine_price())
        .sum()
}

pub fn solve_2() -> usize {
    determine_regions(parse_input(INPUT))
        .into_iter()
        .map(|region| region.determine_price_surrounding())
        .sum()
}
