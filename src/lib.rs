pub trait Solver<T, U> {
    fn new(input: &str) -> Self;
    fn part1(&self) -> T;
    fn part2(&self) -> U;
}

pub fn nsew() -> [isize; 4] {}
