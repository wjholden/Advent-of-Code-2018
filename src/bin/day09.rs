use std::collections::VecDeque;

const PUZZLE: &str = include_str!("../../puzzles/day09.txt");

fn main() {
    let solver = Puzzle::new(PUZZLE);
    println!("Part 1: {}", solver.part1());
    println!("Part 2: {}", solver.part2());
}

struct Puzzle {
    players: usize,
    last_marble: usize,
}

impl Puzzle {
    fn new(input: &str) -> Self {
        let mut it = input.split_ascii_whitespace();
        Self {
            players: it.next().unwrap().parse().unwrap(),
            last_marble: it.nth_back(1).unwrap().parse().unwrap(),
        }
    }

    fn part1(&self) -> usize {
        let mut circle = VecDeque::from([0]);
        let mut scores = vec![0; self.players];

        // This is basically copied from https://www.reddit.com/r/adventofcode/comments/a4i97s/comment/ebepyc7.
        // It isn't so different conceptually from what I had, but there is one
        // very very important distinction: I was calling `VecDeque::insert`
        // instead of rotating the queue. I had assumed that those insertions
        // would be constant time. Apparently not!
        for marble in 1..=self.last_marble {
            if marble.is_multiple_of(23) {
                circle.rotate_left(7);
                let player = marble % self.players;
                scores[player] += marble + circle.pop_front().unwrap();
                circle.rotate_right(1);
            } else {
                circle.rotate_right(1);
                circle.push_front(marble);
            }
        }

        scores.into_iter().max().unwrap()
    }

    fn part2(&self) -> usize {
        Self {
            players: self.players,
            last_marble: self.last_marble * 100,
        }
        .part1()
    }
}

#[cfg(test)]
mod marble_mania {
    use super::*;

    #[test]
    fn test9_25() {
        assert_eq!(
            Puzzle::new("9 players; last marble is worth 25 points").part1(),
            32
        )
    }

    #[test]
    fn test10_1618() {
        assert_eq!(
            Puzzle::new("10 players; last marble is worth 1618 points").part1(),
            8317
        )
    }

    #[test]
    fn test13_7999() {
        assert_eq!(
            Puzzle::new("13 players; last marble is worth 7999 points").part1(),
            146373
        )
    }

    #[test]
    fn test17_1104() {
        assert_eq!(
            Puzzle::new("17 players; last marble is worth 1104 points").part1(),
            2764
        )
    }

    #[test]
    fn test21_6111() {
        assert_eq!(
            Puzzle::new("21 players; last marble is worth 6111 points").part1(),
            54718
        )
    }

    #[test]
    fn test30_5807() {
        assert_eq!(
            Puzzle::new("30 players; last marble is worth 5807 points").part1(),
            37305
        )
    }
}
