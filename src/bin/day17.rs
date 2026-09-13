use std::{collections::HashMap, fmt::Display};

use advent_of_code_2018::Solver;

const PUZZLE: &str = include_str!("../../puzzles/day17.txt");

fn main() {
    let mut solver = Puzzle::new(PUZZLE);
    println!("Part 1: {}", solver.part1());
    //println!("Part 2: {}", solver.part2());
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
    max_y: usize,
    start: Vec<(usize, usize)>,
    parent: HashMap<(usize, usize), (usize, usize)>,
}

impl Puzzle {
    /// This function is a depth-first search.
    fn drip(&mut self) {
        let (x, mut y) = self.start[0];
        // Fall until you find clay or still water.
        loop {
            y += 1;
            if y > self.max_y {
                self.start.remove(0);
                return;
            }
            self.tiles.insert((x, y), Tile::WaterFall);
            match self.tiles.get(&(x, y + 1)) {
                Some(Tile::Clay | Tile::WaterStill) => {
                    break;
                }
                Some(Tile::WaterFall) | None => {
                    continue;
                }
            }
        }
        if self.is_walled(x, y) {
            self.fill_row(x, y, Tile::WaterStill);
        } else {
            self.fill_row(x, y, Tile::WaterFall);
            self.start.remove(0);
        }
    }

    fn is_fillable(&self, x: usize, y: usize, direction: Direction) -> bool {
        for dx in 1.. {
            let new_x = match direction {
                Direction::Left => x - dx,
                Direction::Right => x + dx,
            };
            if matches!(
                self.tiles.get(&(new_x, y + 1)),
                Some(Tile::WaterFall) | None
            ) {
                return false;
            }
            if matches!(
                self.tiles.get(&(new_x, y)),
                Some(Tile::Clay | Tile::WaterStill)
            ) {
                return true;
            }
        }
        panic!()
    }

    /// This function searches left and right to see if a position (x,y) can
    /// fill up with water.
    fn is_walled(&self, x: usize, y: usize) -> bool {
        self.is_fillable(x, y, Direction::Left) && self.is_fillable(x, y, Direction::Right)
        // let (mut left_is_walled, mut right_is_walled) = (None, None);
        // // First look left.
        // for dx in 1.. {
        //     if matches!(
        //         self.tiles.get(&(x - dx, y + 1)),
        //         Some(Tile::WaterFall) | None
        //     ) {
        //         left_is_walled = Some(false);
        //         break;
        //     }
        //     if matches!(
        //         self.tiles.get(&(x - dx, y)),
        //         Some(Tile::Clay | Tile::WaterFall)
        //     ) {
        //         left_is_walled = Some(true);
        //         break;
        //     }
        // }
        // // Now right.
        // for dx in 1.. {
        //     if self.tiles.get(&(x + dx, y + 1)).is_none() {
        //         right_is_walled = Some(false);
        //         break;
        //     }
        //     if matches!(self.tiles.get(&(x + dx, y)), Some(Tile::Clay)) {
        //         right_is_walled = Some(true);
        //         break;
        //     }
        // }
        // left_is_walled.unwrap() && right_is_walled.unwrap()
    }

    fn fill(&mut self, x: usize, y: usize, direction: Direction, tile: Tile) {
        for dx in 1.. {
            let new_x = match direction {
                Direction::Left => x - dx,
                Direction::Right => x + dx,
            };
            match self.tiles.get(&(new_x, y)) {
                Some(Tile::Clay) => {
                    break;
                }
                Some(Tile::WaterStill) => {
                    println!("{self}");
                    println!("{:?}", self.start);
                    println!(
                        "We are trying to fill a row that is already filled at (x={new_x},y={y})"
                    );
                    let current_start = self.start.remove(0);
                    // dbg!(&self.parent);
                    self.start.push(self.parent[&current_start]);
                    break;
                }
                Some(Tile::WaterFall) | None => {
                    self.tiles.insert((new_x, y), tile);
                }
            }
            // Stop filling if we are over something that is either empty or falling.
            if matches!(
                self.tiles.get(&(new_x, y + 1)),
                Some(Tile::WaterFall) | None
            ) {
                // if self.tiles.get(&(new_x, y + 1)).is_none() {
                assert!(matches!(tile, Tile::WaterFall));
                self.parent.insert((new_x, y), self.start[0]);
                if !self.start.contains(&(new_x, y)) {
                    self.start.push((new_x, y));
                }
                break;
            }
        }
    }

    fn fill_row(&mut self, x: usize, y: usize, tile: Tile) {
        self.fill(x, y, Direction::Left, tile);
        self.fill(x, y, Direction::Right, tile);
        // Finally the center.
        self.tiles.insert((x, y), tile);
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
            for col in xmin..=xmax {
                write!(
                    f,
                    "{}",
                    match self.tiles.get(&(col, row)) {
                        //Some(Tile::Sand) => ".",
                        Some(Tile::Clay) => "#",
                        Some(Tile::WaterStill) => "~",
                        Some(Tile::WaterFall) => "|",
                        // Some(Tile::Water) => "~",
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
        let max_y = tiles.keys().fold(0, |acc, &(_, y)| acc.max(y));
        Self {
            tiles,
            max_y,
            start: vec![(500, 0)],
            parent: HashMap::default(),
        }
    }

    fn part1(&mut self) -> usize {
        while !self.start.is_empty() {
            self.drip();
        }
        println!("{self}");
        self.tiles.values().fold(0, |acc, t| {
            acc + match t {
                Tile::WaterFall | Tile::WaterStill => 1,
                Tile::Clay => 0,
            }
        })
    }

    fn part2(&mut self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod reservoir_research {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day17.txt");

    #[test]
    fn wall_detection() {
        let mut p = Puzzle::new(SAMPLE);
        let x = 500;
        for y in (3..=6).rev() {
            assert!(p.is_walled(x, y) == true);
            p.fill_row(x, y, Tile::WaterStill);
            println!("{p}");
        }
        assert_eq!(p.is_walled(x, 2), false);
        p.fill_row(x, 2, Tile::WaterFall);
        println!("{p}");
        assert!(p.start.contains(&(502, 2)));
    }

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).part1(), 57)
    }

    #[test]
    fn test2() {}
}
