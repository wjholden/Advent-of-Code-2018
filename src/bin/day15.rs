use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fmt::{Debug, Display},
};

use ndarray::Array2;

const PUZZLE: &str = include_str!("../../puzzles/day15.txt");

fn main() {
    println!("Part 1: {:?}", part1());
    println!("Part 2: {}", part2().unwrap());
}

fn is_adjacent(a: &(usize, usize), b: &(usize, usize)) -> bool {
    let dr = a.0.abs_diff(b.0);
    let dc = a.1.abs_diff(b.1);
    dr + dc == 1
}

#[derive(Debug, PartialEq)]
enum GameResult {
    ElvesWin(usize),
    GoblinsWin(usize),
}

enum GameObject {
    Wall,
    Empty,
    Goblin { hp: usize },
    Elf { hp: usize },
}

impl GameObject {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '.' => Self::Empty,
            'G' => Self::Goblin { hp: 200 },
            'E' => Self::Elf { hp: 200 },
            _ => panic!("unexpected symbol"),
        }
    }

    fn attack(&mut self, power: usize) -> usize {
        match self {
            GameObject::Wall | GameObject::Empty => panic!(),
            GameObject::Goblin { hp } | GameObject::Elf { hp } => {
                *hp = hp.saturating_sub(power);
                *hp
            }
        }
    }
}

impl Display for GameObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameObject::Wall => write!(f, "#"),
            GameObject::Empty => write!(f, "."),
            GameObject::Goblin { hp: _ } => write!(f, "G"),
            GameObject::Elf { hp: _ } => write!(f, "E"),
        }
    }
}

impl Debug for GameObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wall => write!(f, "#"),
            Self::Empty => write!(f, "."),
            Self::Goblin { hp } => write!(f, "G({hp})"),
            Self::Elf { hp } => write!(f, "E({hp})"),
        }
    }
}

struct Puzzle {
    objects: Array2<GameObject>,
    rows: usize,
    cols: usize,
}

impl Puzzle {
    fn new(input: &str) -> Self {
        let rows = input.lines().count();
        let cols = input.lines().next().unwrap().len();
        let data = input
            .lines()
            .flat_map(|line| line.chars().map(GameObject::from));
        Self {
            objects: Array2::from_shape_vec((rows, cols), data.collect()).unwrap(),
            rows,
            cols,
        }
    }

