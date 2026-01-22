use advent_of_code_2018::Solver;

const PUZZLE: &str = include_str!("../../puzzles/dayXX.txt");

fn main() {
    let solver = Puzzle::new(PUZZLE);
    println!("Part 1: {}", solver.part1());
    //println!("Part 2: {}", solver.part2());
}

struct Puzzle {}

impl Solver<usize, usize> for Puzzle {
    fn new(input: &str) -> Self {
        Self {}
    }

    fn part1(&self) -> usize {
        todo!()
    }

    fn part2(&self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod puzzle_name {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/dayXX.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).part1(), todo!())
    }

    #[test]
    fn test2() {}
}
