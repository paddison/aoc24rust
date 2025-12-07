use std::fmt::Display;

use crate::sizes;

use crate::util::point::{Point, DIRS};

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d20_test");
static INPUT: &str = include_str!("../data/d20");

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum Tile {
    Start,
    End,
    Floor,
    #[default]
    Wall,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Start,
            'E' => Self::End,
            '.' => Self::Floor,
            _ => Self::Wall,
        }
    }
}

struct BitMapIdx {
    array_index: usize,
    indexed_bit_in_array_index: u8,
}

/// S has to be a power of 2
#[derive(Debug)]
struct BitMap<const S: usize> {
    tiles: [u8; S],
}

impl<const S: usize> BitMap<S> {
    const DIM: usize = (S * 8) >> ((S * 8).trailing_zeros() / 2);
}

impl<const S: usize> BitMap<S> {
    fn new() -> Self {
        Self { tiles: [0; S] }
    }

    fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < Self::DIM && y < Self::DIM
    }

    fn get_index(&self, x: usize, y: usize) -> BitMapIdx {
        let indexed_bit = y * Self::DIM + x;
        let array_index = indexed_bit / 8;
        let indexed_bit_in_array_index = 1 << (7 - (indexed_bit % 8));
        BitMapIdx {
            array_index,
            indexed_bit_in_array_index,
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        if self.is_in_bounds(x, y) {
            let idx = self.get_index(x, y);
            self.tiles[idx.array_index] & idx.indexed_bit_in_array_index != 0
        } else {
            false
        }
    }

    fn set(&mut self, x: usize, y: usize) {
        if self.is_in_bounds(x, y) {
            let idx = self.get_index(x, y);
            self.tiles[idx.array_index] |= idx.indexed_bit_in_array_index
        }
    }
}

impl<const D: usize> Display for BitMap<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = D / Self::DIM;
        let height = D / width;
        let mut s = String::new();
        for y in 0..height {
            for x in 0..width {
                s.push_str(&format!("{:08b}", self.tiles[y * width + x]));
            }
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}

struct RhombusIterator {
    center: Point,
    current: Point,
    i: usize,
}

impl RhombusIterator {
    const DIRS: [Point; 4] = [
        Point::new(usize::MAX, 1),
        Point::new(usize::MAX, usize::MAX),
        Point::new(1, usize::MAX),
        Point::new(1, 1),
    ];

    fn new(center: Point, length: usize) -> Self {
        Self {
            current: Point::new(center.x + length, center.y),
            center,
            i: 0,
        }
    }
}

impl Iterator for RhombusIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 4 {
            None
        } else {
            let next = self.current;
            self.current = self.current + Self::DIRS[self.i];
            if self.current.x == self.center.x || self.current.y == self.center.y {
                self.i += 1;
            }
            Some(next)
        }
    }
}

fn parse_bit_map<const D: usize>(input: &str) -> (BitMap<D>, Point, Point) {
    let mut bit_map = BitMap::new();
    let mut start = Point::new(0, 0);
    let mut end = Point::new(0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            match tile {
                '#' => (),
                tile => {
                    bit_map.set(x, y);
                    match tile {
                        'S' => start = Point::new(x, y),
                        'E' => end = Point::new(x, y),
                        _ => (),
                    }
                }
            }
        }
    }

    (bit_map, start, end)
}

fn dfs<const BM_ARRAY_SIZE: usize, const AREA: usize>(
    map: &BitMap<BM_ARRAY_SIZE>,
    start: Point,
) -> [Option<u16>; AREA] {
    let mut distances = [None; AREA];
    let mut current_opt = Some((start, 0));

    while let Some((current, cost)) = current_opt.take() {
        distances[current.flatten(BitMap::<BM_ARRAY_SIZE>::DIM)] = Some(cost);

        for dir in DIRS {
            let next = current + dir;
            if map.get(next.x, next.y)
                && distances[next.flatten(BitMap::<BM_ARRAY_SIZE>::DIM)].is_none()
            {
                current_opt = Some((next, cost + 1));
            }
        }
    }

    distances
}

fn cheat(
    distances: &[Option<u16>],
    max_cheat_length: u16,
    map_size: usize,
    path_length: u16,
    min_length_to_be_saved: u16,
) -> usize {
    let filter_costs = |(i, cost_opt): (usize, &Option<u16>)| match *cost_opt {
        Some(cost) => Some((i, cost)),
        _none => None,
    };

    let mut number_of_cheats_saving_time = 0;

    for (start_flat, cost_from_start) in distances
        .iter()
        .enumerate()
        .filter_map(filter_costs)
        .filter(|(_, cost)| *cost < path_length - min_length_to_be_saved)
    {
        let start = Point::unflatten(start_flat, map_size);

        for cheat_length in 2..=max_cheat_length {
            number_of_cheats_saving_time += RhombusIterator::new(start, cheat_length as usize)
                .filter(|point| point.x < map_size && point.y < map_size)
                .filter_map(|point| distances[point.flatten(map_size)])
                .filter(|cost_from_cheat_end| *cost_from_cheat_end > cost_from_start + cheat_length)
                .filter(|cost_from_cheat_end| {
                    *cost_from_cheat_end - cost_from_start - cheat_length >= min_length_to_be_saved
                })
                .count();
        }
    }

    number_of_cheats_saving_time
}

fn solve(max_cheat_length: u16, min_saved_cost: u16) -> usize {
    sizes!(256, 256);
    let (m, start, end) = parse_bit_map::<SIZE_ARRAY>(INPUT);

    let distances = dfs::<SIZE_ARRAY, AREA>(&m, start);
    let total_cost = distances[end.flatten(WIDTH)].unwrap();

    cheat(
        &distances,
        max_cheat_length,
        WIDTH,
        total_cost,
        min_saved_cost,
    )
}

pub fn solve_1() -> usize {
    solve(2, 100)
}

pub fn solve_2() -> usize {
    solve(20, 100)
}

#[macro_export]
macro_rules! sizes {
    ($width:expr, $height:expr) => {
        const WIDTH: usize = $width;
        const HEIGHT: usize = $height;
        const AREA: usize = WIDTH * HEIGHT;
        assert_eq!(AREA % 8, 0);
        const SIZE_ARRAY: usize = AREA / 8;
    };
}
