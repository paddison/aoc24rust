use std::{
    fmt::Display,
    ops::{Add, AddAssign},
};

pub const DIRS: [Point; 4] = [
    Point::new(0, usize::MAX),
    Point::new(1, 0),
    Point::new(usize::MAX, 0),
    Point::new(0, 1),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn flatten(&self, width: usize) -> usize {
        (self.y * width).wrapping_add(self.x)
    }

    pub fn unflatten(n: usize, width: usize) -> Self {
        Self {
            x: n % width,
            y: n / width,
        }
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
