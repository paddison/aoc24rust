use std::{collections::HashMap, fmt::Display};

#[allow(dead_code)]
static TEST: &str = include_str!("../data/d11_test");
static INPUT: &str = include_str!("../data/d11");

#[derive(Debug)]
struct SplitNode<T> {
    value: T,
    next: usize,
}

impl<T> SplitNode<T> {
    fn new(value: T, next: usize) -> Self {
        Self { value, next }
    }
}

#[derive(Debug)]
struct SplitVec<T> {
    inner: Vec<SplitNode<T>>,
    back: usize,
}

impl<T> SplitVec<T> {
    fn new() -> Self {
        Self {
            inner: Vec::new(),
            back: 0,
        }
    }
    // Index points to the actual item in the Vec
    fn split(&mut self, index: usize, value_left: T, value_right: T) -> usize {
        if index >= self.len() {
            return 0;
        }

        let next = self.inner[index].next;
        if next == 0 {
            self.back = self.inner.len();
        }

        self.inner[index].next = self.len();
        self.inner[index].value = value_left;
        self.inner.push(SplitNode::new(value_right, next));

        next
    }

    fn push(&mut self, value: T) {
        let next = self.len();
        if let Some(last) = self.inner.get_mut(self.back) {
            last.next = next;
        }
        self.back = next;
        self.inner.push(SplitNode::new(value, 0));
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn get(&self, index: usize) -> Option<&SplitNode<T>> {
        self.inner.get(index)
    }
}

impl Display for SplitVec<usize> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        let mut current = 0;
        loop {
            if let Some(node) = self.get(current) {
                buffer.push_str(&node.value.to_string());
                buffer.push(' ');
                if node.next == 0 {
                    break write!(f, "{buffer}");
                }
                current = node.next;
            } else {
                break write!(f, "{buffer}");
            }
        }
    }
}

fn loop_over(line: &mut SplitVec<usize>) {
    if line.len() == 0 {
        return;
    }
    let mut current = 0;

    loop {
        let SplitNode { value, mut next } = *line.get(current).unwrap();
        if value == 0 {
            line.inner.get_mut(current).unwrap().value = 1;
        } else {
            match split_number(value) {
                Some(ten) => next = line.split(current, value / ten, value % ten),
                None => line.inner.get_mut(current).unwrap().value *= 2024,
            }
        }

        if next == 0 {
            break;
        }

        current = next;
    }
}

fn split_number(mut n: usize) -> Option<usize> {
    // Check how many digits a number has and return 10^(number_of_digits / 2) if it is an even number
    let mut m = 0;
    let mut ten = 1;

    while n > 0 {
        n /= 10;
        m += 1;
        if m % 2 == 0 {
            ten *= 10;
        }
    }

    if m % 2 == 0 {
        Some(ten)
    } else {
        None
    }
}

fn parse_input(input: &str) -> SplitVec<usize> {
    let mut list = SplitVec::new();

    input
        .split_whitespace()
        .filter_map(|n| n.parse::<usize>().ok())
        .for_each(|n| list.push(n));

    list
}

pub fn solve_1() -> usize {
    let mut list = parse_input(INPUT);
    for _ in 0..25 {
        loop_over(&mut list);
    }
    list.len()
}

pub fn solve_2() -> usize {
    // determine, for each number, the resulting numbers at each step, before they become single digits again
    // store data like so:
    // store one map: number and an id
    // store one vec which for each id contains the current count. and an additional vec for
    // on each iteration, create a new vec with the new counts.
    // at the end, sum up all counts
    let mut id_to_number: Vec<usize> = INPUT
        .split_whitespace()
        .filter_map(|n| n.parse::<usize>().ok())
        .collect();
    let mut number_to_id: HashMap<usize, usize> = id_to_number
        .iter()
        .enumerate()
        .map(|(id, number)| (*number, id))
        .collect();
    let mut id_to_count = vec![1; id_to_number.len()];
    let mut id_count = id_to_number.len();

    // Update the maps on an iteration:
    for _ in 0..75 {
        let mut updated_id_to_count = id_to_count.clone();
        assert_eq!(id_to_count.len(), id_to_number.len());
        let max_id = id_count;

        for id in 0..max_id {
            let count = id_to_count[id];
            let number = id_to_number[id];
            // Calculate how the number transforms in the next loop
            let next_numbers = if number == 0 {
                [Some(1), None]
            } else {
                match split_number(number) {
                    None => [Some(number * 2024), None],
                    Some(split) => [Some(number / split), Some(number % split)],
                }
            };

            for next_number in next_numbers.iter().filter_map(|n| *n) {
                // Get the id of the next number
                let next_id = match number_to_id.get(&next_number) {
                    Some(next_id) => *next_id,
                    None => {
                        // If the number was not known before, insert it
                        number_to_id.insert(next_number, id_count);
                        id_count += 1;
                        updated_id_to_count.push(0);
                        id_to_number.push(next_number);
                        id_count - 1
                    }
                };

                updated_id_to_count[next_id] += count;
            }

            // An iteration "removes" the number from the list.
            updated_id_to_count[id] -= count;
        }

        // Swap out the old counts with the new one
        id_to_count = updated_id_to_count;
    }

    id_to_count.into_iter().sum()
}
