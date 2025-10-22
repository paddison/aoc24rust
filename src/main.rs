use std::time::Instant;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;

fn main() {
    aoc_result!(1, 1, d01::get_solution_1());
    aoc_result!(1, 2, d01::get_solution_2());
    aoc_result!(2, 1, d02::get_solution_1());
    aoc_result!(2, 2, d02::get_solution_2());
    aoc_result!(3, 1, d03::get_solution_1());
    aoc_result!(3, 2, d03::get_solution_2());
    aoc_result!(4, 1, d04::get_solution_1());
    aoc_result!(4, 2, d04::get_solution_2());
    aoc_result!(5, 1, d05::get_solution_1());
    aoc_result!(5, 2, d05::get_solution_2());
    aoc_result!(6, 1, d06::solve_1());
    aoc_result!(6, 2, d06::solve_2());
    aoc_result!(7, 1, d07::solve_1());
    aoc_result!(7, 2, d07::solve_2());
    aoc_result!(8, 1, d08::solve_1());
    aoc_result!(8, 2, d08::solve_2());
    aoc_result!(9, 1, d09::solve_1());
    aoc_result!(9, 2, d09::solve_2());
    aoc_result!(10, 1, d10::solve_1());
    aoc_result!(10, 2, d10::solve_2());
    aoc_result!(11, 1, d11::solve_1());
    aoc_result!(11, 2, d11::solve_2());
    aoc_result!(12, 1, d12::solve_1());
    aoc_result!(12, 2, d12::solve_2());
    aoc_result!(13, 1, d13::solve_1());
    aoc_result!(13, 2, d13::solve_2());
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
