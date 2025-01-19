use std::collections::HashMap;

static TEST: &str = include_str!("../data/d01");

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let nums: Vec<usize> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        assert_eq!(nums.len(), 2);

        left.push(nums[0]);
        right.push(nums[1]);
    }

    (left, right)
}

fn get_diff(mut left: Vec<usize>, mut right: Vec<usize>) -> usize {
    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

fn count_occurrences(left: &[usize], right: &[usize]) -> usize {
    let mut occurrences: HashMap<usize, usize> = HashMap::new();

    for n in left {
        if occurrences.contains_key(n) {
            continue;
        }
        let counts = right.iter().filter(|other| *other == n).count();

        occurrences.insert(*n, counts);
    }

    occurrences.into_iter().map(|(a, b)| a * b).sum()
}

pub fn get_solution_1() -> usize {
    let (left, right) = parse_input(TEST);
    get_diff(left, right)
}

pub fn get_solution_2() -> usize {
    let (left, right) = parse_input(TEST);
    count_occurrences(&left, &right)
}
