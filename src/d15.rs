use std::{
    marker::PhantomData,
    ops::{Add, AddAssign},
};

#[allow(dead_code)]
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

trait Tile: Clone + Copy {}

#[derive(Clone, Copy)]
enum Tile1 {
    Floor,
    Wall,
    Box,
}

impl Tile for Tile1 {}

impl From<char> for Tile1 {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Wall,
            'O' => Self::Box,
            _ => Self::Floor,
        }
    }
}

impl From<Tile1> for char {
    fn from(value: Tile1) -> Self {
        match value {
            Tile1::Floor => '.',
            Tile1::Wall => '#',
            Tile1::Box => 'O',
        }
    }
}
#[derive(Clone, Copy)]
enum Tile2 {
    Floor,
    BoxLeft,
    BoxRight,
    Wall,
}

impl From<Tile2> for char {
    fn from(value: Tile2) -> Self {
        match value {
            Tile2::Floor => '.',
            Tile2::Wall => '#',
            Tile2::BoxLeft => '[',
            Tile2::BoxRight => ']',
        }
    }
}

impl Tile for Tile2 {}

struct Map<T: Tile> {
    _inner: Vec<T>,
    width: usize,
    height: usize,
}

#[allow(dead_code)]
fn print<T: Into<char> + Tile>(map: &Map<T>, robot: &Robot<T>) {
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

impl<T: Tile> Map<T> {
    fn new(tiles: Vec<T>, width: usize, height: usize) -> Self {
        Self {
            _inner: tiles,
            width,
            height,
        }
    }
    #[inline(always)]
    fn calculate_index(&self, point: &Point) -> usize {
        self.width * point.y + point.x
    }

    #[inline(always)]
    fn is_in_range(&self, point: &Point) -> bool {
        point.x < self.width || point.y < self.height
    }

    fn get(&self, point: &Point) -> Option<T> {
        if self.is_in_range(point) {
            Some(self._inner[self.calculate_index(point)])
        } else {
            None
        }
    }

    fn set(&mut self, tile: T, point: &Point) {
        if self.is_in_range(point) {
            let index = self.calculate_index(point);
            self._inner[index] = tile;
        }
    }
}

impl Map<Tile1> {
    fn calculate_score(&self) -> usize {
        let mut score = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(Tile1::Box) = self.get(&Point::new(x, y)) {
                    score += 100 * y + x
                }
            }
        }

        score
    }
}

impl Map<Tile2> {
    fn calculate_score(&self) -> usize {
        let mut score = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(Tile2::BoxLeft) = self.get(&Point::new(x, y)) {
                    score += 100 * y + x
                }
            }
        }

        score
    }
}

impl From<Map<Tile1>> for Map<Tile2> {
    fn from(value: Map<Tile1>) -> Self {
        //
        let Map::<Tile1> {
            _inner: tiles,
            width,
            height,
        } = value;

        let mut tiles2 = Vec::new();

        for tile in tiles {
            match tile {
                Tile1::Floor => {
                    tiles2.push(Tile2::Floor);
                    tiles2.push(Tile2::Floor);
                }
                Tile1::Wall => {
                    tiles2.push(Tile2::Wall);
                    tiles2.push(Tile2::Wall);
                }
                Tile1::Box => {
                    tiles2.push(Tile2::BoxLeft);
                    tiles2.push(Tile2::BoxRight);
                }
            }
        }

        Map::<Tile2> {
            _inner: tiles2,
            width: width * 2,
            height,
        }
    }
}

struct Robot<T> {
    position: Point,
    direction: Direction,
    _boo: PhantomData<T>,
}

impl<T: Tile> Robot<T> {
    fn new(x: usize, y: usize, direction: Direction) -> Self {
        Self {
            position: Point::new(x, y),
            direction,
            _boo: PhantomData,
        }
    }

    fn move_in_direction(&mut self, direction: Direction) {
        self.position += direction.into();
    }

    fn turn_in_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}

