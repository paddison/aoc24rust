use core::ops::Range;

static INPUT: &str = include_str!("../data/d02");

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn is_safe(report: &[isize]) -> bool {
    if report.len() == 1 {
        true
    } else if report[0] < report[1] {
        is_safe_increasing(report, -3..0)
    } else {
        is_safe_increasing(report, 1..4)
    }
}

fn is_safe_increasing(report: &[isize], range: Range<isize>) -> bool {
    for (a, b) in report.iter().zip(&report[1..]) {
        if !range.contains(&(a - b)) {
            return false;
        }
    }

    true
}

fn is_safe_variants(report: &[isize]) -> bool {
    for i in 0..report.len() {
        let variant: Vec<isize> = report
            .iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .map(|(_, n)| *n)
            .collect();

        if is_safe(&variant) {
            return true;
        }
    }

    false
}

pub fn get_solution_1() -> usize {
    parse_input(INPUT)
        .into_iter()
        .filter(|report| is_safe(report))
        .count()
}

pub fn get_solution_2() -> usize {
    parse_input(INPUT)
        .into_iter()
        .filter(|report| is_safe_variants(report))
        .count()
}
