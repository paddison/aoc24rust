use std::iter::Peekable;

static DATA: &str = include_str!("../data/d03");
static TEST: &str = include_str!("../data/d03_test");

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

#[derive(Debug, Clone, Copy)]
enum State {
    Start,
    Mul,
    Do,
    Dont,
    Num(Option<u32>),
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let mut data = Vec::new();

    for line in input.lines() {
        let mut memory_iter = line.chars().peekable();
        while let Some(nums) = find_multiply_instruction(&mut memory_iter) {
            data.push(nums);
        }
    }

    data
}

fn consume<I: Iterator<Item = char>>(iter: &mut Peekable<I>, other: char) -> bool {
    match iter.peek() {
        Some(c) if *c == other => {
            _ = iter.next();
            true
        }
        _ => false,
    }
}

fn parse_num<I: Iterator<Item = char>>(iter: &mut I) -> Option<(char, u32)> {
    let mut n = 0;
    for c in iter {
        if let Some(d) = c.to_digit(10) {
            n = n * 10 + d;
        } else {
            return Some((c, n));
        }
    }

    None
}

fn is_call<I: Iterator<Item = char>>(iter: &mut Peekable<I>) -> bool {
    consume(iter, '(') && consume(iter, ')')
}

fn find_multiply_instruction<I: Iterator<Item = char>>(
    memory_iter: &mut Peekable<I>,
) -> Option<Instruction> {
    let mut state = State::Start;

    while let Some(c) = memory_iter.peek() {
        match (c, state) {
            (c, State::Start) => match c {
                'm' => state = State::Mul,
                'd' => state = State::Do,
                _ => {
                    memory_iter.next();
                    continue;
                }
            },
            (c, State::Mul) => {
                assert_eq!(*c, 'm');
                _ = memory_iter.next(); // consume the 'm'
                if !(consume(memory_iter, 'u')
                    && consume(memory_iter, 'l')
                    && consume(memory_iter, '('))
                {
                    state = State::Start;
                    continue;
                }
                state = State::Num(None);
            }
            (c, State::Do) => {
                assert_eq!(*c, 'd');
                _ = memory_iter.next(); // consume the 'd' (nice!)
                if !consume(memory_iter, 'o') {
                    state = State::Start;
                    continue;
                }
                if let Some(&'n') = memory_iter.peek() {
                    state = State::Dont;
                    continue;
                }
                if !is_call(memory_iter) {
                    state = State::Start;
                    continue;
                } else {
                    return Some(Instruction::Do);
                }
            }
            (c, State::Dont) => {
                assert_eq!(*c, 'n');
                _ = memory_iter.next(); // Consume the 'n'
                if !(consume(memory_iter, '\'')
                    && consume(memory_iter, 't')
                    && is_call(memory_iter))
                {
                    state = State::Start;
                    continue;
                } else {
                    return Some(Instruction::Dont);
                }
            }
            (c, State::Num(None)) if c.is_numeric() => match parse_num(memory_iter) {
                Some((',', lhs)) => state = State::Num(Some(lhs)),
                _ => state = State::Start,
            },
            (c, State::Num(Some(lhs))) if c.is_numeric() => match parse_num(memory_iter) {
                Some((')', rhs)) => return Some(Instruction::Mul(lhs, rhs)),
                _ => state = State::Start,
            },
            _ => {
                unreachable!()
            }
        }
    }
    None
}

pub fn get_solution_1() -> u32 {
    parse_input(DATA)
        .iter()
        .map(|instruction| match instruction {
            Instruction::Mul(lhs, rhs) => lhs * rhs,
            _ => 0,
        })
        .sum()
}

pub fn get_solution_2() -> u32 {
    let mut do_mul = true;
    let mut sum = 0;

    for instruction in parse_input(DATA) {
        match instruction {
            Instruction::Mul(lhs, rhs) if do_mul => sum += lhs * rhs,
            Instruction::Do => do_mul = true,
            Instruction::Dont => do_mul = false,
            _ => (),
        }
    }

    sum
}
