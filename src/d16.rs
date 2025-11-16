use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    ops::{Add, AddAssign},
};

#[allow(dead_code)]
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
            *self.map.get(point.y * self.width + point.x).unwrap()
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct DiGraph<N, E> {
    nodes: Vec<Nodee<N>>,
    edges: Vec<Edge<E>>,
}

impl<N, E> DiGraph<N, E> {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn add_node(&mut self, weight: N) -> usize {
        self.nodes.push(Nodee::new(weight));
        self.nodes.len() - 1
    }

    fn add_edge(&mut self, head: usize, tail: usize, weight: E) -> Option<usize> {
        // prepend the edge
        let _ = self.nodes.get(tail)?;
        let head_node = self.nodes.get_mut(head)?;
        let previous_edge = head_node.edges;
        let edge = Edge::new(head, tail, previous_edge, weight);
        head_node.edges = self.edges.len();
        self.edges.push(edge);

        Some(self.edges.len() - 1)
    }

    fn get_neighbours(&self, node_index: usize) -> Vec<usize> {
        let mut neighbours = Vec::new();
        let Some(node) = self.nodes.get(node_index) else {
            return Vec::new();
        };

        let mut edge_index = node.edges;
        while edge_index != usize::MAX {
            let edge = &self.edges[edge_index];
            neighbours.push(edge.tail);
            edge_index = edge.next;
        }

        neighbours
    }

    fn get_edge(&self, head: usize, tail: usize) -> Option<usize> {
        let mut edge_index = self.nodes.get(head)?.edges;

        while edge_index != usize::MAX {
            let edge = &self.edges[edge_index];
            if edge.tail == tail {
                return Some(edge_index);
            } else {
                edge_index = edge.next;
            }
        }
        None
    }

    fn get_node_weight(&self, node_index: usize) -> Option<&N> {
        self.nodes.get(node_index).map(|node| &node.weight)
    }

    fn get_edge_weight(&self, edge_index: usize) -> Option<&E> {
        self.edges.get(edge_index).map(|edge| &edge.weight)
    }
}

#[derive(Debug)]
struct Nodee<T> {
    weight: T,
    /// Next outgoing edge
    edges: usize,
}

impl<T> Nodee<T> {
    fn new(weight: T) -> Self {
        Self {
            weight,
            edges: usize::MAX,
        }
    }
}

#[derive(Debug)]
struct Edge<T> {
    head: usize,
    tail: usize,
    next: usize,
    weight: T,
}

impl<T> Edge<T> {
    fn new(head: usize, tail: usize, next: usize, weight: T) -> Self {
        Self {
            head,
            tail,
            next,
            weight,
        }
    }
}

fn get_next(node: &Node, cost: usize, map: &Map) -> Vec<(Node, usize)> {
    let cl = node.dir.rotate_clockwise();
    let ccl = node.dir.rotate_counter_clockwise();

    [
        (Node::new(node.point + node.dir.into(), node.dir), cost + 1),
        (Node::new(node.point + cl.into(), cl), cost + 1001),
        (Node::new(node.point + ccl.into(), ccl), cost + 1001),
    ]
    .into_iter()
    .filter(|n| map.get(n.0.point))
    .collect::<Vec<_>>()
}

fn build_graph(map: &Map) -> DiGraph<Node, usize> {
    let start_node = Node::new(Point::new(1, map.height - 2), Dir::Right);
    let end_point = Point::new(map.width - 2, 1);
    let mut seen_nodes = HashMap::new();
    let mut seen_edges = HashSet::new();
    let mut graph = DiGraph::<Node, usize>::new();
    let start_index = graph.add_node(start_node);
    seen_nodes.insert(start_node, start_index);
    let mut queue = vec![start_index];

    while let Some(origin_index) = queue.pop() {
        let cur = *graph.get_node_weight(origin_index).unwrap();

        if cur.point == end_point {
            continue;
        }

        let mut next_nodes_glob = get_next(&cur, 0, map);

        while next_nodes_glob.len() == 1 {
            let (cur, cost) = next_nodes_glob[0];
            if cur.point == end_point {
                next_nodes_glob = vec![(cur, cost)];
                break;
            }
            next_nodes_glob = get_next(&cur, cost, map);
        }

        if !next_nodes_glob.is_empty() {
            for (node, cost) in next_nodes_glob {
                let tail_index = *seen_nodes
                    .entry(node)
                    .or_insert_with(|| graph.add_node(node));
                if seen_edges.insert((origin_index, tail_index)) {
                    graph.add_edge(origin_index, tail_index, cost);
                    queue.push(tail_index);
                }
            }
        }
    }

    graph
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct WeightedNode {
    node: Node,
    score: usize,
}

impl WeightedNode {
    fn new(node: Node, score: usize) -> Self {
        Self { node, score }
    }
}

impl Ord for WeightedNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    node_index: usize,
    cost: usize,
}

