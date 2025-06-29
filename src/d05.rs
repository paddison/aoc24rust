use std::collections::HashMap;

static TEST: &str = include_str!("../data/d05_test");
static INPUT: &str = include_str!("../data/d05");

type Data = (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>);

fn get_pair(line: &str) -> Option<(usize, usize)> {
    if let Some(delim) = line.find('|') {
        let from = line[0..delim].parse();
        let to = line[delim + 1..line.len()].parse();
        match (from, to) {
            (Ok(f), Ok(t)) => Some((f, t)),
            _ => None,
        }
    } else {
        None
    }
}

fn parse(input: &str) -> Data {
    let mut graph = HashMap::new();
    let mut lines = input.lines();

    for line in &mut lines {
        if let Some((from, to)) = get_pair(line) {
            graph.entry(from).or_insert(Vec::new()).push(to);
        } else {
            assert_eq!("", line);
            break;
        }
    }

    let lists = lines
        .map(|list| list.split(',').filter_map(|n| n.parse().ok()).collect())
        .collect();

    (graph, lists)
}

fn is_ordered(graph: &HashMap<usize, Vec<usize>>, list: &[usize]) -> bool {
    assert!(!list.is_empty());
    // do a search through the graph.
    // if a page can be found, it is in order.
    // if not, it is not in order
    let mut frontier = vec![list[0]];
    let mut cur = 1;
    let mut ordered = false;

    while let Some(node) = frontier.pop() {
        if let Some(neighbors) = graph.get(&node) {
            if neighbors.contains(&list[cur]) {
                frontier.push(list[cur]);
                cur += 1;
                if cur == list.len() {
                    ordered = true;
                    break;
                }
            } else {
                for neighbor in neighbors {
                    frontier.push(*neighbor);
                }
            }
        }
    }

    ordered
}

pub fn get_solution_1() -> usize {
    let (graph, lists) = parse(INPUT);
    println!("{graph:?}");
    println!("{lists:?}");

    let filtered: Vec<Vec<usize>> = lists
        .iter()
        .filter(|list| is_ordered(&graph, list))
        .cloned()
        .collect();

    assert_ne!(lists.len(), filtered.len());
    //.map(|list| list[list.len() / 2])
    //.sum()
    0
}