    fn targets(&self, from: (usize, usize)) -> Vec<(usize, usize)> {
        let mut t = Vec::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                match (&self.objects[from], &self.objects[(i, j)]) {
                    (GameObject::Goblin { hp: _ }, GameObject::Elf { hp: _ })
                    | (GameObject::Elf { hp: _ }, GameObject::Goblin { hp: _ }) => t.push((i, j)),
                    _ => {}
                }
            }
        }
        t
    }

    fn in_range(&self, targets: &[(usize, usize)]) -> Vec<(usize, usize)> {
        let mut r = Vec::new();
        for (row, col) in targets {
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let candidate = (row.wrapping_add_signed(dr), col.wrapping_add_signed(dc));
                if let Some(GameObject::Empty) = self.objects.get(candidate)
                    && !r.contains(&candidate)
                {
                    r.push(candidate);
                }
            }
        }
        r
    }

    /// This function combines the reachable, nearest, and chosen steps.
    /// We use a breadth-first search to discover reachable empty tiles,
    /// which are adjacent to our list of targets. This is a fallible
    /// operation: there may be no path to any of those empty tiles.
    ///
    /// This greedy algorithm makes good use of tuple ordering to terminate
    /// upon reaching a solution. The heap is ordered first by distance,
    /// then by reading order, which is exactly what we need.
    fn reachable_nearest_choose(
        &self,
        from: (usize, usize),
        ranges: &[(usize, usize)],
    ) -> Option<(usize, usize)> {
        let mut discovered = HashSet::new();
        let mut frontier = BinaryHeap::new();

        frontier.push(Reverse((0, from)));
        discovered.insert(from);

        while let Some(Reverse((distance, position))) = frontier.pop() {
            if ranges.contains(&position) {
                return Some(position);
            }

            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let candidate = (
                    position.0.wrapping_add_signed(dr),
                    position.1.wrapping_add_signed(dc),
                );
                if let Some(GameObject::Empty) = self.objects.get(candidate)
                    && !discovered.contains(&candidate)
                {
                    discovered.insert(candidate);
                    frontier.push(Reverse((distance + 1, candidate)));
                }
            }
        }

        None
    }

    fn next_step(&self, from: (usize, usize), to: (usize, usize)) -> (usize, usize) {
        // Same as choosing the nearest among many *reachable* targets,
        // selecting the next step towards our chosen target is an
        // infallible operation. Don't call this on something where there
        // is no path.
        let mut neighbors: Vec<(usize, (usize, usize))> = [(-1, 0), (0, -1), (0, 1), (1, 0)]
            .into_iter()
            .filter_map(|(dr, dc)| {
                let candidate = (
                    from.0.wrapping_add_signed(dr),
                    from.1.wrapping_add_signed(dc),
                );

                if let Some(obj) = self.objects.get(candidate)
                    && !matches!(obj, GameObject::Empty)
                {
                    return None;
                }

                let distance = self.bfs(candidate, to);
                if let Some(d) = distance {
                    Some((d, candidate))
                } else {
                    None
                }
            })
            .collect();
        let min_dist = neighbors.iter().min_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
        neighbors.retain(|(d, _position)| *d == min_dist);
        neighbors[0].1
    }

    fn bfs(&self, from: (usize, usize), to: (usize, usize)) -> Option<usize> {
        let mut discovered = HashSet::new();
        let mut frontier = BinaryHeap::new();
        frontier.push(Reverse((0, from)));
        discovered.insert(from);

        while let Some(Reverse((distance, position))) = frontier.pop() {
            if position == to {
                return Some(distance);
            }

            let (row, col) = position;
            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let candidate = (row.wrapping_add_signed(dr), col.wrapping_add_signed(dc));
                if !discovered.contains(&candidate)
                    && matches!(self.objects.get(candidate), Some(GameObject::Empty))
                {
                    discovered.insert(candidate);
                    frontier.push(Reverse((distance + 1, candidate)));
                }
            }
        }

        None
    }

    fn round(&mut self, elf_attack_power: usize) -> bool {
        let mut units = Vec::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                if let Some(GameObject::Elf { hp: _ } | GameObject::Goblin { hp: _ }) =
                    self.objects.get((i, j))
                {
                    units.push((i, j));
                }
            }
        }

        'turn: for mut from in units {
            // Killed units get replaced by empty tiles.
            if let Some(GameObject::Empty) = self.objects.get(from) {
                continue 'turn;
            }

            // First the unit moves.
            let t = self.targets(from);
            if t.is_empty() {
                // println!("{from:?} has nothing to attack");
                return false;
            }

            // Unit only needs to move if it is not adjacent to anyone else.
            if !t.iter().any(|target| is_adjacent(&from, target)) {
                let enemies_in_range = self.in_range(&t);

                if let Some(chosen_destination) =
                    self.reachable_nearest_choose(from, &enemies_in_range)
                {
                    let to = self.next_step(from, chosen_destination);
                    // "move" the game object by swapping.
                    self.objects.swap(from, to);
                    from = to;
                } else {
                    continue 'turn;
                }
            }

            // Then the unit attacks.
            let adj: Vec<_> = t.iter().filter(|t| is_adjacent(&from, t)).collect();
            if adj.is_empty() {
                // Nothing adjacent to attack.
                continue 'turn;
            }
            let (_min_hp, to) = adj
                .into_iter()
                .map(|&t| match self.objects[t] {
                    GameObject::Empty | GameObject::Wall => panic!(),
                    GameObject::Goblin { hp } | GameObject::Elf { hp } => (hp, t),
                })
                .min()
                .unwrap();
            // The adjacent target with the fewest HP.
            // This function returns true if the object's HP went to zero.
            let attack_power = match self.objects[from] {
                GameObject::Wall | GameObject::Empty => panic!(),
                GameObject::Goblin { hp: _ } => 3,
                GameObject::Elf { hp: _ } => elf_attack_power,
            };
            let enemy_health = self.objects[to].attack(attack_power);
            if enemy_health == 0 {
                self.objects[to] = GameObject::Empty;
            }
        }
        true
    }

    fn battle(&mut self, elf_attack_power: usize) -> GameResult {
        let mut i = 0;
        loop {
            if self.round(elf_attack_power) {
                i += 1;
            } else {
                break;
            }
        }
        let mut elves = 0usize;
        let mut goblins = 0usize;
        let hp = self.objects.fold(0, |acc, obj| {
            acc + match obj {
                GameObject::Empty | GameObject::Wall => 0,
                GameObject::Elf { hp } => {
                    elves += 1;
                    *hp
                }
                GameObject::Goblin { hp } => {
                    goblins += 1;
                    *hp
                }
            }
        });
        let outcome = i * hp;
        assert!(elves == 0 || goblins == 0);
        assert!(!(elves == 0 && goblins == 0));
        assert_eq!(elves, self.elf_count());
        if elves > 0 {
            GameResult::ElvesWin(outcome)
        } else {
            GameResult::GoblinsWin(outcome)
        }
    }

    fn elf_count(&self) -> usize {
        self.objects.iter().fold(0, |acc, obj| {
            if matches!(obj, GameObject::Elf { hp: _ }) {
                acc + 1
            } else {
                acc
            }
        })
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.objects.rows() {
            for obj in row {
                write!(f, "{obj}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Debug for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.objects.rows() {
            for obj in row {
                write!(f, "{obj}")?;
            }
            write!(f, "   ")?;
            write!(
                f,
                "{}",
                row.iter()
                    .filter(|obj| {
                        matches!(
                            obj,
                            GameObject::Elf { hp: _ } | GameObject::Goblin { hp: _ }
                        )
                    })
                    .map(|obj| format!("{obj:?}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1() -> usize {
    let mut solver = Puzzle::new(PUZZLE);
    match solver.battle(3) {
        GameResult::ElvesWin(outcome) | GameResult::GoblinsWin(outcome) => outcome,
    }
}

fn part2() -> Option<usize> {
    let elf_count = PUZZLE.chars().filter(|&c| c == 'E').count();
    for i in 4..200 {
        let mut solver = Puzzle::new(PUZZLE);
        match solver.battle(i) {
            GameResult::ElvesWin(outcome) if solver.elf_count() == elf_count => {
                return Some(outcome);
            }
            GameResult::ElvesWin(_) => continue,
            GameResult::GoblinsWin(_) => continue,
        };
    }
    None
}

#[cfg(test)]
mod beverage_bandits {
    use super::*;

    const BASIC: &str = include_str!("../../samples/day15-0.txt");
    const MOVEMENT: &str = include_str!("../../samples/day15-1.txt");
    const BATTLE: &str = include_str!("../../samples/day15-2.txt");

    const BATTLE_36334: &str = include_str!("../../samples/day15-3.txt");
    const BATTLE_39514: &str = include_str!("../../samples/day15-4.txt");
    const BATTLE_27755: &str = include_str!("../../samples/day15-5.txt");
    const BATTLE_28944: &str = include_str!("../../samples/day15-6.txt");
    const BATTLE_18740: &str = include_str!("../../samples/day15-7.txt");

    #[test]
    fn targetting() {
        let p = Puzzle::new(BASIC);
        assert_eq!(p.targets((1, 1)), vec![(1, 4), (3, 2), (3, 5)])
    }

    #[test]
    fn ranging() {
        let p = Puzzle::new(BASIC);
        let t = p.targets((1, 1));
        let mut r = p.in_range(&t);
        r.sort();
        assert_eq!(r, vec![(1, 3), (1, 5), (2, 2), (2, 5), (3, 1), (3, 3)])
    }

    #[test]
    fn reach_nearest_and_choose() {
        let p = Puzzle::new(BASIC);
        let from = (1, 1);
        let t = p.targets(from);
        let ranges = p.in_range(&t);
        let s = p.reachable_nearest_choose(from, &ranges);
        assert_eq!(s, Some((1, 3)))
    }

    #[test]
    fn next_step() {
        let p = Puzzle::new(BASIC);
        let from = (1, 1);
        let t = p.targets(from);
        let r = p.in_range(&t);
        let c = p.reachable_nearest_choose(from, &r);
        let to = p.next_step(from, c.unwrap());
        assert_eq!(to, (1, 2))
    }

    #[test]
    fn basic_2() {
        let p = Puzzle::new(
            "#######
#.E...#
#.....#
#...G.#
#######",
        );
        let from = (1, 2);
        let t = p.targets(from);
        let r = p.in_range(&t);
        let s = p.reachable_nearest_choose(from, &r).unwrap();
        let to = p.next_step(from, s);
        assert_eq!(to, (1, 3))
    }

    #[test]
    fn nothing_in_range() {
        let p = Puzzle::new(
            "#######
#E##..#
####..#
#....G#
#######",
        );
        let from = (1, 1);
        let t = p.targets(from);
        let r = p.in_range(&t);
        let s = p.reachable_nearest_choose(from, &r);
        assert_eq!(s, None)
    }

    #[test]
    fn nothing_at_all() {
        let p = Puzzle::new(
            "#######
#E##..#
####..#
#.....#
#######",
        );
        let from = (1, 1);
        let t = p.targets(from);
        assert!(t.is_empty());
        let r = p.in_range(&t);
        assert!(r.is_empty());
        let s = p.reachable_nearest_choose(from, &r);
        assert_eq!(s, None)
    }

    #[test]
    fn larger_movement() {
        let mut p = Puzzle::new(MOVEMENT);
        for _ in 1..5 {
            p.round(3);
        }
        assert_eq!(
            p.to_string(),
            "#########
#.......#
#..GGG..#
#..GEG..#
#G..G...#
#......G#
#.......#
#.......#
#########
"
        )
    }

    #[test]
    fn losing_battle_animated() {
        let mut p = Puzzle::new(BATTLE);

        for i in 0..48 {
            if i == 0 {
                println!("Initially:");
            } else if i == 1 {
                println!("After 1 round:");
            } else {
                println!("After {i} rounds:");
            }
            println!("{p:?}");
            p.round(3);
            println!();
        }
    }

    #[test]
    fn losing_batle_outcome() {
        let mut p = Puzzle::new(BATTLE);
        assert_eq!(p.battle(3), GameResult::GoblinsWin(27730))
    }

    #[test]
    fn battle_36334() {
        let mut p = Puzzle::new(BATTLE_36334);
        println!("{p:?}");
        let outcome = p.battle(3);
        println!("{p:?}");
        assert_eq!(outcome, GameResult::ElvesWin(36334))
    }

    #[test]
    fn battle_39514() {
        let mut p = Puzzle::new(BATTLE_39514);
        println!("{p:?}");
        let outcome = p.battle(3);
        println!("{p:?}");
        assert_eq!(outcome, GameResult::ElvesWin(39514))
    }

    #[test]
    fn battle_27755() {
        let mut p = Puzzle::new(BATTLE_27755);
        println!("{p:?}");
        let outcome = p.battle(3);
        println!("{p:?}");
        assert_eq!(outcome, GameResult::GoblinsWin(27755))
    }

    #[test]
    fn battle_28944() {
        let mut p = Puzzle::new(BATTLE_28944);
        println!("{p:?}");
        let outcome = p.battle(3);
        println!("{p:?}");
        assert_eq!(outcome, GameResult::GoblinsWin(28944))
    }

    #[test]
    fn battle_18740() {
        let mut p = Puzzle::new(BATTLE_18740);
        println!("{p:?}");
        let outcome = p.battle(3);
        println!("{p:?}");
        assert_eq!(outcome, GameResult::GoblinsWin(18740))
    }
}
