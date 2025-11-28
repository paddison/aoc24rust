use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    iter::FilterMap,
    ops::{Add, AddAssign},
    str::Lines,
};

static TEST: &str = include_str!("../data/d18_test");
static INPUT: &str = include_str!("../data/d18");

const SIZE_TEST: usize = 7;
const SIZE: usize = 71;

const DIRS: [Point; 4] = [
    Point::new(0, usize::MAX),
    Point::new(1, 0),
    Point::new(usize::MAX, 0),
    Point::new(0, 1),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl TryFrom<&str> for Point {
    type Error = ();
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let delim = input.find(',').ok_or(())?;
        let x = input[..delim].parse::<usize>().map_err(|_| ())?;
        let y = input[delim + 1..].parse::<usize>().map_err(|_| ())?;

        Ok(Point::new(x, y))
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x.wrapping_add(rhs.x), self.y.wrapping_add(rhs.y))
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x.wrapping_add(rhs.x);
        self.y = self.y.wrapping_add(rhs.y);
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

struct Map<const N: usize> {
    tiles: [[bool; N]; N],
}

impl<const N: usize> Display for Map<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in self.tiles {
            s.push_str(
                &row.iter()
                    .map(|tile| if *tile { '.' } else { '#' })
                    .collect::<String>(),
            );
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}

impl<const N: usize> Map<N> {
    fn new(tiles: [[bool; N]; N]) -> Self {
        Self { tiles }
    }

    fn get(&self, Point { x, y }: Point) -> bool {
        if x < N && y < N {
            self.tiles[y][x]
        } else {
            false
        }
    }

    fn set(&mut self, val: bool, Point { x, y }: Point) {
        if x < N && y < N {
            self.tiles[y][x] = val;
        }
    }
}

fn parse<const N: usize, const M: usize>(
    input: &'static str,
) -> (Box<dyn Iterator<Item = Point>>, Map<N>) {
    fn my_filter(line: &str) -> Option<Point> {
        line.try_into().ok()
    }

    let mut tiles_vec = Box::new(input.lines().filter_map(my_filter));

    let mut map = Map::<N>::new([[true; N]; N]);

    for point in (&mut tiles_vec).take(M) {
        map.set(false, point);
    }

    (tiles_vec, map)
}

fn bfs<const N: usize>(map: &Map<N>) -> Option<usize> {
    let mut queue = VecDeque::<(Point, usize)>::new();
    let start = Point::new(0, 0);
    let end = Point::new(N - 1, N - 1);
    let mut seen = HashSet::from([start]);

    queue.push_back((start, 0));

    while let Some((pos, cost)) = queue.pop_front() {
        if pos == end {
            return Some(cost);
        }

        let next_cost = cost + 1;

        for dir in DIRS {
            let next = pos + dir;
            if map.get(next) && seen.insert(next) {
                queue.push_back((next, next_cost));
            }
        }
    }

    None
}

pub fn solve_1() -> usize {
    let (_, map) = parse::<SIZE, 1024>(INPUT);
    bfs(&map).unwrap()
}

pub fn solve_2() -> String {
    let (remaining_blocks, mut map) = parse::<SIZE, 1024>(INPUT);

    for block in remaining_blocks {
        map.set(false, block);

        if bfs(&map).is_none() {
            return block.to_string();
        }
    }

    unreachable!();
}
