use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
    ops::{Add, AddAssign},
};

static TEST: &str = include_str!("../data/d20_test");
static INPUT: &str = include_str!("../data/d20");

const WIDTH_TEST: usize = 15;
const WIDTH: usize = 141;

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

#[derive(Clone, Copy, Default, Debug)]
struct Node<N> {
    weight: N,
    incoming: Option<usize>,
    outgoing: Option<usize>,
}

impl<N> Node<N> {
    fn new(weight: N) -> Self {
        Self {
            weight,
            incoming: None,
            outgoing: None,
        }
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct Edge<E> {
    weight: E,
    from: usize,
    to: usize,
    next_outgoing: Option<usize>,
    next_incoming: Option<usize>,
}

impl<E> Edge<E> {
    fn new(from: usize, to: usize, weight: E) -> Self {
        Self {
            weight,
            from,
            to,
            next_outgoing: None,
            next_incoming: None,
        }
    }
}

#[derive(Debug)]
struct Graph<N, E, const NN: usize, const NE: usize> {
    nodes: [Node<N>; NN],
    edges: [Edge<E>; NE],
    node_count: usize,
    edge_count: usize,
}

impl<N: Default + Copy, E: Default + Copy, const NN: usize, const NE: usize> Graph<N, E, NN, NE> {
    fn default() -> Self {
        Self {
            nodes: [Default::default(); NN],
            edges: [Default::default(); NE],
            node_count: 0,
            edge_count: 0,
        }
    }
}

impl<N: Default, E: Default, const NN: usize, const NE: usize> Graph<N, E, NN, NE> {
    fn new() -> Self {
        Self {
            nodes: core::array::from_fn(|_| Default::default()),
            edges: core::array::from_fn(|_| Default::default()),
            node_count: 0,
            edge_count: 0,
        }
    }
}

impl<N, E, const NN: usize, const NE: usize> Graph<N, E, NN, NE> {
    fn add_node(&mut self, node: N) -> Option<usize> {
        if self.node_count == NN {
            None
        } else {
            self.nodes[self.node_count] = Node::new(node);
            self.node_count += 1;
            Some(self.node_count - 1)
        }
    }

    fn add_edge_undirected(&mut self, from: usize, to: usize, weight: E) -> Option<usize> {
        if self.edge_count == NE || from >= self.node_count || to >= self.node_count {
            None
        } else {
            let edge_index = self.edge_count;
            let mut edge = Edge::new(from, to, weight);

            let from_ref = &mut self.nodes[from];
            edge.next_outgoing = from_ref.outgoing;
            from_ref.outgoing = Some(edge_index);

            let to_ref = &mut self.nodes[to];
            edge.next_incoming = to_ref.incoming;
            to_ref.incoming = Some(edge_index);

            self.edges[edge_index] = edge;

            self.edge_count += 1;

            Some(edge_index)
        }
    }

    fn get_neighbors(&self, node: usize) -> Neighbors<'_, N, E, NN, NE> {
        if node >= self.node_count {
            Neighbors::new_empty(self)
        } else {
            Neighbors::new(node, self)
        }
    }

    fn get_node(&self, node_index: usize) -> &N {
        &self.nodes[node_index].weight
    }
}

impl<N: Eq + PartialEq, E, const NN: usize, const NE: usize> Graph<N, E, NN, NE> {
    fn find_node_by_weight(&self, weight: N) -> Option<usize> {
        for (i, n) in self.nodes.iter().enumerate() {
            if n.weight == weight {
                return Some(i);
            }
        }

        None
    }
}

struct Neighbors<'a, N, E, const NN: usize, const NE: usize> {
    graph: &'a Graph<N, E, NN, NE>,
    edge: Option<usize>,
}

impl<'a, N, E, const NN: usize, const NE: usize> Neighbors<'a, N, E, NN, NE> {
    fn new(node: usize, graph: &'a Graph<N, E, NN, NE>) -> Self {
        let edge = graph.nodes[node].outgoing;
        Self { graph, edge }
    }

