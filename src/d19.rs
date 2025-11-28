use std::collections::HashSet;

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

fn can_create_pattern_recurse(
    towels: &HashSet<&'static str>,
    pattern: &'static str,
    max_number_of_stripes: usize,
    seen: &mut HashSet<&'static str>,
) -> bool {
    if pattern.is_empty() {
        return true;
    }

    let min = max_number_of_stripes.min(pattern.len());

    for substring_len in 1..min + 1 {
        if towels.contains(&pattern[..substring_len]) {
            let next_pattern = &pattern[substring_len..];

            if seen.insert(next_pattern)
                && can_create_pattern_recurse(
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

pub fn solve_1() -> usize {
    let (towels, patterns, max_number_of_stripes) = parse(INPUT);

    patterns
        .iter()
        .filter(|pattern| {
            can_create_pattern_recurse(&towels, pattern, max_number_of_stripes, &mut HashSet::new())
        })
        .count()
}

#[test]
fn test() {
    println!("{}", solve_1());
}
