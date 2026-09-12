use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, HashMap, VecDeque},
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
    map: HashMap<(isize, isize), Tile>,
    distances: BTreeMap<(isize, isize), usize>,
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
        for y in min_row - 1..=max_row + 1 {
            for x in min_col - 1..=max_col + 1 {
                write!(
                    f,
                    "{}",
                    match self.map.get(&(x, y)) {
                        Some(Tile::Start) => "X",
                        Some(Tile::Room) => ".",
                        Some(Tile::HDoor) => "-",
                        Some(Tile::VDoor) => "|",
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
        map.insert((0, 0), Tile::Start);

        let mut i = 0;
        let mut pos = VecDeque::from([(0, 0)]);
        loop {
            match input.chars().nth(i).unwrap() {
                '^' => {}
                direction @ ('N' | 'S' | 'E' | 'W') => {
                    let (dx, dy, door_type) = match direction {
                        'N' => (0, -1, Tile::HDoor),
                        'S' => (0, 1, Tile::HDoor),
                        'W' => (-1, 0, Tile::VDoor),
                        'E' => (1, 0, Tile::VDoor),
                        _ => panic!(),
                    };
                    pos[0].0 += dx;
                    pos[0].1 += dy;
                    map.insert(pos[0].clone(), door_type);
                    pos[0].0 += dx;
                    pos[0].1 += dy;
                    map.entry(pos[0].clone()).or_insert(Tile::Room);
                }
                '(' => {
                    pos.push_front(pos[0].clone());
                }
                ')' => {
                    pos.pop_front();
                }
                '|' => {
                    pos[0] = pos[1];
                }
                '$' => {
                    break;
                }
                _ => panic!("Unexpected character in regex"),
            }
            i += 1;
        }

        let mut distances: BTreeMap<(isize, isize), usize> = BTreeMap::new();
        let mut frontier = BinaryHeap::new();
        frontier.push(Reverse((0, (0, 0))));
        while let Some(Reverse((d, (x, y)))) = frontier.pop() {
            if !distances.contains_key(&(x, y)) {
                distances.insert((x, y), d);
            }
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                // Is there a door adjacent to the current position?
                if matches!(map.get(&(x + dx, y + dy)), Some(Tile::VDoor | Tile::HDoor)) {
                    // The candidate room is beyond that door.
                    let (cx, cy) = (x + 2 * dx, y + 2 * dy);
                    // Have we already discovered this position?
                    if !distances.contains_key(&(cx, cy)) {
                        // If not, then let's add this room to the frontier.
                        frontier.push(Reverse((d + 1, (cx, cy))));
                    }
                }
            }
        }

        Self {
            regex: input.to_owned(),
            map,
            distances,
        }
    }

    fn part1(&mut self) -> usize {
        // println!("{self}");
        *self.distances.values().max().unwrap()
    }

    fn part2(&mut self) -> usize {
        // Doesn't work.
        // self.map
        //     .values()
        //     .fold(0, |acc, v| if v.1 >= 1000 { acc + 1 } else { acc })
        self.distances.values().filter(|&&v| v >= 1000).count()
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
