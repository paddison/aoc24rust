use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    ops::{Add, AddAssign},
    rc::Rc,
    usize,
};

static TEST: &str = include_str!("../data/d16_test");
static INPUT: &str = include_str!("../data/d16");

const NUMBER_OF_DIRS: usize = 4;
const DIRS: [Dir; 4] = [Dir::Up, Dir::Right, Dir::Down, Dir::Left];

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn reverse(self) -> Self {
        DIRS[(self as usize + 2) % NUMBER_OF_DIRS]
    }

    fn rotate_clockwise(self) -> Self {
        DIRS[(self as usize + 1) % NUMBER_OF_DIRS]
    }

    fn rotate_counter_clockwise(self) -> Self {
        DIRS[(self as usize).wrapping_add(usize::MAX) % NUMBER_OF_DIRS]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl From<Dir> for Point {
    fn from(value: Dir) -> Self {
        match value {
            Dir::Up => Point::new(0, usize::MAX),
            Dir::Right => Point::new(1, 0),
            Dir::Left => Point::new(usize::MAX, 0),
            Dir::Down => Point::new(0, 1),
        }
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

struct Map {
    map: Vec<bool>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(map: Vec<bool>, width: usize, height: usize) -> Self {
        Self { map, width, height }
    }

    fn get(&self, point: Point) -> bool {
        if point.x < self.width {
            *self.map.get(point.y * self.height + point.x).unwrap()
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    point: Point,
    dir: Dir,
}

impl Node {
    fn new(point: Point, dir: Dir) -> Self {
        Self { point, dir }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PathNode {
    node: Node,
    prev: Option<Rc<PathNode>>,
}

impl PathNode {
    fn new(node: Node, prev: Option<Rc<PathNode>>) -> Self {
        Self { node, prev }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct WeightedNode {
    node: PathNode,
    score: usize,
}

impl WeightedNode {
    fn new(node: PathNode, score: usize) -> Self {
        Self { node, score }
    }
}

impl Ord for WeightedNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for WeightedNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn is_cheapest(node: Node, score: usize, scores: &mut HashMap<Node, usize>) -> bool {
    let min_score = scores.entry(node).or_insert(usize::MAX);

    if score <= *min_score {
        *min_score = score;
        true
    } else {
        false
    }
}

fn turn(
    map: &Map,
    node: PathNode,
    score: usize,
    scores: &mut HashMap<Node, usize>,
) -> Option<Reverse<WeightedNode>> {
    if map.get(node.node.point + node.node.dir.into()) {
        let next_score = score + 1000;
        let next_node = Node::new(node.node.point, node.node.dir);

        if is_cheapest(next_node, next_score, scores) {
            return Some(Reverse(WeightedNode::new(
                PathNode::new(next_node, Some(Rc::new(node))),
                next_score,
            )));
        }
    }

    None
}

fn dijkstra_min(map: &Map) -> usize {
    let start = Point::new(1, map.height - 2);
    let end = Point::new(map.width - 2, 1);
    //let mut queue = BinaryHeap::new();
    let mut scores = HashMap::<Node, usize>::new();
    //let mut min = None;

    0
}

fn dijkstra(map: &Map, abort: bool) -> (usize, Vec<PathNode>) {
    let start = Point::new(1, map.height - 2);
    let end = Point::new(map.width - 2, 1);
    let mut queue = BinaryHeap::new();
    let mut scores = HashMap::<Node, usize>::new();
    let mut min = None;
    let mut paths = Vec::new();
    let mut pred = HashMap::<Node, Vec<Node>>::new();

    queue.push(Reverse(WeightedNode::new(
        PathNode::new(Node::new(start, Dir::Right), None),
        0,
    )));

    while let Some(Reverse(WeightedNode { node, score })) = queue.pop() {
        let point = node.node.point;
        let dir = node.node.dir;
        if point == end {
            if abort {
                return (score, paths);
            }
            match min {
                None => {
                    min = Some(score);
                    paths.push(node);
                }
                Some(m) if m == score => paths.push(node),
                Some(m) => return (m, paths),
            }
            continue;
        }

        let mut cl = node.clone();
        cl.node.dir = cl.node.dir.rotate_clockwise();
        if let Some(weighted_node) = turn(map, cl, score, &mut scores) {
            queue.push(weighted_node);
        }
        let mut ccl = node.clone();
        ccl.node.dir = ccl.node.dir.rotate_counter_clockwise();
        if let Some(weighted_node) = turn(map, ccl, score, &mut scores) {
            queue.push(weighted_node);
        }

        let next_point = point + dir.into();

        if map.get(next_point) {
            let next_score = score + 1;
            let next_node = Node::new(next_point, dir);
            if is_cheapest(next_node, next_score, &mut scores) {
                queue.push(Reverse(WeightedNode::new(
                    PathNode::new(next_node, Some(Rc::new(node))),
                    next_score,
                )));
            }
        }
    }

    (0, paths)
}

fn parse(input: &str) -> Map {
    let width = input.find('\n').unwrap_or(0);
    let lines = input.lines().collect::<Vec<&str>>();
    let height = lines.len();
    let map = lines
        .into_iter()
        .flat_map(|line| line.chars())
        .map(|char| char != '#')
        .collect();

    Map::new(map, width, height)
}

fn collect_best_tiles(paths: Vec<PathNode>) -> HashSet<Point> {
    let mut points = HashSet::new();
    for mut node in paths {
        points.insert(node.node.point);
        while let Some(prev) = node.prev {
            points.insert(prev.node.point);
            node = Rc::unwrap_or_clone(prev);
        }
    }

    points
}

pub fn solve_1() -> usize {
    dijkstra(&parse(INPUT), true).0
}

pub fn solve_2() -> usize {
    let paths = dijkstra(&parse(INPUT), false).1;
    println!("{}", paths.len());
    collect_best_tiles(paths).len()
}

#[test]
fn test() {
    println!("{}", solve_2());
}
