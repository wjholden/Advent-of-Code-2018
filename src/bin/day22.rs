use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

const PUZZLE: &str = include_str!("../../puzzles/day22.txt");

fn main() {
    let mut parameters: Vec<usize> = vec![];
    let re = Regex::new(r"\d+").unwrap();
    for capture in re.captures_iter(PUZZLE) {
        parameters.push(capture[0].parse().unwrap());
    }
    let mut solver = Puzzle::new(parameters[0], (parameters[1], parameters[2]));
    println!("Part 1: {}", solver.part1());
    //println!("Part 2: {}", solver.part2());
}

struct Puzzle {
    depth: usize,
    target: (usize, usize),
    memo: HashMap<(usize, usize), usize>,
}

enum RegionType {
    Rocky,
    Wet,
    Narrow,
}

impl RegionType {
    fn risk_level(&self) -> usize {
        match self {
            Self::Rocky => 0,
            Self::Wet => 1,
            Self::Narrow => 2,
        }
    }
}

impl From<usize> for RegionType {
    fn from(erosion_level: usize) -> Self {
        match erosion_level % 3 {
            0 => Self::Rocky,
            1 => Self::Wet,
            2 => Self::Narrow,
            _ => unreachable!(),
        }
    }
}

impl Puzzle {
    fn new(depth: usize, target: (usize, usize)) -> Self {
        Self {
            depth,
            target,
            memo: HashMap::default(),
        }
    }

    fn geologic_index(&mut self, x: usize, y: usize) -> usize {
        if !self.memo.contains_key(&(x, y)) {
            let gi = match (x, y) {
                (0, 0) => 0,
                (x, 0) => x * 16807,
                (0, y) => y * 48271,
                xy if xy == self.target => 0,
                (x, y) => self.erosion_level(x - 1, y) * self.erosion_level(x, y - 1),
            };
            self.memo.insert((x, y), gi);
        }
        self.memo[&(x, y)]
    }

    fn erosion_level(&mut self, x: usize, y: usize) -> usize {
        (self.geologic_index(x, y) + self.depth) % 20183
    }

    fn part1(&mut self) -> usize {
        let x = 0..=self.target.0;
        let y = 0..=self.target.1;

        x.cartesian_product(y)
            .map(|(x, y)| self.erosion_level(x, y) % 3)
            .sum()
    }

    fn part2(&mut self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod mode_maze {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(510, (10, 10)).part1(), 114)
    }

    #[test]
    fn test2() {}
}
