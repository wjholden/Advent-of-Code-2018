use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use advent_of_code_2018::Solver;

const PUZZLE: &str = include_str!("../../puzzles/day17.txt");

fn main() {
    let mut solver = Puzzle::new(PUZZLE);
    println!("Part 1: {}", solver.part1());
    //println!("Part 2: {}", solver.part2());
}

#[derive(Debug)]
enum Tile {
    WaterFall,
    WaterStill,
    Clay,
}

#[derive(Debug)]
struct Puzzle {
    tiles: HashMap<(usize, usize), Tile>,
    frontier: VecDeque<(usize, usize)>,
}

impl Puzzle {
    fn bfs(&mut self) {
        while let Some((x, y)) = self.frontier.pop_front() {
            match self.tiles.get(&(x, y)) {
                Some(Tile::Clay) => {
                    self.tiles.insert((x, y - 1), Tile::WaterStill);
                }
                Some(Tile::WaterFall) => {}
                Some(Tile::WaterStill) => {}
                None => {
                    self.tiles.insert((x, y), Tile::WaterFall);
                    self.frontier.push_back((x, y + 1));
                }
            };
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
        dbg!(&[xmin, xmax, ymin, ymax]);
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
        let mut frontier = VecDeque::default();
        frontier.push_back((500, 0));
        Self { tiles, frontier }
    }

    fn part1(&mut self) -> usize {
        self.bfs();
        println!("{self}");
        self.tiles
            .values()
            .filter(|v| matches!(v, Tile::WaterStill | Tile::WaterFall))
            .count()
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
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).part1(), 57)
    }

    #[test]
    fn test2() {}
}
