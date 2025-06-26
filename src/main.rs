use std::time::Instant;

mod d01;
mod d02;
mod d03;
mod d04;

fn main() {
    aoc_result!(1, 1, d01::get_solution_1());
    aoc_result!(1, 2, d01::get_solution_2());
    aoc_result!(2, 1, d02::get_solution_1());
    aoc_result!(2, 2, d02::get_solution_2());
    aoc_result!(3, 1, d03::get_solution_1());
    aoc_result!(3, 2, d03::get_solution_2());
    aoc_result!(4, 1, d04::get_solution_1());
    aoc_result!(4, 2, d04::get_solution_2());
}

#[macro_export]
macro_rules! aoc_result {
    ( $d:literal, $p:literal, $r:expr ) => {
        let now = Instant::now();
        println!(
            "d{:2}.{}: {:16}\t{:10}us",
            $d,
            $p,
            $r,
            now.elapsed().as_micros()
        );
    };
}
