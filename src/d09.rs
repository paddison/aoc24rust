#[allow(dead_code)]
static TEST: &str = include_str!("../data/d09_test");
static INPUT: &str = include_str!("../data/d09");

#[derive(Clone, Copy, Debug)]
struct Block {
    start: usize,
    len: usize,
    id: usize,
}

fn parse_input(input: &str) -> Vec<Option<u16>> {
    let mut fs = Vec::new();

    for (id, c) in input[0..input.len() - 1].chars().enumerate() {
        let size = c.to_digit(10).expect("Input has non numerical characters");
        let n = if id % 2 == 0 {
            Some(id as u16 / 2)
        } else {
            None
        };

        for _ in 0..size {
            fs.push(n);
        }
    }

    fs
}

fn parse_input_2(input: &str) -> Vec<Block> {
    let mut start = 0;
    let mut blocks = Vec::new();

    for (id, c) in input[0..input.len() - 1].chars().enumerate() {
        if let Some(len) = c.to_digit(10) {
            if id % 2 == 0 {
                blocks.push(Block {
                    start,
                    len: len as usize,
                    id: id / 2,
                });
            }
            start += len as usize;
        }
    }

    blocks
}

fn compact(fs: &mut [Option<u16>]) {
    let mut i = 0;
    let mut j = fs.len() - 1;

    while i < j {
        while fs[i].is_some() {
            i += 1;
        }

        while fs[j].is_none() {
            j -= 1;
        }

        assert!(i < fs.len());
        assert!(j > 0);

        if i < j {
            fs.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}

fn compact_2(fs: &mut Vec<Block>) {
    assert!(!fs.is_empty());

    for id in (1..=fs.last().unwrap().id).rev() {
        let pos = fs.iter().rposition(|b| b.id == id).unwrap();
        let mut current = fs[pos];

        for k in 0..pos {
            let left = fs[k];
            let right = fs[k + 1];

            if right.start - (left.start + left.len) >= current.len {
                current.start = left.start + left.len;
                fs.remove(pos);
                fs.insert(k + 1, current);
                break;
            }
        }
    }
}

fn checksum(fs: Vec<Option<u16>>) -> usize {
    fs.into_iter()
        .take_while(|n| n.is_some())
        .enumerate()
        .map(|(i, n)| i * n.unwrap() as usize)
        .sum()
}

fn checksum_2(fs: Vec<Block>) -> usize {
    fs.into_iter().map(sum).sum()
}

fn sum(block: Block) -> usize {
    let a = block.start;
    let b = block.start + block.len - 1;

    ((b * (b + 1) - a * (a - 1)) / 2) * block.id
}

pub fn solve_1() -> usize {
    let mut fs = parse_input(INPUT);
    compact(&mut fs);
    checksum(fs)
}

pub fn solve_2() -> usize {
    let mut fs = parse_input_2(INPUT);
    compact_2(&mut fs);
    checksum_2(fs)
}
