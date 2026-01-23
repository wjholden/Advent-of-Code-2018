use std::{collections::HashMap, fmt::Display};

use advent_of_code_2018::Solver;
use num_complex::Complex;

const PUZZLE: &str = include_str!("../../puzzles/day13.txt");

/// Man, I really thought this one was going to take some crazy modulo
/// arithmetic. Turns out you have to read the instructions carefully.
/// No tricks, just a tricky procedural puzzle.
fn main() {
    let first_collisions = Puzzle::new(PUZZLE).part1();
    println!(
        "Part 1: {},{}",
        first_collisions[0].re, first_collisions[0].im
    );
    let mut puzzle = Puzzle::new(PUZZLE);
    let last_cart = puzzle.part2();
    println!("Part 2: {},{}", last_cart.re, last_cart.im);
    println!("{puzzle}");
}

enum Track {
    Vertical,
    Horizontal,
    TurnSW,
    TurnSE,
    Intersection,
}

impl Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Track::Vertical => "|",
                Track::Horizontal => "-",
                Track::TurnSW => "\\",
                Track::TurnSE => "/",
                Track::Intersection => "+",
            }
        )?;
        Ok(())
    }
}

#[derive(Debug)]
enum Decision {
    Left,
    Straight,
    Right,
}

#[derive(Debug)]
struct Cart {
    position: Complex<isize>,
    velocity: Complex<isize>,
    next_decision: Decision,
}

impl Cart {
    fn new(position: Complex<isize>, velocity: Complex<isize>) -> Self {
        Self {
            position,
            velocity,
            next_decision: Decision::Left,
        }
    }

    fn tick(&mut self, tracks: &HashMap<Complex<isize>, Track>) {
        self.position += self.velocity;
        match tracks.get(&self.position) {
            None => {
                let msg = format!(
                    "not on the rails: ({},{})",
                    self.position.re, self.position.im
                );
                panic!("{}", msg)
            }
            Some(Track::Vertical) => {
                assert!(self.velocity.re == 0 && self.velocity.im.abs() == 1)
            }
            Some(Track::Horizontal) => {
                assert!(self.velocity.re.abs() == 1 && self.velocity.im == 0)
            }
            Some(Track::TurnSE) => {
                // Thanks for the tip, @Zefick.
                // https://www.reddit.com/r/adventofcode/comments/1pupbng/comment/nvu6w1f/
                // I think my division trick fails because we are in this world
                // where +y is down.
                self.velocity = Complex::new(-self.velocity.im, -self.velocity.re);
            }
            Some(Track::TurnSW) => {
                self.velocity = Complex::new(self.velocity.im, self.velocity.re);
            }
            Some(Track::Intersection) => {
                match self.next_decision {
                    Decision::Left => self.velocity *= -Complex::i(),
                    Decision::Straight => {}
                    Decision::Right => self.velocity *= Complex::i(),
                };
                self.next_decision = match self.next_decision {
                    Decision::Left => Decision::Straight,
                    Decision::Straight => Decision::Right,
                    Decision::Right => Decision::Left,
                }
            }
        }
    }
}

struct Puzzle {
    carts: Vec<Cart>,
    tracks: HashMap<Complex<isize>, Track>,
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows = self.tracks.keys().map(|p| p.im).max().unwrap();
        let cols = self.tracks.keys().map(|p| p.re).max().unwrap();
        let directions = HashMap::from([
            (Complex::new(-1isize, 0), '<'),
            (Complex::new(1, 0), '>'),
            (Complex::new(0, -1), '^'),
            (Complex::new(0, 1), 'v'),
        ]);
        for row in 0..=rows {
            'column: for col in 0..=cols {
                let p = Complex::new(col, row);
                for cart in self.carts.iter() {
                    if cart.position == p {
                        write!(f, "{}", directions.get(&cart.velocity).unwrap())?;
                        continue 'column;
                    }
                }
                match self.tracks.get(&p) {
                    Some(s) => write!(f, "{s}")?,
                    None => write!(f, " ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Solver<Vec<Complex<isize>>, Complex<isize>> for Puzzle {
    fn new(input: &str) -> Self {
        let mut carts = Vec::new();
        let mut tracks = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.char_indices() {
                let p = Complex::new(x as isize, y as isize);
                match c {
                    '|' => {
                        tracks.insert(p, Track::Vertical);
                    }
                    '-' => {
                        tracks.insert(p, Track::Horizontal);
                    }
                    '/' => {
                        tracks.insert(p, Track::TurnSE);
                    }
                    '\\' => {
                        tracks.insert(p, Track::TurnSW);
                    }
                    '+' => {
                        tracks.insert(p, Track::Intersection);
                    }
                    ' ' => {}
                    'v' => {
                        carts.push(Cart::new(p, Complex::new(0, 1)));
                        tracks.insert(p, Track::Vertical);
                    }
                    '^' => {
                        carts.push(Cart::new(p, Complex::new(0, -1)));
                        tracks.insert(p, Track::Vertical);
                    }
                    '<' => {
                        carts.push(Cart::new(p, Complex::new(-1, 0)));
                        tracks.insert(p, Track::Horizontal);
                    }
                    '>' => {
                        carts.push(Cart::new(p, Complex::new(1, 0)));
                        tracks.insert(p, Track::Horizontal);
                    }
                    _ => panic!(),
                }
            }
        }
        Self { carts, tracks }
    }

    fn part1(&mut self) -> Vec<Complex<isize>> {
        let mut crash_sites = vec![];

        loop {
            // Does order matter? Yes. (Read the instructions!)
            // Thank you https://www.reddit.com/r/adventofcode/comments/a8f32j/comment/ecarp8g/.
            self.carts.sort_by(|c1, c2| {
                let x1 = c1.position.re;
                let x2 = c2.position.re;
                let y1 = c1.position.im;
                let y2 = c2.position.im;
                (y1, x1).cmp(&(y2, x2))
            });

            for i in 0..self.carts.len() {
                // Don't move a crashed cart.
                if crash_sites.contains(&self.carts[i].position) {
                    continue;
                }
                self.carts[i].tick(&self.tracks);
                for j in 0..self.carts.len() {
                    if i != j && self.carts[i].position == self.carts[j].position {
                        crash_sites.push(self.carts[i].position);
                    }
                }
            }

            if !crash_sites.is_empty() {
                return crash_sites;
            }
        }
    }

    fn part2(&mut self) -> Complex<isize> {
        while self.carts.len() > 1 {
            let crash_site = self.part1();
            self.carts
                .retain(|cart| !crash_site.contains(&cart.position));
        }
        assert_eq!(self.carts.len(), 1);
        self.carts[0].position
    }
}

#[cfg(test)]
mod mine_cart_madness {
    use super::*;

    const SAMPLE1: &str = include_str!("../../samples/day13-1.txt");
    const SAMPLE2: &str = include_str!("../../samples/day13-2.txt");
    const SAMPLE3: &str = include_str!("../../samples/day13-extra.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE1).part1(), vec![Complex::new(7, 3)])
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE2).part2(), Complex::new(6, 4))
    }

    #[test]
    /// Really needed this extra test case! Thank you!
    /// https://www.reddit.com/r/adventofcode/comments/a8f32j/comment/ecdqxrx/
    fn extra() {
        assert_eq!(Puzzle::new(SAMPLE3).part1(), vec![Complex::new(0, 1)])
    }
}
