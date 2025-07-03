use std::ops;

static TEST: &str = include_str!("../data/d06_test");

const Up: Point = Point { x: 0, y: -1 };

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

enum Tile {
    Floor,
    Wall,
}

struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

struct Guard {
    dir: Point, // directional vector
    pos: Point, // current position
}

impl Guard {
    fn step(&mut self) {
        self.pos += self.dir;
    }

    fn turn(&mut self) {
        self.dir = self.dir.rotate();
    }
}

#[derive(Clone, Copy)]
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
