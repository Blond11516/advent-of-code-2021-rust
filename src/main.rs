use day::Day;
use std::env::args;

mod day;

#[derive(Debug)]
struct Puzzle {
    day: i32,
    part: i32,
}

fn main() {
    let day = args().nth(1).expect("Bad input format");
    let part = args().nth(2).expect("Bad input format");

    let puzzle = Puzzle {
        day: day.parse().unwrap(),
        part: part.parse().unwrap(),
    };

    let input = get_puzzle_input(puzzle.day);

    let result = match puzzle.day {
        1 => day::Day1 {}.solve(puzzle.part, input),
        _ => panic!("Unknown day"),
    };

    println!("Result: {}", result)
}

fn get_puzzle_input(day: i32) -> String {
    return std::fs::read_to_string(format!("input/day{}.txt", day))
        .expect(format!("No input for day {}", day).as_str());
}