    fn new_empty(graph: &'a Graph<N, E, NN, NE>) -> Self {
        Self { graph, edge: None }
    }
}

impl<'a, N, E, const NN: usize, const NE: usize> Iterator for Neighbors<'a, N, E, NN, NE> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.edge {
            Some(edge_index) => {
                let edge = &self.graph.edges[edge_index];
                let next = edge.to;
                self.edge = edge.next_outgoing;

                Some(next)
            }
            none => none,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum Tile {
    Start,
    End,
    Floor,
    #[default]
    Wall,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Start,
            'E' => Self::End,
            '.' => Self::Floor,
            _ => Self::Wall,
        }
    }
}

fn parse<const NN: usize, const NE: usize>(input: &str) -> Graph<Tile, (), NN, NE> {
    let mut graph = Graph::default();
    let width = input.find('\n').unwrap_or(0);
    let height = input.len() / (width + 1);

    input
        .lines()
        .flat_map(|line| line.chars())
        .map(|tile| tile.into())
        .for_each(|tile| {
            graph.add_node(tile);
        });

    for y in 0..height {
        for x in 0..width {
            let point = Point::new(x, y);
            let point_index = width * y + x;

            for dir in DIRS {
                let neighbor = point + dir;
                if neighbor.x < width && neighbor.y < height {
                    let neighbor_index = width * neighbor.y + neighbor.x;
                    assert!(neighbor_index < graph.node_count);

                    graph.add_edge_undirected(point_index, neighbor_index, ());
                }
            }
        }
    }

    graph
}

struct State {
    node_index: usize,
    cost: usize,
}

fn dfs<const NN: usize, const NE: usize>(graph: &Graph<Tile, (), NN, NE>) -> usize {
    let start = graph.find_node_by_weight(Tile::Start);

    if start.is_none() {
        return 0;
    }

    let mut queue = vec![(start.unwrap(), 0)];
    let mut seen = vec![false; NN * NN];

    while let Some((node_index, cost)) = queue.pop() {
        if seen[node_index] {
            continue;
        } else {
            seen[node_index] = true;
        }

        for neighbor in graph.get_neighbors(node_index) {
            match graph.get_node(neighbor) {
                Tile::Start | Tile::Wall => continue,
                Tile::End => return cost + 1,
                Tile::Floor => queue.push((neighbor, cost + 1)),
            }
        }
    }

    0
}

fn dfs_cheat<const NN: usize, const NE: usize>(
    max: usize,
    when: usize,
    graph: &Graph<Tile, (), NN, NE>,
) -> Vec<usize> {
    let start = graph.find_node_by_weight(Tile::Start);

    if start.is_none() {
        return Vec::new();
    }

    let mut queue = VecDeque::from([(start.unwrap(), 0, 0)]);
    let mut costs = Vec::new();

    while let Some((node_index, cost, prev)) = queue.pop_front() {
        if cost >= max {
            break;
        }
        for neighbor in graph.get_neighbors(node_index) {
            match graph.get_node(neighbor) {
                Tile::Wall if when == cost => queue.push_back((neighbor, cost + 1, node_index)),
                Tile::Start | Tile::Wall => continue,
                Tile::End => costs.push(cost + 1), //return cost + 1,
                Tile::Floor if when == cost => continue,
                Tile::Floor if neighbor != prev => {
                    queue.push_back((neighbor, cost + 1, node_index))
                }
                _ => continue,
            }
        }
    }

    costs
}

pub fn solve_1() -> usize {
    const NUMBER_OF_NODES: usize = WIDTH * WIDTH;
    const NUMBER_OF_EDGES: usize = NUMBER_OF_NODES * 4;
    let g = parse::<NUMBER_OF_NODES, NUMBER_OF_EDGES>(INPUT);
    let maximum_steps = dfs::<NUMBER_OF_NODES, NUMBER_OF_EDGES>(&g);
    (0..maximum_steps - 100)
        .flat_map(|i| dfs_cheat(maximum_steps - 100, i, &g))
        .count()
}

#[test]
fn test() {
    println!("{}", solve_1());
}
