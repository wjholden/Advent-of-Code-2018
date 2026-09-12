use advent_of_code_2018::{ElfcodeVM, Solver};

const PUZZLE: &str = include_str!("../../puzzles/day21.txt");

fn main() {
    let mut solver = Puzzle::new(PUZZLE);
    println!("Part 1: {}", solver.part1());
    println!("Part 2: {}", solver.part2());
}

struct Puzzle {
    vm: ElfcodeVM,
}

impl Solver<usize, usize> for Puzzle {
    fn new(input: &str) -> Self {
        Self {
            vm: ElfcodeVM::new(input),
        }
    }

    fn part1(&mut self) -> usize {
        self.vm.registers[0] = 2884703;
        self.vm.run();
        self.vm.count
    }

    fn part2(&mut self) -> usize {
        does_it_cycle()
    }
}

#[allow(dead_code)]
fn reverse_engineered_v1() {
    let mut r = [0usize; 6];
    r[0] = 2884703;

    // starts at line 0, ends at line 4
    loop {
        r[5] = 123; // line 0
        r[5] = 456 & r[5]; // line 1
        if r[5] == 72 {
            // lines 2-4
            break;
        }
    }

    r[5] = 0; // line 5
    // starts at line 6, ends at line 30
    'outer: loop {
        r[3] = r[5] | 65536; // line 6
        r[5] = 733884; // line 7

        // lines 8-27
        'inner: loop {
            r[1] = r[3] & 255; // line 8
            r[5] += r[1]; // line 9
            r[5] &= 16777215; // line 10
            r[5] *= 65899; // line 11
            r[5] &= 16777215; // line 12

            println!("v1: {r:?}");
            // lines 13-30
            if 256 > r[3] {
                // line 16 -> 28-30
                if r[5] == r[0] {
                    // line 28
                    break 'outer; // line 29
                }
                continue 'outer; // line 30
            } else {
                r[1] = 0; // line 17
                // lines 18-25
                loop {
                    r[2] = r[1] + 1; // line 18
                    r[2] *= 256; // line 19
                    r[2] = usize::from(r[2] > r[3]); // line 20,21
                    if r[2] == 1 {
                        // line 23: goto 26
                        r[3] = r[1]; // line 26
                        continue 'inner; // line 27
                    } else {
                        // line 22: goto 24
                        r[1] += 1; // line 24
                        // line 25: go back to line 18
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
fn reverse_engineered_v2() {
    let mut r = [0usize; 6];
    r[0] = 2884703;

    r[5] = 0; // line 5
    // starts at line 6, ends at line 30
    'outer: loop {
        r[3] = r[5] | 65536; // line 6
        r[5] = 733884; // line 7

        // lines 8-27
        loop {
            r[1] = r[3] & 255; // line 8
            r[5] += r[1]; // line 9
            r[5] &= 16777215; // line 10
            r[5] *= 65899; // line 11
            r[5] &= 16777215; // line 12

            println!("v2: {r:?}");
            // lines 13-30
            if 256 > r[3] {
                // line 16 -> 28-30
                if r[5] == r[0] {
                    // line 28
                    break 'outer; // line 29
                }
                continue 'outer; // line 30
            } else {
                for i in 0.. {
                    // Is this just a division?
                    if (i + 1) * 256 > r[3] {
                        r[3] = i;
                        break;
                    }
                }
            }
        }
    }
}

#[allow(dead_code)]
fn reverse_engineered_v3() {
    let mut r = [0usize; 6];
    r[0] = 2884703;

    r[5] = 0; // line 5
    // starts at line 6, ends at line 30
    'outer: loop {
        r[3] = r[5] | 65536; // line 6
        r[5] = 733884; // line 7

        // lines 8-27
        loop {
            r[1] = r[3] & 255; // line 8
            r[5] += r[1]; // line 9
            r[5] &= 16777215; // line 10
            r[5] *= 65899; // line 11
            r[5] &= 16777215; // line 12

            println!("v3: {r:?}");
            // lines 13-30
            if 256 > r[3] {
                // line 16 -> 28-30
                if r[5] == r[0] {
                    // line 28
                    break 'outer; // line 29
                }
                continue 'outer; // line 30
            } else {
                // It's just division!
                r[3] = r[3] / 256;
            }
        }
    }
}

#[allow(dead_code)]
fn reverse_engineered_v4() {
    let mut r = [0usize; 6];
    r[0] = 2884703;
    loop {
        r[3] = r[5] | 65536;
        r[5] = 733884;
        loop {
            r[5] = (((r[5] + (r[3] & 255)) & 16777215) * 65899) & 16777215;
            println!("v4: {r:?}");
            if 256 > r[3] {
                if r[5] == r[0] {
                    return;
                }
            } else {
                r[3] = r[3] / 256;
            }
        }
    }
}

/// So what does it actually do? My best guess is that this is a PRNG that cycles.
#[allow(dead_code)]
fn reverse_engineered_v5() {
    let mut r = [0usize; 6];
    r[0] = 15400966;
    while r[5] != r[0] {
        r[3] = r[5] | 65536;
        r[5] = 733884;
        r[5] = (((r[5] + (r[3] & 255)) & 16777215) * 65899) & 16777215;
        println!("v5: {r:?}");
        while !(256 > r[3]) {
            r[3] = r[3] >> 8;
            r[5] = (((r[5] + (r[3] & 255)) & 16777215) * 65899) & 16777215;
            println!("v5: {r:?}");
        }
    }
}

/// I can't believe I put this much effort into reverse engineering when the solution was to simply find the cycle!
fn does_it_cycle() -> usize {
    let mut r = [0usize; 6];
    r[0] = 1;
    let mut v = Vec::new();
    loop {
        r[3] = r[5] | 65536;
        r[5] = 733884;
        r[5] = (((r[5] + (r[3] & 255)) & 16777215) * 65899) & 16777215;
        // println!("v5: {r:?}");
        while !(256 > r[3]) {
            r[3] = r[3] >> 8;
            r[5] = (((r[5] + (r[3] & 255)) & 16777215) * 65899) & 16777215;
            // println!("v5: {r:?}");
        }
        if v.contains(&r[5]) {
            return *v.last().unwrap();
        } else {
            v.push(r[5]);
        }
    }
}
