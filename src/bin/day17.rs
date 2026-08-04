use std::{collections::HashMap, fmt::Display, usize};

use advent_of_code_2018::Solver;

const PUZZLE: &str = include_str!("../../puzzles/day17.txt");

fn main() {
    let mut solver = Puzzle::new(PUZZLE);
    // println!("{solver}");
    println!("Part 1: {}", solver.part1());
    //println!("Part 2: {}", solver.part2());
}

enum Tile {
    Clay,
    Water,
}

struct Puzzle {
    xmin: usize,
    xmax: usize,
    ymin: usize,
    ymax: usize,
    area: HashMap<(usize, usize), Tile>,
}

impl Puzzle {
    fn drip(&self) -> (usize, usize) {
        let mut x = 500;
        let mut y = self.ymin;
        loop {
            println!("Search {x},{y}");
            // Stop if you've reached the bottom.
            if y == self.ymax {
                break;
            }
            // Nothing below, drop down.
            else if !self.area.contains_key(&(x, y + 1)) {
                y += 1;
            }
            // Stop if you can't go down and you're still at the top.
            else if y == self.ymin {
                break;
            }
            // Look left.
            else if !self.area.contains_key(&(x - 1, y)) {
                x -= 1;
            }
            // Look right.
            else if !self.area.contains_key(&(x + 1, y)) {
                x += 1;
            }
            // Stop if you can't go
            else {
                println!("Something weird happened at {x}, {y}");
                break;
            }
        }
        (x, y)
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.ymin..=self.ymax {
            for x in self.xmin..=self.xmax {
                if let Some(Tile::Clay) = self.area.get(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Solver<usize, usize> for Puzzle {
    fn new(input: &str) -> Self {
        let mut xmin = usize::MAX;
        let mut xmax = usize::MIN;
        let mut ymin = usize::MAX;
        let mut ymax = usize::MIN;

        let mut area = HashMap::new();

        let input = input.replace("=", " ").replace(",", "").replace("..", " ");
        for line in input.lines() {
            let s: Vec<_> = line.split_ascii_whitespace().collect();
            let (a, b, c) = (
                s[1].parse::<usize>().unwrap(),
                s[3].parse::<usize>().unwrap(),
                s[4].parse::<usize>().unwrap(),
            );
            for i in b..=c {
                let (x, y) = if s[0] == "x" { (a, i) } else { (i, a) };
                xmin = usize::min(x, xmin);
                xmax = usize::max(x, xmax);
                ymin = usize::min(y, ymin);
                ymax = usize::max(y, ymax);
                area.insert((x, y), Tile::Clay);
            }
        }

        dbg!([xmin, xmax, ymin, ymax]);

        Self {
            xmin,
            xmax,
            ymin,
            ymax,
            area,
        }
    }

    fn part1(&mut self) -> usize {
        loop {
            let (x, y) = self.drip();
            println!("{x},{y}");
            if self.area.insert((x, y), Tile::Water).is_some() {
                break;
            }
        }
        self.area
            .values()
            .filter(|t| matches!(t, Tile::Water))
            .count()
    }

    fn part2(&mut self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod puzzle_name {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day17.txt");

    #[test]
    fn test1() {
        println!("{}", Puzzle::new(SAMPLE));
        assert_eq!(Puzzle::new(SAMPLE).part1(), 57)
    }

    #[test]
    fn test2() {}
}
