use std::{collections::HashSet, fmt::Display, ops};

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d06_test");
static INPUT: &str = include_str!("../data/d06");

const UP: Point = Point { x: 0, y: -1 };
const LEFT: Point = Point { x: -1, y: 0 };
const DOWN: Point = Point { x: 0, y: 1 };
const RIGHT: Point = Point { x: 1, y: 0 };

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Floor,
    Wall,
}

struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Map {
    fn is_valid_idx(&self, idx: &Point) -> bool {
        let Point { x, y } = *idx;
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }
    fn get(&self, idx: &Point) -> Option<&Tile> {
        if self.is_valid_idx(idx) {
            let Point { x, y } = *idx;
            Some(&self.tiles[x as usize + y as usize * self.width])
        } else {
            None
        }
    }

    fn set(&mut self, idx: &Point, val: Tile) {
        if self.is_valid_idx(idx) {
            let Point { x, y } = *idx;
            self.tiles[x as usize + y as usize * self.width] = val;
        }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Guard {
    dir: Point, // directional vector
    pos: Point, // current position
}

impl Guard {
    fn turn(&mut self) {
        self.dir = self.dir.rotate();
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn rotate(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
}

impl ops::Add for Point {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

struct State {
    map: Map,
    guard: Guard,
}

impl State {
    // This function returns the current position
    // of the guard, not the next one.
    //
    // Also, if the current position is valid,
    // it advances the guard by one step, regardless
    // if it will end up outside of the map.
    //
    // If the guard is moved outside of the map, the
    // next call to advance will return None.
    fn advance(&mut self) -> Option<Point> {
        let State { map, guard } = self;

        if map.get(&guard.pos).is_none() {
            None
        } else {
            let current_pos = guard.pos;
            let mut next_pos = guard.pos + guard.dir;
            let mut next_tile = map.get(&next_pos);

            while let Some(Tile::Wall) = next_tile {
                guard.turn();
                next_pos = guard.pos + guard.dir;
                next_tile = map.get(&next_pos);
            }

            guard.pos = next_pos;

            Some(current_pos)
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt_string = Vec::new();

        for (i, tile) in self.map.tiles.iter().enumerate() {
            let x = i % self.map.width;
            let y = i / self.map.width;

            if i != 0 && x == 0 {
                fmt_string.push('\n');
            }

            if self.guard.pos.x == x as i32 && self.guard.pos.y == y as i32 {
                let c = match self.guard.dir {
                    p if p == UP => '^',
                    p if p == RIGHT => '>',
                    p if p == DOWN => 'v',
                    p if p == LEFT => '<',
                    _ => unreachable!(),
                };
                fmt_string.push(c);
            } else {
                let c = match tile {
                    Tile::Floor => '.',
                    Tile::Wall => '#',
                };
                fmt_string.push(c);
            }
        }
        writeln!(f, "{}", fmt_string.into_iter().collect::<String>())
    }
}

impl Iterator for State {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance()
    }
}

fn parse_input(input: &str) -> State {
    let mut guard = Guard {
        dir: UP,
        pos: Point { x: 0, y: 0 },
    };
    let width = input.find('\n').unwrap();
    let height = input.len() / width - 1; // subtract 1 because of the '\n' characters

    let mut tiles = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            match tile {
                '.' => tiles.push(Tile::Floor),
                '#' => tiles.push(Tile::Wall),
                _ => {
                    assert_eq!(guard.pos.x, 0);
                    assert_eq!(guard.pos.y, 0);
                    guard.pos = Point {
                        x: x as i32,
                        y: y as i32,
                    };
                    tiles.push(Tile::Floor);
                }
            }
        }
    }

    State {
        map: Map {
            tiles,
            width,
            height,
        },
        guard,
    }
}

fn is_cycle(state: &mut State) -> bool {
    let mut seen = HashSet::<Guard>::new();

    while let Some(_) = state.next() {
        let g = state.guard;
        if seen.contains(&g) {
            return true;
        } else {
            seen.insert(state.guard);
        }
    }

    false
}

pub fn solve_1() -> usize {
    parse_input(INPUT).collect::<HashSet<Point>>().len()
}

pub fn solve_2() -> usize {
    let mut state = parse_input(INPUT);
    let start = state.guard;
    // I have to collect these into a Vec.
    // If using a Hashset it sometimes leads to
    // an off by one error (probably due to the order being unstable)
    // and i haven't figured out why yet.
    let mut visited = Vec::new();
    (&mut state).for_each(|p| {
        if !visited.contains(&p) {
            visited.push(p);
        }
    });
    let mut count = 0;

    // Brute force approach
    for point in visited {
        state.map.set(&point, Tile::Wall);

        if is_cycle(&mut state) {
            count += 1;
        }

        // Reset the map
        state.map.set(&point, Tile::Floor);
        state.guard = start;
    }

    count
}
