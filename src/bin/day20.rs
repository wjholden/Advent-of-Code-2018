use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use advent_of_code_2018::Solver;

const PUZZLE: &str = include_str!("../../puzzles/day20.txt");

fn main() {
    let mut solver = Puzzle::new(PUZZLE);
    println!("Part 1: {}", solver.part1());
    println!("Part 2: {}", solver.part2()); // 17099 too high
}

struct Puzzle {
    regex: String,
    map: HashMap<(isize, isize), (Tile, usize)>,
}

#[derive(Debug)]
enum Tile {
    Start,
    Room,
    //Wall,
    HDoor,
    VDoor,
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.regex)?;
        let min_row = self.map.keys().map(|k| k.1).min().unwrap();
        let max_row = self.map.keys().map(|k| k.1).max().unwrap();
        let min_col = self.map.keys().map(|k| k.0).min().unwrap();
        let max_col = self.map.keys().map(|k| k.0).max().unwrap();
        for r in min_row - 1..=max_row + 1 {
            for c in min_col - 1..=max_col + 1 {
                write!(
                    f,
                    "{}",
                    match self.map.get(&(c, r)) {
                        Some((Tile::Start, _)) => "X",
                        Some((Tile::Room, _)) => ".",
                        Some((Tile::HDoor, _)) => "-",
                        Some((Tile::VDoor, _)) => "|",
                        None => "#",
                    },
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Solver<usize, usize> for Puzzle {
    fn new(input: &str) -> Self {
        let mut map = HashMap::default();
        map.insert((0, 0), (Tile::Start, 0));

        let mut i = 0;
        let mut pos = VecDeque::from([(0, 0)]);
        let mut doors = VecDeque::from([0]);
        loop {
            match input.chars().nth(i).unwrap() {
                '^' => {}
                'N' => {
                    pos[0].1 -= 1;
                    map.insert(pos[0].clone(), (Tile::HDoor, doors[0]));
                    doors[0] += 1;
                    pos[0].1 -= 1;
                    // map.insert(pos[0].clone(), (Tile::Room, doors[0]));
                    map.entry(pos[0].clone()).or_insert((Tile::Room, doors[0]));
                    doors[0] = doors[0].min(map.get(&pos[0]).unwrap().1);
                }
                'S' => {
                    pos[0].1 += 1;
                    map.insert(pos[0].clone(), (Tile::HDoor, doors[0]));
                    doors[0] += 1;
                    pos[0].1 += 1;
                    // map.insert(pos[0].clone(), (Tile::Room, doors[0]));
                    map.entry(pos[0].clone()).or_insert((Tile::Room, doors[0]));
                    doors[0] = doors[0].min(map.get(&pos[0]).unwrap().1);
                }
                'W' => {
                    pos[0].0 -= 1;
                    map.insert(pos[0].clone(), (Tile::VDoor, doors[0]));
                    doors[0] += 1;
                    pos[0].0 -= 1;
                    // map.insert(pos[0].clone(), (Tile::Room, doors[0]));
                    map.entry(pos[0].clone()).or_insert((Tile::Room, doors[0]));
                    doors[0] = doors[0].min(map.get(&pos[0]).unwrap().1);
                }
                'E' => {
                    pos[0].0 += 1;
                    map.insert(pos[0].clone(), (Tile::VDoor, doors[0]));
                    doors[0] += 1;
                    pos[0].0 += 1;
                    // map.insert(pos[0].clone(), (Tile::Room, doors[0]));
                    map.entry(pos[0].clone()).or_insert((Tile::Room, doors[0]));
                    doors[0] = doors[0].min(map.get(&pos[0]).unwrap().1);
                }
                '(' => {
                    pos.push_front(pos[0].clone());
                    doors.push_front(doors[0]);
                }
                ')' => {
                    pos.pop_front();
                    doors.pop_front();
                }
                '|' => {
                    pos.pop_front();
                    pos.push_front(pos[0].clone());
                    doors.pop_front();
                    doors.push_front(doors[0]);
                }
                '$' => {
                    break;
                }
                _ => panic!("Unexpected character in regex"),
            }
            i += 1;
        }

        Self {
            regex: input.to_owned(),
            map,
        }
    }

    fn part1(&mut self) -> usize {
        println!("{self}");
        self.map.values().map(|v| v.1).max().unwrap()
    }

    fn part2(&mut self) -> usize {
        // Doesn't work.
        self.map
            .values()
            .fold(0, |acc, v| if v.1 >= 1000 { acc + 1 } else { acc })
    }
}

#[cfg(test)]
mod a_regular_map {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new("^WNE$").part1(), 3)
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new("^ENWWW(NEEE|SSE(EE|N))$").part1(), 10)
    }

    #[test]
    fn test3() {
        assert_eq!(
            Puzzle::new("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$").part1(),
            18
        )
    }

    #[test]
    fn test4() {
        assert_eq!(
            Puzzle::new("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$").part1(),
            23
        )
    }

    #[test]
    fn test5() {
        assert_eq!(
            Puzzle::new("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$")
                .part1(),
            31
        )
    }
}
