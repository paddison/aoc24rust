use std::collections::HashMap;

static TEST: &str = include_str!("../data/d05_test");

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
            graph.entry(from).or_insert(vec![to]).push(to);
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

pub fn get_solution_1() -> usize {
    let data = parse(TEST);
    println!("{data:?}");
    0
}
