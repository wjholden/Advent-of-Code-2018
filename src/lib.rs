use std::fmt::Display;

pub trait Solver<T, U> {
    fn new(input: &str) -> Self;
    fn part1(&mut self) -> T;
    fn part2(&mut self) -> U;
}

pub fn nsew(x: usize, y: usize) -> [(usize, usize); 4] {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .map(|(dx, dy)| (x.saturating_add_signed(dx), y.saturating_add_signed(dy)))
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Opcode {
    addr,
    addi,
    mulr,
    muli,
    banr,
    bani,
    borr,
    bori,
    setr,
    seti,
    gtir,
    gtri,
    gtrr,
    eqir,
    eqri,
    eqrr,
}

impl From<&str> for Opcode {
    fn from(s: &str) -> Self {
        match s {
            "addr" => Opcode::addr,
            "addi" => Opcode::addi,
            "mulr" => Opcode::mulr,
            "muli" => Opcode::muli,
            "banr" => Opcode::banr,
            "bani" => Opcode::bani,
            "borr" => Opcode::borr,
            "bori" => Opcode::bori,
            "setr" => Opcode::setr,
            "seti" => Opcode::seti,
            "gtir" => Opcode::gtir,
            "gtri" => Opcode::gtri,
            "gtrr" => Opcode::gtrr,
            "eqir" => Opcode::eqir,
            "eqri" => Opcode::eqri,
            "eqrr" => Opcode::eqrr,
            _ => panic!("unrecognized instruction"),
        }
    }
}

impl From<usize> for Opcode {
    fn from(i: usize) -> Self {
        match i {
            1 => Opcode::addr,
            13 => Opcode::addi,
            15 => Opcode::mulr,
            14 => Opcode::muli,
            0 => Opcode::banr,
            9 => Opcode::bani,
            8 => Opcode::borr,
            5 => Opcode::bori,
            3 => Opcode::setr,
            7 => Opcode::seti,
            6 => Opcode::gtir,
            12 => Opcode::gtri,
            4 => Opcode::gtrr,
            10 => Opcode::eqir,
            2 => Opcode::eqri,
            11 => Opcode::eqrr,
            _ => panic!("unrecognized instruction"),
        }
    }
}

#[derive(Debug)]
pub struct ElfcodeInstruction {
    opcode: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

impl Display for ElfcodeInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {} {}", self.opcode, self.a, self.b, self.c)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct ElfcodeVM {
    ip: usize,
    pub registers: [usize; 6],
    instructions: Vec<ElfcodeInstruction>,
    pub count: usize,
}

impl ElfcodeVM {
    pub fn new(program: &str) -> Self {
        let mut instructions = Vec::new();

        let mut lines = program.lines().peekable();

        let has_pragma = lines.peek().unwrap().contains("#");

        let ip = if has_pragma {
            lines
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap()
        } else {
            5
        };

        for line in lines {
            let mut words = line.trim().split_ascii_whitespace();
            let opcode_s = words.next().unwrap();
            let opcode = match opcode_s.parse::<usize>() {
                Ok(i) => Opcode::from(i),
                Err(_) => Opcode::from(opcode_s),
            };
            let a = words.next().unwrap().parse().unwrap();
            let b = words.next().unwrap().parse().unwrap();
            let out = words.next().unwrap().parse().unwrap();
            assert!(words.next().is_none());
            instructions.push(ElfcodeInstruction {
                opcode,
                a,
                b,
                c: out,
            });
        }

        Self {
            ip,
            registers: [0, 0, 0, 0, 0, 0],
            instructions,
            count: 0,
        }
    }

    pub fn current_instruction(&self) -> &ElfcodeInstruction {
        &self.instructions[self.registers[self.ip]]
    }

    pub fn tick(&mut self) {
        // if self.registers[self.ip] == 28 {
        //     println!("{self:?}");
        //     // exit(0);
        // }
        let &ElfcodeInstruction { opcode, a, b, c } = self.current_instruction();
        self.registers[c] = match opcode {
            Opcode::addr => self.registers[a] + self.registers[b],
            Opcode::addi => self.registers[a] + b,
            Opcode::mulr => self.registers[a] * self.registers[b],
            Opcode::muli => self.registers[a] * b,
            Opcode::banr => self.registers[a] & self.registers[b],
            Opcode::bani => self.registers[a] & b,
            Opcode::borr => self.registers[a] | self.registers[b],
            Opcode::bori => self.registers[a] | b,
            Opcode::setr => self.registers[a],
            Opcode::seti => a,
            Opcode::gtir => usize::from(a > self.registers[b]),
            Opcode::gtri => usize::from(self.registers[a] > b),
            Opcode::gtrr => usize::from(self.registers[a] > self.registers[b]),
            Opcode::eqir => usize::from(a == self.registers[b]),
            Opcode::eqri => usize::from(self.registers[a] == b),
            Opcode::eqrr => usize::from(self.registers[a] == self.registers[b]),
        };
        self.registers[self.ip] += 1;
        self.count += 1;
    }

    pub fn run(&mut self) {
        while self.registers[self.ip] < self.instructions.len() {
            // print!(
            //     "ip={} {:?} {} ",
            //     self.registers[self.ip],
            //     self.registers,
            //     self.current_instruction()
            // );
            self.tick();
            // let mut registers_before_advance = self.registers.clone();
            // registers_before_advance[self.ip] -= 1;
            // println!("{:?}", registers_before_advance);
        }
    }
}

#[cfg(test)]
mod elfcode_machine {
    use regex::Regex;

    use super::*;

    #[test]
    fn all_16_instructions() {
        for i in 0..16 {
            let a = Opcode::from(i);
            let s = format!("{:?}", a);
            let b = Opcode::from(s.as_str());
            assert_eq!(a, b);
            // println!("Instruction {i} is {s}.");
        }
    }

    #[test]
    fn day16_ambiguous_cases() {
        let day16 = include_str!("../puzzles/day16.txt");
        let (before_after, test_program) = day16.split_once("\n\n\n\n").unwrap();

        let re = Regex::new(r"Before: \[(?<r0>\d+), (?<r1>\d+), (?<r2>\d+), (?<r3>\d+)\]\n(?<program>(?<opcode>\d+) (?<a>\d+) (?<b>\d+) (?<c>\d+))\nAfter:  \[(?<r4>\d+), (?<r5>\d+), (?<r6>\d+), (?<r7>\d+)\]").unwrap();
        for captures in re.captures_iter(before_after) {
            let program = &captures["program"];
            let mut vm = ElfcodeVM::new(program);
            vm.registers[0] = captures["r0"].parse().unwrap();
            vm.registers[1] = captures["r1"].parse().unwrap();
            vm.registers[2] = captures["r2"].parse().unwrap();
            vm.registers[3] = captures["r3"].parse().unwrap();
            let expect = ["r4", "r5", "r6", "r7"].map(|r| captures[r].parse::<usize>().unwrap());
            // println!(
            //     "{program} (opcode {:?}) initialized with {:?} should produce {expect:?}",
            //     Opcode::from(vm.instructions[0].opcode),
            //     &vm.registers[0..4]
            // );

            vm.tick();
            assert_eq!(expect, vm.registers[0..4]);
        }

        let mut machine = ElfcodeVM::new(test_program.trim());
        machine.run();
        assert_eq!(machine.registers[0], 649);
    }

    #[test]
    fn day19_example() {
        let program = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

        let mut vm = ElfcodeVM::new(program);
        assert_eq!(vm.ip, 0);
        vm.run();
        assert_eq!(vm.registers, [7, 5, 6, 0, 0, 9]);
    }

    #[test]
    fn day19_part1() {
        let day19 = include_str!("../puzzles/day19.txt");
        let mut vm = ElfcodeVM::new(day19);
        vm.run();
        assert_eq!(vm.registers[0], 2040);
    }
}
