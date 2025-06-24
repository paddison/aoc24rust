#[allow(dead_code)]
static TEST: &str = include_str!("../data/d04_test");
static DATA: &str = include_str!("../data/d04");

trait Vec2D {
    fn get_at(&self, row: usize, col: usize) -> Option<char>;
}

impl Vec2D for [Vec<char>] {
    fn get_at(&self, row: usize, col: usize) -> Option<char> {
        self.get(row).and_then(|r| r.get(col)).copied()
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn count_xmas(text: Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for (row, line) in text.iter().enumerate() {
        for (col, letter) in line.iter().enumerate() {
            if *letter == 'X' {
                count += find_xmas(&text, row, col);
            }
        }
    }

    count
}

fn find_xmas(text: &[Vec<char>], row: usize, col: usize) -> usize {
    assert_eq!(text.get_at(row, col), Some('X'));
    // up:
    let up: String = (0..4)
        .filter_map(|i| text.get_at(row, col.wrapping_sub(i)))
        .collect();
    let right: String = (0..4).filter_map(|i| text.get_at(row + i, col)).collect();
    let down: String = (0..4).filter_map(|i| text.get_at(row, col + i)).collect();
    let left: String = (0..4)
        .filter_map(|i| text.get_at(row.wrapping_sub(i), col))
        .collect();
    let up_right = (0..4)
        .filter_map(|i| text.get_at(row + i, col.wrapping_sub(i)))
        .collect();
    let down_right: String = (0..4)
        .filter_map(|i| text.get_at(row + i, col + i))
        .collect();
    let down_left: String = (0..4)
        .filter_map(|i| text.get_at(row.wrapping_sub(i), col + i))
        .collect();
    let up_left: String = (0..4)
        .filter_map(|i| text.get_at(row.wrapping_sub(i), col.wrapping_sub(i)))
        .collect();

    [
        up, right, down, left, up_right, down_right, down_left, up_left,
    ]
    .into_iter()
    .filter(|s| s.as_str() == "XMAS")
    .count()
}

pub fn get_solution_1() -> usize {
    count_xmas(parse_input(DATA))
}