impl Robot<Tile1> {
    fn move_box(&self, map: &mut Map<Tile1>, current: Tile1, position: Point) -> bool {
        let next_position = position + self.direction.into();
        match map.get(&next_position) {
            Some(Tile1::Floor) => {
                map.set(current, &next_position);
                true
            }
            Some(Tile1::Box) => {
                if self.move_box(map, Tile1::Box, position + self.direction.into()) {
                    map.set(current, &next_position);
                    true
                } else {
                    false
                }
            }
            None | Some(Tile1::Wall) => false,
        }
    }

    fn make_move(&mut self, map: &mut Map<Tile1>, direction: Direction) {
        self.turn_in_direction(direction);
        if self.move_box(map, Tile1::Floor, self.position) {
            self.move_in_direction(direction);
        }
    }
}

impl Robot<Tile2> {
    fn can_move_box(&self, map: &mut Map<Tile2>, position: Point) -> bool {
        let next_position = position + self.direction.into();
        match map.get(&next_position) {
            Some(Tile2::Floor) => true,
            None | Some(Tile2::Wall) => false,
            Some(box_tile) => match self.direction {
                Direction::Up | Direction::Down => match box_tile {
                    Tile2::BoxLeft => {
                        self.can_move_box(map, next_position)
                            && self.can_move_box(map, next_position + Direction::Right.into())
                    }
                    Tile2::BoxRight => {
                        self.can_move_box(map, next_position)
                            && self.can_move_box(map, next_position + Direction::Left.into())
                    }
                    _ => unreachable!(),
                },
                _ => self.can_move_box(map, next_position),
            },
        }
    }

    fn move_box(&self, map: &mut Map<Tile2>, position: Point, tile: Tile2) {
        let next_position = position + self.direction.into();
        match map.get(&next_position) {
            Some(Tile2::Floor) => {
                map.set(tile, &next_position);
            }
            None | Some(Tile2::Wall) => (),
            Some(box_tile) => match self.direction {
                Direction::Up | Direction::Down => {
                    let (other_position, other_box_tile) = match box_tile {
                        Tile2::BoxLeft => {
                            (next_position + Direction::Right.into(), Tile2::BoxRight)
                        }
                        Tile2::BoxRight => (next_position + Direction::Left.into(), Tile2::BoxLeft),
                        _ => unreachable!(),
                    };
                    self.move_box(map, next_position, box_tile);
                    self.move_box(map, other_position, other_box_tile);
                    map.set(Tile2::Floor, &other_position);
                    map.set(tile, &next_position);
                }
                _ => {
                    self.move_box(map, next_position, box_tile);
                    map.set(map.get(&position).unwrap(), &next_position);
                }
            },
        };
    }

    fn make_move(&mut self, map: &mut Map<Tile2>, direction: Direction) {
        self.turn_in_direction(direction);
        if self.can_move_box(map, self.position) {
            self.move_box(map, self.position, Tile2::Floor);
            self.move_in_direction(direction);
        }
    }
}

fn parse_input(input: &str) -> (Map<Tile1>, Robot<Tile1>, Vec<Direction>) {
    let width = input.find('\n').expect("Input has no linebreaks");
    let mut iter = input.lines().enumerate();
    let mut tiles = Vec::new();
    let mut robot = Robot::<Tile1>::new(0, 0, Direction::Up);
    let mut height = 0;

    for (y, line) in iter.by_ref() {
        if line.is_empty() {
            height = y;
            break;
        } else {
            for (x, tile) in line.chars().enumerate() {
                if tile == '@' {
                    robot = Robot::<Tile1>::new(x, y, Direction::Up);
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

    for direction in directions {
        robot.make_move(&mut map, direction);
    }

    map.calculate_score()
}

pub fn solve_2() -> usize {
    let (map, robot, directions) = parse_input(INPUT);

    let mut map: Map<Tile2> = map.into();
    let mut robot: Robot<Tile2> = Robot::<Tile2> {
        position: Point::new(robot.position.x * 2, robot.position.y),
        direction: robot.direction,
        _boo: PhantomData,
    };

    for direction in directions {
        robot.make_move(&mut map, direction);
    }

    map.calculate_score()
}
