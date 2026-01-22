pub trait Solver<T, U> {
    fn new(input: &str) -> Self;
    fn part1(&self) -> T;
    fn part2(&self) -> U;
}

pub fn nsew(x: usize, y: usize) -> [(usize, usize); 4] {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .map(|(dx, dy)| (x.saturating_add_signed(dx), y.saturating_add_signed(dy)))
}
