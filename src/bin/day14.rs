const PUZZLE: &str = include_str!("../../puzzles/day14.txt");

/// Searching for the desired sequence at the end of the array works, but you
/// have to account for there being one or two digits we append to the
/// sequence. I was expecting this to be a very hard Fibonacci-like problem,
/// but you can use ordinary arrays for this.
fn main() {
    let iterations = PUZZLE.trim().parse().unwrap();
    println!("Part 1: {}", part1(iterations));
    println!("Part 2: {}", part2(&sequence(PUZZLE)));
}

fn sequence(input: &str) -> Vec<usize> {
    input
        .trim()
        .chars()
        .flat_map(|c| c.to_digit(10))
        .map(|u| u as usize)
        .collect()
}

fn part1(n: usize) -> String {
    let mut v = vec![3, 7];
    let mut e1 = 0;
    let mut e2 = 1;

    while v.len() < n + 10 {
        let new_recipe = v[e1] + v[e2];
        assert!(new_recipe <= 18);
        if new_recipe >= 10 {
            v.push(new_recipe / 10);
        }
        v.push(new_recipe % 10);
        let shift1 = 1 + v[e1];
        let shift2 = 1 + v[e2];
        e1 = (e1 + shift1) % v.len();
        e2 = (e2 + shift2) % v.len();
    }

    v[n..n + 10].iter().fold(String::new(), |mut a, &i| {
        a.push(char::from_digit(i as u32, 10).unwrap());
        a
    })
}

fn part2(seq: &[usize]) -> usize {
    let mut v = vec![3, 7];
    let mut e1 = 0;
    let mut e2 = 1;

    let n = seq.len();

    loop {
        let new_recipe = v[e1] + v[e2];
        assert!(new_recipe <= 18);
        if new_recipe >= 10 {
            v.push(new_recipe / 10);
            if v.len() >= n && &v[v.len() - n..] == seq {
                return v.len() - n;
            }
        }
        v.push(new_recipe % 10);
        // Repeat the terminating condition test because it could happen on
        // the first or the second digit we appended.
        if v.len() >= n && &v[v.len() - n..] == seq {
            return v.len() - n;
        }
        let shift1 = 1 + v[e1];
        let shift2 = 1 + v[e2];
        e1 = (e1 + shift1) % v.len();
        e2 = (e2 + shift2) % v.len();
    }
}

#[cfg(test)]
mod chocolate_charts {
    use super::*;

    #[test]
    fn part1_9() {
        assert_eq!(part1(9), "5158916779")
    }

    #[test]
    fn part1_5() {
        assert_eq!(part1(5), "0124515891")
    }

    #[test]
    fn part1_18() {
        assert_eq!(part1(18), "9251071085")
    }

    #[test]
    fn part1_2018() {
        assert_eq!(part1(2018), "5941429882")
    }

    #[test]
    fn part2_51589() {
        assert_eq!(part2(&[5, 1, 5, 8, 9]), 9)
    }

    #[test]
    fn part2_01245() {
        assert_eq!(part2(&[0, 1, 2, 4, 5]), 5)
    }

    #[test]
    fn part2_92510() {
        assert_eq!(part2(&[9, 2, 5, 1, 0]), 18)
    }

    #[test]
    fn part2_59414() {
        assert_eq!(part2(&[5, 9, 4, 1, 4]), 2018)
    }
}
