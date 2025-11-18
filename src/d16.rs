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

    fn get_start(&self) -> Point {
        Point::new(1, self.height - 2)
    }

    fn get_end(&self) -> Point {
        Point::new(self.width - 2, 1)
    }
}

#[derive(Debug)]
struct DiGraph<N, E> {
    nodes: Vec<Node<N>>,
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
        self.nodes.push(Node::new(weight));
        self.nodes.len() - 1
    }

    fn add_edge(&mut self, head: usize, tail: usize, weight: E) -> Option<usize> {
        // prepend the edge
        let _ = self.nodes.get(tail)?;
        let head_node = self.nodes.get_mut(head)?;
        let previous_edge = head_node.edges;
        let edge = Edge::new(tail, previous_edge, weight);
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
struct Node<T> {
    weight: T,
    /// Next outgoing edge
    edges: usize,
}

impl<T> Node<T> {
    fn new(weight: T) -> Self {
        Self {
            weight,
            edges: usize::MAX,
        }
    }
}

#[derive(Debug)]
struct Edge<T> {
    tail: usize,
    next: usize,
    weight: T,
}

impl<T> Edge<T> {
    fn new(tail: usize, next: usize, weight: T) -> Self {
        Self { tail, next, weight }
    }
}

fn get_next(node: &State, cost: usize, map: &Map) -> Vec<(State, usize)> {
    let cl = node.dir.rotate_clockwise();
    let ccl = node.dir.rotate_counter_clockwise();

    [
        (State::new(node.point + node.dir.into(), node.dir), cost + 1),
        (State::new(node.point + cl.into(), cl), cost + 1001),
        (State::new(node.point + ccl.into(), ccl), cost + 1001),
    ]
    .into_iter()
    .filter(|n| map.get(n.0.point))
    .collect::<Vec<_>>()
}

fn build_graph(map: &Map) -> DiGraph<State, usize> {
    let start_node = State::new(map.get_start(), Dir::Right);
    let end_point = map.get_end();
    let mut graph = DiGraph::<State, usize>::new();
    let start_index = graph.add_node(start_node);
    let mut seen_edges = HashSet::new();
    let mut seen_nodes = HashMap::from([(start_node, start_index)]);
    let mut queue = vec![start_index];

    while let Some(origin_index) = queue.pop() {
        let cur = *graph.get_node_weight(origin_index).unwrap();

        if cur.point == end_point {
            continue;
        }

        let mut next_nodes = get_next(&cur, 0, map);

        while next_nodes.len() == 1 {
            let (cur, cost) = next_nodes[0];
            if cur.point == end_point {
                next_nodes = vec![(cur, cost)];
                break;
            }
            next_nodes = get_next(&cur, cost, map);
        }

        if !next_nodes.is_empty() {
            for (node, cost) in next_nodes {
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
struct State {
    point: Point,
    dir: Dir,
}

impl State {
    fn new(point: Point, dir: Dir) -> Self {
        Self { point, dir }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct WeightedState {
    node_index: usize,
    cost: usize,
}

impl WeightedState {
    fn new(node_index: usize, cost: usize) -> Self {
        Self { node_index, cost }
    }
}

impl Ord for WeightedState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for WeightedState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra_graph(graph: &DiGraph<State, usize>, end: Point) -> (usize, usize, Vec<Vec<usize>>) {
    let mut queue = BinaryHeap::from([WeightedState::new(0, 0)]);
    let mut min = usize::MAX;
    let mut costs = vec![usize::MAX; graph.nodes.len()];
    let mut predecessor_lookup = vec![Vec::new(); graph.nodes.len()];
    let mut end_index = usize::MAX;

    while let Some(WeightedState { node_index, cost }) = queue.pop() {
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
                queue.push(WeightedState::new(neighbor, next_cost));
            } else if costs[neighbor] == next_cost {
                predecessor_lookup[neighbor].push(node_index);
                queue.push(WeightedState::new(neighbor, next_cost));
            }
        }
    }

    (min, end_index, predecessor_lookup)
}

fn count_nodes_graph(
    graph: &DiGraph<State, usize>,
    end_index: usize,
    predecessor_lookup: Vec<Vec<usize>>,
) -> usize {
    let mut number_of_positions = 0;
    let mut seen_edges = vec![false; graph.edges.len()];
    let mut seen_nodes = vec![false; graph.nodes.len()];

    let mut queue = vec![end_index];

    while let Some(node_index) = queue.pop() {
        if seen_nodes[node_index] {
            number_of_positions -= 1;
        } else {
            seen_nodes[node_index] = true;

            let predecessors = predecessor_lookup[node_index]
                .iter()
                .collect::<HashSet<_>>();

            for predecessor in &predecessors {
                let edge = graph.get_edge(**predecessor, node_index).unwrap();
                if !seen_edges[edge] {
                    seen_edges[edge] = true;
                    number_of_positions += graph.get_edge_weight(edge).unwrap() % 1000;
                    queue.push(**predecessor);
                }
            }

            if predecessors.len() > 1 {
                number_of_positions -= (predecessors.len() - 1) * 2;
            }
        }
    }

    number_of_positions + 1
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
    let map = parse(INPUT);
    let graph = build_graph(&map);
    let (min, _, _) = dijkstra_graph(&graph, map.get_end());
    min
}

pub fn solve_2() -> usize {
    let map = parse(INPUT);
    let graph = build_graph(&map);
    let (_, end_index, lookup) = dijkstra_graph(&graph, map.get_end());
    count_nodes_graph(&graph, end_index, lookup)
}
