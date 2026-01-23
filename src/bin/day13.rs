use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use advent_of_code_2018::Solver;
use num_complex::Complex;

const PUZZLE: &str = include_str!("../../puzzles/day13.txt");
// const SPECIAL_CART: Complex<isize> = Complex::new(117, 95);

fn main() {
    let mut solver = Puzzle::new(PUZZLE);
    let first_collisions = solver.part1();
    solver
        .carts
        .retain(|cart| !first_collisions.contains(&cart.position));
    println!(
        "Part 1: {},{}",
        first_collisions[0].re, first_collisions[0].im
    );
    println!("Part 2: {}", solver.part2());
}

enum Track {
    Vertical,
    Horizontal,
    TurnSW,
    TurnSE,
    Intersection,
}

impl ToString for Track {
    fn to_string(&self) -> String {
        String::from(match self {
            Track::Vertical => "|",
            Track::Horizontal => "-",
            Track::TurnSW => "\\",
            Track::TurnSE => "/",
            Track::Intersection => "+",
        })
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
    ticks: usize,
    initial_position: Complex<isize>,
    initial_velocity: Complex<isize>,
    period: Option<usize>,
    // output_file: Option<File>,
    // old_path: HashMap<Complex<isize>, HashSet<(Complex<isize>, usize)>>,
    path: Vec<Complex<isize>>,
}

impl Cart {
    fn new(position: Complex<isize>, velocity: Complex<isize>) -> Self {
        // let output_file = if position == SPECIAL_CART {
        //     let mut of = File::create("special_cart_path.csv").expect("create special cart csv");
        //     writeln!(of, "x,y,ticks,direction").unwrap();
        //     Some(of)
        // } else {
        //     None
        // };
        Self {
            position,
            velocity,
            next_decision: Decision::Left,
            ticks: 0,
            initial_position: position,
            initial_velocity: velocity,
            period: None,
            // output_file,
            // old_path: HashMap::new(),
            path: Vec::new(),
        }
    }

    fn tick(&mut self, tracks: &HashMap<Complex<isize>, Track>) {
        // if self.initial_position == SPECIAL_CART && self.period.is_none() {
        //     println!("{},{}", self.position.re, self.position.im);
        // }

        // if let Some(ref mut of) = self.output_file
        //     && self.period.is_none()
        // {
        //     let dir = match (self.velocity.re, self.velocity.im) {
        //         (0, 1) => "v",
        //         (0, -1) => "^",
        //         (-1, 0) => "<",
        //         (1, 0) => ">",
        //         _ => unreachable!(),
        //     };
        //     writeln!(
        //         of,
        //         "{},{},{},\"{}\"",
        //         self.position.re, self.position.im, self.ticks, dir
        //     )
        //     .unwrap();
        // }

        // if self.period.is_none() {
        //     self.old_path
        //         .entry(self.position)
        //         .or_default()
        //         .insert((self.velocity, self.ticks));
        // }

        if self.period.is_none() {
            self.path.push(self.position);
        }

        self.ticks += 1;
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

        if self.period.is_none()
            && self.position == self.initial_position
            && self.velocity == self.initial_velocity
            && matches!(self.next_decision, Decision::Left)
        {
            self.period = Some(self.ticks);
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
                    Some(s) => write!(f, "{}", s.to_string())?,
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
        let mut positions = HashSet::new();
        let mut period_discovered = false;

        loop {
            positions.clear();
            let mut crash_site: Option<Vec<Complex<isize>>> = None;

            // Does order matter?
            self.carts.sort_by(|c1, c2| {
                let x1 = c1.position.re;
                let x2 = c2.position.re;
                let y1 = c1.position.im;
                let y2 = c2.position.im;
                (y1, x1).cmp(&(y2, x2))
            });

            // for i in 0..self.carts.len() {
            //     let cart = &mut self.carts[i];
            //     cart.tick(&self.tracks);
            //     let cart = &self.carts[i];
            //     for j in i + 1..self.carts.len() {
            //         if cart.position == self.carts[j].position {
            //             if !positions.insert(cart.position) {
            //                 if let Some(ref mut crashes) = crash_site {
            //                     println!("multiple crashes in the same tick!");
            //                     crashes.push(cart.position);
            //                 } else {
            //                     crash_site = Some(vec![cart.position]);
            //                 }
            //             }
            //         }
            //     }
            // }

            for cart in self.carts.iter_mut() {
                cart.tick(&self.tracks);

                if !positions.insert(cart.position) {
                    if let Some(ref mut crashes) = crash_site {
                        println!("multiple crashes in the same tick!");
                        crashes.push(cart.position);
                    } else {
                        crash_site = Some(vec![cart.position]);
                    }
                }
            }

            if let Some(crash_site) = crash_site {
                return crash_site;
            }
            if !period_discovered
                && self.carts.len() == 3
                && self.carts.iter().all(|cart| cart.period.is_some())
            {
                period_discovered = true;
                // dbg!(&self.carts);

                // for cart in &self.carts[0..=2] {
                //     cart.old_path
                //         .iter()
                //         .filter(|(_, velocities)| {
                //             let s: HashSet<Complex<isize>> =
                //                 HashSet::from_iter(velocities.iter().map(|(v, _)| *v));
                //             s.len() == 4
                //         })
                //         .for_each(|(position, velocities)| {
                //             println!("{position:?} at {velocities:?}")
                //         });
                // }

                // let s1: HashSet<Complex<isize>> = self.carts[0].old_path.keys().cloned().collect();
                // let s3: HashSet<Complex<isize>> = self.carts[2].old_path.keys().cloned().collect();
                // println!("{:?}", s1.intersection(&s3).count());

                // We now know the full path length of each of the three remaining carts.
                // The first and second are on exactly the same path and
                // apparently never collide. I'm thinking the collision happens
                // with the third cart. So, we're going to step through each point
                // where the carts could possibly intersect and figure out how
                // many steps it would take to get there.
                for (i, cart) in self.carts.iter().enumerate() {
                    println!("Cart #{} period is {}.", i, cart.period.unwrap());
                }
            }
        }
    }

    fn part2(&mut self) -> Complex<isize> {
        while self.carts.len() > 1 {
            let crash_site = self.part1();
            self.carts
                .retain(|cart| !crash_site.contains(&cart.position));
            // println!("crash at {crash_site:?}, {} carts left", self.carts.len());
        }
        return self.carts[0].position;
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
    fn extra() {
        assert_eq!(Puzzle::new(SAMPLE3).part1(), vec![Complex::new(0, 1)])
    }
}
