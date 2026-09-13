use std::{collections::HashMap, fmt::Display};

use advent_of_code_2018::Solver;

const PUZZLE: &str = include_str!("../../puzzles/day17.txt");

fn main() {
    let mut solver = Puzzle::new(PUZZLE);
    solver.drop_iter(500, 0);
    println!("{solver}");
    println!("Part 1: {}", solver.part1());
    println!("Part 2: {}", solver.part2());
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    WaterFall,
    WaterStill,
    Clay,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Puzzle {
    tiles: HashMap<(usize, usize), Tile>,
    min_y: usize,
    max_y: usize,
}

impl Puzzle {
    /// Took a little help from Google's AI to transform my recursive version
    /// into the iterative version you see here. Runs surprisingly fast!
    fn drop_iter(&mut self, x: usize, y: usize) {
        let mut stack = vec![(x, y)];

        while let Some((x, y)) = stack.pop() {
            // println!("{self}");
            // Base case
            if y >= self.max_y {
                // return;
                continue;
            }

            // Recursive case
            // Fill down as far as we can.
            match self.tiles.get(&(x, y + 1)) {
                None => {
                    self.tiles.insert((x, y + 1), Tile::WaterFall);
                    // self.drop(x, y + 1);
                    stack.push((x, y)); // we need to come back later
                    stack.push((x, y + 1));
                    continue;
                }
                Some(Tile::WaterFall) => {
                    // return;
                    continue;
                }
                _ => {}
            }

            // Ok, we filled so far down that we're either on still water or clay.
            // See if we can fill to the left or right.
            let left_fillable = self.fillable_v2(x, y, Direction::Left);
            let right_fillable = self.fillable_v2(x, y, Direction::Right);
            // If both are fillable then we need to flood this row.
            match (left_fillable, right_fillable) {
                (true, true) => {
                    self.flood(x, y, Direction::Left, Tile::WaterStill);
                    self.flood(x, y, Direction::Right, Tile::WaterStill);
                }
                (true, false) => {
                    self.flood(x, y, Direction::Left, Tile::WaterFall);
                    let rx = self.flood(x, y, Direction::Right, Tile::WaterFall);
                    // self.drop(rx, y);
                    stack.push((rx, y));
                }
                (false, true) => {
                    self.flood(x, y, Direction::Right, Tile::WaterFall);
                    let lx = self.flood(x, y, Direction::Left, Tile::WaterFall);
                    // self.drop(lx, y);
                    stack.push((lx, y));
                }
                (false, false) => {
                    let rx = self.flood(x, y, Direction::Right, Tile::WaterFall);
                    // self.drop(rx, y);
                    stack.push((rx, y));
                    let lx = self.flood(x, y, Direction::Left, Tile::WaterFall);
                    // self.drop(lx, y);
                    stack.push((lx, y));
                }
            }
        }
    }

    #[deprecated]
    #[allow(dead_code)]
    fn drop(&mut self, x: usize, y: usize) {
        // Base case
        if y >= self.max_y {
            return;
        }

        // Recursive case
        // Fill down as far as we can.
        match self.tiles.get(&(x, y + 1)) {
            None => {
                self.tiles.insert((x, y + 1), Tile::WaterFall);
                self.drop(x, y + 1);
            }
            Some(Tile::WaterFall) => {
                return;
            }
            _ => {}
        }

        // Ok, we filled so far down that we're either on still water or clay.
        // See if we can fill to the left or right.
        let left_fillable = self.fillable_v2(x, y, Direction::Left);
        let right_fillable = self.fillable_v2(x, y, Direction::Right);
        dbg!([left_fillable, right_fillable]);
        // If both are fillable then we need to flood this row.
        match (left_fillable, right_fillable) {
            (true, true) => {
                self.flood(x, y, Direction::Left, Tile::WaterStill);
                self.flood(x, y, Direction::Right, Tile::WaterStill);
            }
            (true, false) => {
                self.flood(x, y, Direction::Left, Tile::WaterFall);
                let rx = self.flood(x, y, Direction::Right, Tile::WaterFall);
                self.drop(rx, y);
            }
            (false, true) => {
                self.flood(x, y, Direction::Right, Tile::WaterFall);
                let lx = self.flood(x, y, Direction::Left, Tile::WaterFall);
                self.drop(lx, y);
            }
            (false, false) => {
                let rx = self.flood(x, y, Direction::Right, Tile::WaterFall);
                self.drop(rx, y);
                let lx = self.flood(x, y, Direction::Left, Tile::WaterFall);
                self.drop(lx, y);
            }
        }
    }

    fn flood(&mut self, x: usize, y: usize, direction: Direction, tile: Tile) -> usize {
        let mut new_x = x;
        while !matches!(self.tiles.get(&(new_x, y)), Some(Tile::Clay)) {
            self.tiles.insert((new_x, y), tile);
            if matches!(
                self.tiles.get(&(new_x, y + 1)),
                None | Some(Tile::WaterFall)
            ) {
                return new_x;
            }
            new_x = match direction {
                Direction::Left => new_x - 1,
                Direction::Right => new_x + 1,
            };
        }
        new_x
    }

    fn fillable_v2(&self, x: usize, y: usize, direction: Direction) -> bool {
        // println!("let's see if ({x},{y}) is fillable from the {direction:?}");
        let mut new_x = x;
        loop {
            // dbg!([new_x, y]);
            if matches!(
                self.tiles.get(&(new_x, y + 1)),
                Some(Tile::WaterFall) | None
            ) {
                return false;
            }
            new_x = match direction {
                Direction::Left => new_x - 1,
                Direction::Right => new_x + 1,
            };
            if matches!(
                self.tiles.get(&(new_x, y)),
                Some(Tile::Clay) | Some(Tile::WaterStill)
            ) {
                return true;
            }
        }
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (xmin, xmax, ymin, ymax) = self.tiles.keys().fold(
            (usize::MAX, 0, usize::MAX, 0),
            |(xmin, xmax, ymin, ymax), (x, y)| {
                (xmin.min(*x), xmax.max(*x), ymin.min(*y), ymax.max(*y))
            },
        );
        for row in ymin..=ymax {
            for col in xmin - 1..=xmax + 1 {
                write!(
                    f,
                    "{}",
                    match self.tiles.get(&(col, row)) {
                        Some(Tile::Clay) => "#",
                        Some(Tile::WaterStill) => "~",
                        Some(Tile::WaterFall) => "|",
                        None => ".",
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Solver<usize, usize> for Puzzle {
    fn new(input: &str) -> Self {
        let input = input.replace("..", " ");
        let input = input.replace("=", " ");
        let input = input.replace(",", "");
        let mut tiles = HashMap::default();
        for line in input.lines() {
            let line: Vec<_> = line.split_ascii_whitespace().collect();
            let v = [line[1], line[3], line[4]];
            let v = v.map(|s| s.parse::<usize>().unwrap());
            match line[0] {
                "x" => {
                    for y in v[1]..=v[2] {
                        tiles.insert((v[0], y), Tile::Clay);
                    }
                }
                "y" => {
                    for x in v[1]..=v[2] {
                        tiles.insert((x, v[0]), Tile::Clay);
                    }
                }
                _ => panic!(),
            }
        }
        let (min_y, max_y) = tiles
            .keys()
            .fold((usize::MAX, 0), |acc, &(_, y)| (acc.0.min(y), acc.1.max(y)));
        Self {
            tiles,
            min_y,
            max_y,
        }
    }

    fn part1(&mut self) -> usize {
        self.tiles.iter().fold(0, |acc, ((_, y), t)| {
            if *y >= self.min_y && matches!(t, Tile::WaterFall | Tile::WaterStill) {
                acc + 1
            } else {
                acc
            }
        })
    }

    fn part2(&mut self) -> usize {
        self.tiles.iter().fold(0, |acc, ((_, y), t)| {
            if *y >= self.min_y && matches!(t, Tile::WaterStill) {
                acc + 1
            } else {
                acc
            }
        })
    }
}

#[cfg(test)]
mod reservoir_research {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day17.txt");

    #[test]
    fn test1() {
        let mut p = Puzzle::new(SAMPLE);
        p.drop_iter(500, 0);
        assert_eq!(p.part1(), 57)
    }

    #[test]
    fn test2() {
        let mut p = Puzzle::new(SAMPLE);
        p.drop_iter(500, 0);
        assert_eq!(p.part2(), 29)
    }
}
