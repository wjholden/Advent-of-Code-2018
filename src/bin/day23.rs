use regex::Regex;

const PUZZLE: &str = include_str!("../../puzzles/day23.txt");

fn main() {
    let solver = Puzzle::new(PUZZLE);
    println!("Part 1:");
    solver.part1();
    println!("Part 2:");
    solver.part2();
}

struct Puzzle {
    coordinates: Vec<String>,
    radii: Vec<String>,
}

impl Puzzle {
    fn new(input: &str) -> Self {
        let re = Regex::new(r"pos=<(?<coordinate>.+)>, r=(?<radius>\d+)").unwrap();
        let mut coordinates = Vec::new();
        let mut radii = Vec::new();
        for captures in re.captures_iter(input) {
            coordinates.push(captures["coordinate"].to_owned());
            radii.push(captures["radius"].to_owned());
        }
        Self { coordinates, radii }
    }

    fn part1(&self) {
        print!(
            "
int: n = {};

array[1..n, 1..3] of int: nanobots =
[|",
            self.coordinates.len()
        );

        for coordinate in self.coordinates.iter() {
            print!("{coordinate},\n |");
        }

        println!(
            "];

array[1..n] of int: ranges = [{}];

var int: i; % index of the strongest nanobot

var int: in_range;

constraint in_range = sum(j in 1..n)(bool2int(
    abs(nanobots[i,1] - nanobots[j,1]) +
    abs(nanobots[i,2] - nanobots[j,2]) +
    abs(nanobots[i,3] - nanobots[j,3]) <= ranges[i]
));

solve maximize ranges[i];
            ",
            self.radii.join(",")
        );
    }

    fn part2(&self) {
        println!(
            "n = {};
nanobots =
[|{}
 |];
ranges = [{}];",
            self.coordinates.len(),
            self.coordinates.join(",\n |"),
            self.radii.join(",")
        );
    }
}

#[cfg(test)]
mod experimental_emergency_teleportation {
    use super::*;

    const SAMPLE1: &str = include_str!("../../samples/day23-1.txt");
    const SAMPLE2: &str = include_str!("../../samples/day23-2.txt");

    #[test]
    fn test1() {
        Puzzle::new(SAMPLE1).part1()
    }

    #[test]
    fn test2() {
        Puzzle::new(SAMPLE2).part2()
    }
}
