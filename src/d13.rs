#[allow(dead_code)]
static TEST: &str = include_str!("../data/d13_test");

static INPUT: &str = include_str!("../data/d13");

const PART_2_INCREASE: isize = 10000000000000;

struct Button {
    x: isize,
    y: isize,
}

impl From<&str> for Button {
    fn from(value: &str) -> Self {
        let numbers: Vec<_> = value.split_whitespace().skip(2).take(2).collect();
        let x = numbers
            .first()
            .and_then(|x_str| x_str[2..x_str.len() - 1].parse().ok())
            .expect("Button string has unexpected format");
        let y = numbers
            .get(1)
            .and_then(|y_str| y_str[2..].parse().ok())
            .expect("Button string has unexpected format");

        Button { x, y }
    }
}

struct Machine {
    button_a: Button, // a d
    button_b: Button, // b e
    prize_x: isize,
    prize_y: isize,
}

impl Machine {
    fn solve(&self) -> Option<(isize, isize)> {
        self.solve_y().and_then(|y| self.solve_x(y))
    }

    fn solve_y(&self) -> Option<isize> {
        let denominator = self.button_b.y * self.button_a.x - self.button_b.x * self.button_a.y;
        // unsolvable
        if denominator == 0 {
            None
        } else {
            let numerator = self.prize_y * self.button_a.x - self.button_a.y * self.prize_x;
            if numerator % denominator != 0 {
                None
            } else {
                Some(numerator / denominator)
            }
        }
    }

    fn solve_x(&self, y: isize) -> Option<(isize, isize)> {
        let denominator = self.button_a.x;
        if denominator == 0 {
            None
        } else {
            let numerator = self.prize_x - self.button_b.x * y;

            if numerator % denominator != 0 {
                None
            } else {
                Some((numerator / denominator, y))
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    let mut lines = input.lines();
    let mut machines = Vec::new();

    while let Some(button_a_str) = lines.next() {
        let button_a = button_a_str.into();
        let button_b = lines
            .next()
            .expect("Input has invalid format. Expect Button B.")
            .into();
        let mut prize_parts = lines
            .next()
            .expect("Input has invalid format. Expect Prize.")
            .split_whitespace()
            .skip(1)
            .take(2);

        let x = prize_parts
            .next()
            .and_then(|x_str| x_str[2..x_str.len() - 1].parse().ok())
            .expect("Prize string has invald format");
        let y = prize_parts
            .next()
            .and_then(|y_str| y_str[2..].parse().ok())
            .expect("Prize string has invald format");

        machines.push(Machine {
            button_a,
            button_b,
            prize_x: x,
            prize_y: y,
        });

        let _ = lines.next();
    }

    machines
}

pub fn solve_1() -> isize {
    parse_input(INPUT)
        .iter()
        .filter_map(|machine| machine.solve())
        .filter(|(a_presses, b_presses)| *a_presses <= 100 && *b_presses <= 100)
        .map(|(a_presses, b_presses)| a_presses * 3 + b_presses)
        .sum()
}

#[allow(clippy::manual_inspect)]
pub fn solve_2() -> isize {
    parse_input(INPUT)
        .iter_mut()
        .map(|machine| {
            machine.prize_x += PART_2_INCREASE;
            machine.prize_y += PART_2_INCREASE;

            machine
        })
        .filter_map(|machine| machine.solve())
        .map(|(a_presses, b_presses)| a_presses * 3 + b_presses)
        .sum()
}
