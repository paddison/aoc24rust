#[allow(dead_code)]
static TEST: &str = include_str!("../data/d07_test");
static INPUT: &str = include_str!("../data/d07");

struct Equation {
    result: u64,
    nums: Vec<u64>,
}

fn parse_input(input: &str) -> Vec<Equation> {
    let mut equations = Vec::new();

    for line in input.lines() {
        let colon = line.find(':').expect("No colon found in equation");
        let result = line[..colon]
            .parse()
            .expect("Invalid number for result of equation.");
        let nums = line[colon + 2..]
            .split(" ")
            .map(|n| n.parse())
            .collect::<Result<Vec<u64>, _>>()
            .expect("Invalid number for nums of equation");

        equations.push(Equation { result, nums });
    }

    equations
}

fn is_true(Equation { result, nums }: &Equation, do_concat: bool) -> bool {
    if nums.is_empty() {
        false
    } else {
        is_true_inner(&nums[1..], nums[0], *result, do_concat)
    }
}

fn is_true_inner(nums: &[u64], sum: u64, result: u64, do_concat: bool) -> bool {
    if nums.is_empty() {
        sum == result
    } else {
        is_true_inner(&nums[1..], sum * nums[0], result, do_concat)
            || is_true_inner(&nums[1..], sum + nums[0], result, do_concat)
            || (do_concat && is_true_inner(&nums[1..], concat(sum, nums[0]), result, do_concat))
    }
}

fn concat(a: u64, b: u64) -> u64 {
    let mut zero_padding = 1;

    while b / zero_padding != 0 {
        zero_padding *= 10;
    }

    a * zero_padding + b
}

pub fn solve_1() -> u64 {
    parse_input(INPUT)
        .into_iter()
        .filter(|eq| is_true(eq, false))
        .map(|Equation { result, .. }| result)
        .sum()
}

pub fn solve_2() -> u64 {
    parse_input(INPUT)
        .into_iter()
        .filter(|eq| is_true(eq, true))
        .map(|Equation { result, .. }| result)
        .sum()
}
