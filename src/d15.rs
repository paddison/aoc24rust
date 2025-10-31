use std::ops::{Add, AddAssign};

static TEST: &str = include_str!("../data/d15_test");
static INPUT: &str = include_str!("../data/d15");

const UP: Point = Point {
    x: 0,
    y: usize::MAX,
};
const RIGHT: Point = Point { x: 1, y: 0 };
const DOWN: Point = Point { x: 0, y: 1 };
const LEFT: Point = Point {
    x: usize::MAX,
    y: 0,
};

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            _ => Self::Left,
        }
    }
}

impl From<Direction> for char {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl From<Direction> for Point {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => UP,
            Direction::Right => RIGHT,
            Direction::Down => DOWN,
            Direction::Left => LEFT,
        }
    }
}

impl From<&Direction> for Point {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Up => UP,
            Direction::Right => RIGHT,
            Direction::Down => DOWN,
            Direction::Left => LEFT,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.wrapping_add(rhs.x),
            y: self.y.wrapping_add(rhs.y),
        }
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x.wrapping_add(rhs.x),
            y: self.y.wrapping_add(rhs.y),
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x.wrapping_add(rhs.x);
        self.y = self.y.wrapping_add(rhs.y);
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Floor,
    Wall,
    Box,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            'O' => Self::Box,
            _ => Self::Floor,
        }
    }
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Floor => '.',
            Tile::Wall => '#',
            Tile::Box => 'O',
        }
    }
}

struct Map {
    _inner: Vec<Tile>,
    width: usize,
    height: usize,
}

fn print(map: &Map, robot: &Robot) {
    let mut string = String::new();

    for y in 0..map.height {
        for x in 0..map.width {
            let point = Point::new(x, y);

            if point == robot.position {
                string.push('@');
            } else {
                string.push(map.get(&point).unwrap().into());
            }
        }
        string.push('\n');
    }

    println!("{string}");
}

struct Robot {
    position: Point,
    direction: Direction,
}

impl Robot {
    fn new(x: usize, y: usize, direction: Direction) -> Self {
        Self {
            position: Point::new(x, y),
            direction,
        }
    }
    fn move_box(&self, map: &mut Map, current: Tile, position: Point) -> bool {
        let next_position = position + self.direction.into();
        match map.get(&next_position) {
            Some(Tile::Floor) => {
                map.set(current, &next_position);
                true
            }
            Some(Tile::Box) => {
                if self.move_box(map, Tile::Box, position + self.direction.into()) {
                    map.set(current, &next_position);
                    true
                } else {
                    false
                }
            }
            None | Some(Tile::Wall) => false,
        }
    }

    fn move_in_direction(&mut self, direction: Direction) {
        self.position += direction.into();
    }

    fn turn_in_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn make_move(&mut self, map: &mut Map, direction: Direction) {
        self.turn_in_direction(direction);
        if self.move_box(map, Tile::Floor, self.position) {
            self.move_in_direction(direction);
        }
    }
}

impl Map {
    fn new(tiles: Vec<Tile>, width: usize, height: usize) -> Self {
        Self {
            _inner: tiles,
            width,
            height,
        }
    }
    #[inline(always)]
    fn calculate_index(&self, point: &Point) -> usize {
        self.height * point.y + point.x
    }

    #[inline(always)]
    fn is_in_range(&self, point: &Point) -> bool {
        point.x < self.width || point.y < self.height
    }

    fn get(&self, point: &Point) -> Option<Tile> {
        if self.is_in_range(point) {
            Some(self._inner[self.calculate_index(point)])
        } else {
            None
        }
    }

    fn set(&mut self, tile: Tile, point: &Point) {
        if self.is_in_range(point) {
            let index = self.calculate_index(point);
            self._inner[index] = tile;
        }
    }

    fn calculate_score(&self) -> usize {
        let mut score = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(Tile::Box) = self.get(&Point::new(x, y)) {
                    score += 100 * y + x
                }
            }
        }

        score
    }
}

fn parse_input(input: &str) -> (Map, Robot, Vec<Direction>) {
    let width = input.find('\n').expect("Input has no linebreaks");
    let mut iter = input.lines().enumerate();
    let mut tiles = Vec::new();
    let mut robot = Robot::new(0, 0, Direction::Up);
    let mut height = 0;

    for (y, line) in iter.by_ref() {
        if line.is_empty() {
            height = y;
            break;
        } else {
            for (x, tile) in line.chars().enumerate() {
                if tile == '@' {
                    robot = Robot::new(x, y, Direction::Up);
                }
                tiles.push(tile.into());
            }
        }
    }

    let directions = iter
        .flat_map(|line| line.1.chars())
        .map(|direction| direction.into())
        .collect();

    let map = Map::new(tiles, width, height);

    assert!(height > 0);

    (map, robot, directions)
}

pub fn solve_1() -> usize {
    let (mut map, mut robot, directions) = parse_input(INPUT);

    print(&map, &robot);
    for direction in directions {
        println!("{}", char::from(direction));
        robot.make_move(&mut map, direction);
        print(&map, &robot);
    }

    map.calculate_score()
}

#[test]
fn test() {
    println!("{}", solve_1());
}