impl State {
    fn new(node_index: usize, cost: usize) -> Self {
        Self { node_index, cost }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra_graph(graph: &DiGraph<Node, usize>, end: Point) -> (usize, usize, Vec<Vec<usize>>) {
    let start_state = State::new(0, 0);

    let mut queue = BinaryHeap::new();
    let mut min = usize::MAX;
    let mut costs = vec![usize::MAX; graph.nodes.len()];
    let mut predecessor_lookup = graph.nodes.iter().map(|_| Vec::new()).collect::<Vec<_>>();
    let mut end_index = usize::MAX;

    queue.push(start_state);

    while let Some(State { node_index, cost }) = queue.pop() {
        let node = graph.get_node_weight(node_index).unwrap();

        if cost > min {
            break;
        } else if node.point == end {
            min = cost;
            end_index = node_index;
            continue;
        }

        for neighbor in graph.get_neighbours(node_index) {
            let edge_index = graph.get_edge(node_index, neighbor).unwrap();
            let next_cost = cost + graph.get_edge_weight(edge_index).unwrap();

            if costs[neighbor] > next_cost {
                costs[neighbor] = next_cost;
                predecessor_lookup[neighbor] = vec![node_index];
                queue.push(State::new(neighbor, next_cost));
            } else if costs[neighbor] == next_cost {
                predecessor_lookup[neighbor].push(node_index);
                queue.push(State::new(neighbor, next_cost));
            }
        }
    }

    (min, end_index, predecessor_lookup)
}

fn dijkstra(map: &Map) -> (usize, HashMap<Node, Vec<Node>>) {
    let start = Point::new(1, map.height - 2);
    let end = Point::new(map.width - 2, 1);
    let mut queue = BinaryHeap::new();
    let mut scores = HashMap::<Node, usize>::new();
    let mut min = usize::MAX;
    let mut predecessor_lookup = HashMap::<Node, Vec<Node>>::new();

    queue.push(WeightedNode::new(Node::new(start, Dir::Right), 0));

    while let Some(WeightedNode {
        node: Node { point, dir },
        score,
    }) = queue.pop()
    {
        if score > min {
            return (min, predecessor_lookup);
        }

        if point == end {
            if min > score {
                min = score
            }
            continue;
        }

        for (next_dir, next_score) in [
            (dir, score + 1),
            (dir.rotate_clockwise(), score + 1001),
            (dir.rotate_counter_clockwise(), score + 1001),
        ] {
            let next_point = point + next_dir.into();
            if map.get(next_point) {
                let next_node = Node::new(next_point, next_dir);
                if is_cheapest(next_node, next_score, &mut scores) {
                    predecessor_lookup
                        .entry(next_node)
                        .or_insert(Vec::new())
                        .push(Node::new(point, dir));
                    queue.push(WeightedNode::new(next_node, next_score));
                }
            }
        }
    }

    unreachable!();
}

fn count_nodes_graph(
    graph: &DiGraph<Node, usize>,
    end_index: usize,
    predecessor_lookup: Vec<Vec<usize>>,
) -> usize {
    let mut number_of_positions = 0;
    let mut sub = 0;
    let mut seen = vec![false; graph.edges.len()];
    let mut seen2 = vec![false; graph.nodes.len()];

    let mut queue = vec![end_index];

    while let Some(node_index) = queue.pop() {
        if seen[node_index] {
            continue;
        }

        let mut a = graph.get_node_weight(node_index).unwrap().point;

        let predecessors = predecessor_lookup[node_index]
            .iter()
            .collect::<HashSet<_>>();
        //println!("{:?}", predecessors);
        for predecessor in &predecessors {
            let mut b = graph.get_node_weight(**predecessor).unwrap().point;
            let edge = graph.get_edge(**predecessor, node_index).unwrap();
            if !seen[edge] {
                seen[edge] = true;
                let weight = graph.get_edge_weight(edge).unwrap();
                number_of_positions += weight % 1000; //graph.get_edge_weight(edge).unwrap() % 1000;
                println!("{:?}->{:?}", b, a);
                queue.push(**predecessor);
            }
        }
        if predecessors.len() > 1 {
            number_of_positions -= predecessors.len() - 1;
        }

        seen2[node_index] = true;
    }

    number_of_positions
}

fn count_nodes(map: Map, predecessor_map: HashMap<Node, Vec<Node>>) -> usize {
    let end = Point::new(map.width - 2, 1);
    let mut seen = HashSet::new();
    let mut paths = HashSet::new();
    let mut queue = Vec::new();
    paths.insert(end);

    for end in [Node::new(end, Dir::Up), Node::new(end, Dir::Right)] {
        if let Some(predecessor) = predecessor_map.get(&end) {
            predecessor.iter().for_each(|n| queue.push(*n));
            while let Some(n) = queue.pop() {
                if seen.insert(n) {
                    paths.insert(n.point);
                    for nn in predecessor_map.get(&n).unwrap_or(&Vec::new()) {
                        queue.push(*nn);
                    }
                }
            }
        }
    }

    paths.len()
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

pub fn solve_1() -> usize {
    dijkstra(&parse(INPUT)).0
}

pub fn solve_2() -> usize {
    let map = parse(TEST);
    let graph = build_graph(&map);
    let end = Point::new(map.width - 2, 1);
    let (min, end_index, lookup) = dijkstra_graph(&graph, end);
    count_nodes_graph(&graph, end_index, lookup)
}

#[test]
fn test() {
    solve_2();
    let map = parse(TEST);
    let graph = build_graph(&map);
    let end = Point::new(map.width - 2, 1);
    for (node_index, node) in graph.nodes.iter().enumerate() {
        let neighbors = graph
            .get_neighbours(node_index)
            .into_iter()
            .map(|idx| graph.get_node_weight(idx).unwrap())
            .collect::<Vec<_>>();
        println!(
            "({},{}), {:?}",
            node.weight.point.x, node.weight.point.y, node.weight.dir
        );
        for neighbor in neighbors {
            println!(
                "-->({},{}), {:?}",
                neighbor.point.x, neighbor.point.y, neighbor.dir
            );
        }
    }

    let (min, end_index, lookup) = dijkstra_graph(&graph, end);
    println!("{min}");

    for n in &graph.edges {
        println!("{:?}", n);
    }
}
