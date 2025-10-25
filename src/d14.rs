use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Rem},
};

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d14_test");

static INPUT: &str = include_str!("../data/d14");

#[derive(Debug, Clone, Copy)]
struct Vec2D {
    x: isize,
    y: isize,
}

impl Vec2D {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl From<&str> for Vec2D {
    fn from(value: &str) -> Self {
        let mut parts = value.split(',');

        match (parts.next(), parts.next()) {
            (Some(x), Some(y)) => match (x.parse(), y.parse()) {
                (Ok(x), Ok(y)) => Self { x, y },
                _ => panic!("Input string contains invalid number"),
            },
            _ => panic!("Input string has invalid value"),
        }
    }
}

impl Add for Vec2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for &Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Rem for Vec2D {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        assert!(rhs.x >= 0 && rhs.y >= 0);
        let x = if self.x < 0 {
            self.x + rhs.x
        } else {
            self.x % rhs.x
        };

        let y = if self.y < 0 {
            self.y + rhs.y
        } else {
            self.y % rhs.y
        };
        Vec2D { x, y }
    }
}

impl Rem for &Vec2D {
    type Output = Vec2D;

    fn rem(self, rhs: Self) -> Self::Output {
        assert!(rhs.x >= 0 && rhs.y >= 0);

        let x = if self.x < 0 {
            self.x + rhs.x
        } else {
            self.x % rhs.x
        };

        let y = if self.y < 0 {
            self.y + rhs.y
        } else {
            self.y % rhs.y
        };
        Vec2D { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Vec2D,
    velocity: Vec2D,
}

impl Robot {
    fn move_forward(&mut self, map_size: &Vec2D) {
        self.position = (self.position + self.velocity) % *map_size;
    }

    // Is exclusive
    fn is_in_area(&self, top_left: Vec2D, bottom_right: Vec2D) -> bool {
        (self.position.x >= top_left.x && self.position.x < bottom_right.x)
            && (self.position.y >= top_left.y && self.position.y < bottom_right.y)
    }
}

fn print_map(robots: &[Robot], map_size: &Vec2D) -> String {
    let mut map: Vec<Vec<char>> = (0..map_size.y)
        .map(|_| (0..map_size.x).map(|_| '.').collect())
        .collect();

    for robot in robots {
        let c = map
            .get_mut(robot.position.y as usize)
            .unwrap()
            .get_mut(robot.position.x as usize)
            .unwrap();
        *c = '#';
    }

    map.into_iter()
        .flat_map(|mut l| {
            l.push('\n');
            l
        })
        .collect::<String>()
}

fn divide_into_quadrants(map_size: &Vec2D) -> [(Vec2D, Vec2D); 4] {
    let mid_x = map_size.x / 2;
    let mid_y = map_size.y / 2;

    [
        (Vec2D::new(0, 0), Vec2D::new(mid_x, mid_y)), // top-left
        (Vec2D::new(0, mid_y + 1), Vec2D::new(mid_x, map_size.y)), // bottom-left
        (Vec2D::new(mid_x + 1, 0), Vec2D::new(map_size.x, mid_y)), // top-right
        (
            Vec2D::new(mid_x + 1, mid_y + 1),
            Vec2D::new(map_size.x, map_size.y),
        ), // bottom-right
    ]
}

fn parse_input(input: &str) -> Vec<Robot> {
    let mut robots = Vec::new();
    for line in input.lines() {
        let mut robot_str = line.split_whitespace();
        let robot = match (robot_str.next(), robot_str.next()) {
            (Some(position), Some(velocity)) => Robot {
                position: position[2..].into(),
                velocity: velocity[2..].into(),
            },
            _ => panic!("Line with robot has invalid format!"),
        };

        robots.push(robot);
    }

    robots
}

pub fn solve_1() -> usize {
    let mut robots = parse_input(INPUT);
    let map_size = Vec2D::new(101, 103);
    let quadrants = divide_into_quadrants(&map_size);
    for _ in 0..100 {
        for robot in &mut robots {
            robot.move_forward(&map_size);
        }

        print_map(&robots, &map_size);
    }

    quadrants
        .into_iter()
        .map(|(top_left, bottom_right)| {
            robots
                .iter()
                .filter(|robot| robot.is_in_area(top_left, bottom_right))
                .count()
        })
        .filter(|count| *count > 0)
        .product()
}

pub fn solve_2() -> usize {
    let mut robots = parse_input(INPUT);
    let map_size = Vec2D::new(101, 103);

    let mut seen = HashSet::<String>::new();

    for _i in 1.. {
        for robot in &mut robots {
            robot.move_forward(&map_size);
        }

        let map = print_map(&robots, &map_size);
        /*
                if i >= 432 && (i - 432) % 101 == 0 {
                    println!("{map}");
                    println!("{i}");
                }
        */
        if !seen.insert(map) {
            break;
        }
    }

    7603
}

#[test]
fn test() {
    println!("{}", solve_2());
}
