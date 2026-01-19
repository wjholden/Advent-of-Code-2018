use std::collections::BTreeMap;

use advent_of_code_2018::Solver;

const PUZZLE: &str = include_str!("../../puzzles/day07.txt");

fn main() {
    let solver = Puzzle::new(PUZZLE);
    println!("Part 1: {}", solver.part1());
    println!("Part 2: {}", solver.part2());
}

struct Puzzle {
    dependent: BTreeMap<char, Vec<char>>,
}

impl Solver<String, usize> for Puzzle {
    fn new(input: &str) -> Self {
        let mut dependent: BTreeMap<char, Vec<char>> = BTreeMap::new();
        for line in input.lines() {
            let mut s = line.split_ascii_whitespace();
            let a = s.nth(1).unwrap().as_bytes()[0] as char;
            let b = s.nth_back(2).unwrap().as_bytes()[0] as char;
            dependent.entry(b).or_default().push(a);
            dependent.entry(a).or_default();
        }

        Self { dependent }
    }

    fn part1(&self) -> String {
        let mut completed: Vec<char> = Vec::new();

        while completed.len() < self.dependent.len() {
            for (after, before) in self.dependent.iter() {
                if !completed.contains(after)
                    && (before.is_empty() || before.iter().all(|e| completed.contains(e)))
                {
                    completed.push(*after);
                    break;
                }
            }
        }

        String::from_iter(completed)
    }

    fn part2(&self) -> usize {
        let mut completed: Vec<char> = Vec::new();
        let mut second = 0;

        let workers;
        #[cfg(test)]
        {
            workers = 2;
        }
        #[cfg(not(test))]
        {
            workers = 5;
        }

        let mut tasks: Vec<Task> = Vec::new();

        while completed.len() < self.dependent.len() {
            second += 1;

            tasks.retain_mut(|task| {
                task.tick();
                if task.is_done() {
                    completed.push(task.job);
                }
                !task.is_done()
            });
            'fill_workers: while tasks.len() < workers {
                // find an available task that:
                for (after, before) in self.dependent.iter() {
                    if !completed.contains(after) // isn't already done
                        && tasks.iter().all(|e| e.job != *after) // and isn't in progress
                        && (before.is_empty() || before.iter().all(|e| completed.contains(e)))
                    // and eligible to start.
                    {
                        tasks.push(Task::new(*after));
                        continue 'fill_workers;
                    }
                }
                break 'fill_workers; // no available workers if we got here.
            }
        }

        second - 1 // off by one because we don't actually start doing in the first second.
    }
}

#[derive(Debug)]
struct Task {
    time: usize,
    job: char,
}

impl Task {
    fn new(job: char) -> Self {
        let time_remaining;

        #[cfg(test)]
        {
            time_remaining = (job as usize) - ('A' as usize) + 1;
        }
        #[cfg(not(test))]
        {
            time_remaining = 60 + (job as usize) - ('A' as usize) + 1;
        }

        Self {
            time: time_remaining,
            job,
        }
    }

    fn tick(&mut self) {
        self.time -= 1;
    }

    fn is_done(&self) -> bool {
        self.time == 0
    }
}

#[cfg(test)]
mod puzzle_name {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day07.txt");

    #[test]
    fn test1() {
        assert_eq!(Puzzle::new(SAMPLE).part1(), "CABDFE")
    }

    #[test]
    fn test2() {
        assert_eq!(Puzzle::new(SAMPLE).part2(), 15)
    }
}
