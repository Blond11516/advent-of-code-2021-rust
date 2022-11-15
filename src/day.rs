use itertools::Itertools;

pub trait Day {
    fn solve(&self, part: i32, input: String) -> usize;
}

pub struct Day1();

impl Day for Day1 {
    fn solve(&self, part: i32, input: String) -> usize {
        match part {
            1 => solve_part_1(input),
            2 => solve_part_2(input),
            _ => panic!("Unknown part {}", part),
        }
    }
}

fn solve_part_1(input: String) -> usize {
    input
        .lines()
        .tuple_windows()
        .map(|(prev, cur)| (prev.parse::<i32>().unwrap(), cur.parse().unwrap()))
        .map(|(prev, cur)| prev < cur)
        .filter(|has_increased| *has_increased)
        .count()
}

fn solve_part_2(input: String) -> usize {
    input
        .lines()
        .tuple_windows()
        .map(|(prev1, prev2, cur)| {
            (
                prev1.parse::<i32>().unwrap(),
                prev2.parse::<i32>().unwrap(),
                cur.parse::<i32>().unwrap(),
            )
        })
        .map(|(prev1, prev2, cur)| prev1 + prev2 + cur)
        .tuple_windows()
        .map(|(prev, cur)| prev < cur)
        .filter(|has_increased| *has_increased)
        .count()
}
