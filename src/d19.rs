use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d19_test");
static INPUT: &str = include_str!("../data/d19");

fn parse(input: &str) -> (HashSet<&str>, Vec<&str>, usize) {
    let mut lines = input.lines();
    let mut towels = HashSet::new();
    let mut max_number_of_stripes = 0;

    if let Some(line) = lines.next() {
        for towel in line.split(',').map(|s| s.trim()) {
            towels.insert(towel);

            if max_number_of_stripes < towel.len() {
                max_number_of_stripes = towel.len();
            }
        }
    }

    let _ = lines.next();
    (
        towels,
        lines.map(|s| s.trim()).collect(),
        max_number_of_stripes,
    )
}

fn can_create_pattern(
    towels: &HashSet<&'static str>,
    pattern: &'static str,
    max_number_of_stripes: usize,
    seen: &mut HashSet<&'static str>,
) -> bool {
    if pattern.is_empty() {
        return true;
    }

    for substring_len in 1..max_number_of_stripes.min(pattern.len()) + 1 {
        if towels.contains(&pattern[..substring_len]) {
            let next_pattern = &pattern[substring_len..];

            if seen.insert(next_pattern)
                && can_create_pattern(
                    towels,
                    &pattern[substring_len..],
                    max_number_of_stripes,
                    seen,
                )
            {
                return true;
            }
        }
    }

    false
}

fn count_number_of_possibilities(
    towels: &HashSet<&'static str>,
    pattern: &'static str,
    max_number_of_stripes: usize,
    seen: &mut HashMap<&'static str, usize>,
) -> usize {
    (1..max_number_of_stripes.min(pattern.len()) + 1)
        .filter(|substring_len| towels.contains(&pattern[..*substring_len]))
        .map(|substring_len| &pattern[substring_len..])
        .map(|next_pattern| {
            if next_pattern.is_empty() {
                1
            } else if !seen.contains_key(next_pattern) {
                let number_of_possibilities = count_number_of_possibilities(
                    towels,
                    next_pattern,
                    max_number_of_stripes,
                    seen,
                );
                seen.insert(next_pattern, number_of_possibilities);
                number_of_possibilities
            } else {
                *seen.get_mut(&next_pattern).unwrap()
            }
        })
        .sum()
}

pub fn solve_1() -> usize {
    let (towels, patterns, max_number_of_stripes) = parse(INPUT);

    patterns
        .iter()
        .filter(|pattern| {
            can_create_pattern(&towels, pattern, max_number_of_stripes, &mut HashSet::new())
        })
        .count()
}

pub fn solve_2() -> usize {
    let (towels, patterns, max_number_of_stripes) = parse(INPUT);

    patterns
        .iter()
        .map(|p| {
            count_number_of_possibilities(&towels, p, max_number_of_stripes, &mut HashMap::new())
        })
        .sum()
}
