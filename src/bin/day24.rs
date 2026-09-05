use std::{cell::RefCell, collections::HashMap};

use advent_of_code_2018::Solver;
use regex::Regex;

const PUZZLE: &str = include_str!("../../puzzles/day24.txt");

fn main() {
    let mut solver = Puzzle::new(PUZZLE);
    let mut s2 = solver.clone();
    println!("Part 1: {}", solver.part1());
    // Masterful puzzle, Eric! We get trapped in a loop here at boost=69 because
    // the two surviving units can't do any damage to each other.
    println!("Part 2: {}", s2.part2());
}

#[derive(Debug, Clone)]
struct Army {
    groups: Vec<Group>,
}

impl Army {
    fn boost(&mut self, amount: usize) {
        for group in self.groups.iter_mut() {
            group.attack_damage += amount;
        }
    }

    fn empty(&mut self) {
        for group in self.groups.iter_mut() {
            *group.units.borrow_mut() = 0;
        }
    }

    fn units(&self) -> usize {
        self.groups.iter().fold(0, |a, g| a + *g.units.borrow())
    }

    fn order(&mut self) {
        self.groups.sort_by(|a, b| {
            a.effective_power()
                .cmp(&b.effective_power())
                .reverse()
                .then(a.initiative.cmp(&b.initiative))
        });
    }

    fn target_selection(&self, other: &Self) -> HashMap<usize, usize> {
        let mut target_attacker = HashMap::new();
        for (i, attacker) in self.groups.iter().enumerate() {
            if *attacker.units.borrow() == 0 {
                continue;
            }
            let mut max_damage = 0;
            let mut target = None;
            for (j, candidate) in other.groups.iter().enumerate() {
                if *candidate.units.borrow() == 0 {
                    continue;
                }
                if target_attacker.contains_key(&j) {
                    continue;
                }
                // println!(
                //     "{} group {} would deal defending group {} {} damage",
                //     attacker.army,
                //     attacker.id,
                //     candidate.id,
                //     attacker.damage_against(&candidate)
                // );
                match attacker.damage_against(&candidate) {
                    0 => {}
                    d if d > max_damage => {
                        target = Some(j);
                        max_damage = d;
                    }
                    d if d == max_damage => {
                        let current_defender = &other.groups[target.unwrap()];
                        match candidate
                            .effective_power()
                            .cmp(&current_defender.effective_power())
                        {
                            std::cmp::Ordering::Less => {
                                // stick with the current target
                            }
                            std::cmp::Ordering::Equal => {
                                // choose the one with higher initative
                                if candidate.initiative > current_defender.initiative {
                                    target = Some(j);
                                }
                            }
                            std::cmp::Ordering::Greater => {
                                // take this one because it has a higher effective power
                                target = Some(j);
                            }
                        }
                    }
                    _ => {}
                }
            }
            if let Some(t) = target {
                target_attacker.insert(t, i);
            }
        }
        target_attacker
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Group {
    units: RefCell<usize>,
    hp: usize,
    attack_damage: usize,
    attack_type: String,
    initiative: usize,
    weaknesses: Vec<String>,
    immunities: Vec<String>,
    army: String,
    id: usize,
}

impl Group {
    fn effective_power(&self) -> usize {
        let units = self.units.borrow();
        *units * self.attack_damage
    }

    fn damage_against(&self, other: &Self) -> usize {
        assert_ne!(self.army, other.army);
        if other.immunities.contains(&self.attack_type) {
            0
        } else if other.weaknesses.contains(&self.attack_type) {
            2 * self.effective_power()
        } else {
            self.effective_power()
        }
    }

    fn receive_attack(&self, damage: usize) -> usize {
        let mut units = self.units.borrow_mut();
        let kia = (damage / self.hp).min(*units);
        *units = *units - kia;
        kia
    }
}

#[derive(Debug, Clone)]
struct Puzzle {
    immune_system: Army,
    infection: Army,
}

impl Solver<usize, usize> for Puzzle {
    fn new(input: &str) -> Self {
        // Thank goodness for https://regex101.com.
        let re = Regex::new(
            r"(\d+) units each with (\d+) hit points (?:\((?:(weak|immune) to ([a-z, ]+))?;? ?(?:(weak|immune) to ([a-z, ]+))?\) )?with an attack that does (\d+) ([a-z]+)? damage at initiative (\d+)",
        ).unwrap();
        // dbg!(&re);
        let mut immune_system = Army { groups: Vec::new() }; // satisfy compiler, this value will be discarded
        let mut current = Vec::new();
        let mut id = 0;
        let mut army = "";

        for line in input.lines() {
            match line {
                "Immune System:" => army = "Immune System",
                "" => {}
                "Infection:" => {
                    immune_system.groups = current;
                    current = Vec::new();
                    id = 0;
                    army = "Infection"
                }
                other => {
                    let caps = re.captures(other).unwrap();
                    let units = RefCell::new(caps[1].parse().unwrap());
                    let hp = caps[2].parse().unwrap();
                    let mut weaknesses = Vec::new();
                    let mut immunities = Vec::new();

                    for wi in [3, 5] {
                        if caps.get(wi).is_some() {
                            match &caps[wi] {
                                "weak" => {
                                    for weakness in caps[wi + 1].split(", ") {
                                        weaknesses.push(weakness.to_owned());
                                    }
                                }
                                "immune" => {
                                    for immunity in caps[wi + 1].split(", ") {
                                        immunities.push(immunity.to_owned());
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                    }

                    let attack_damage = caps[7].parse().unwrap();
                    let attack_type = caps[8].to_owned();
                    let initiative = caps[9].parse().unwrap();

                    id = id + 1;

                    let group = Group {
                        units,
                        hp,
                        weaknesses,
                        immunities,
                        attack_damage,
                        attack_type,
                        initiative,
                        id,
                        army: army.to_owned(),
                    };

                    current.push(group);
                }
            }
        }
        let infection = Army { groups: current };
        Self {
            immune_system,
            infection,
        }
    }

    fn part1(&mut self) -> usize {
        // dbg!(&self.immune_system);
        // dbg!(&self.infection);
        while self.immune_system.units() > 0 && self.infection.units() > 0 {
            let mut total_kia = 0;

            // println!("Immune System:");
            // for ig in self.immune_system.groups.iter() {
            //     println!("Group {} contains {} units", ig.id, ig.units.borrow());
            // }
            // println!("Infection:");
            // for ig in self.infection.groups.iter() {
            //     println!("Group {} contains {} units", ig.id, ig.units.borrow());
            // }
            // println!();

            self.immune_system.order();
            self.infection.order();

            let mut attacker_defender = Vec::new();

            for (i, j) in self.infection.target_selection(&self.immune_system) {
                attacker_defender.push((&self.infection.groups[j], &self.immune_system.groups[i]));
            }

            for (i, j) in self.immune_system.target_selection(&self.infection) {
                attacker_defender.push((&self.immune_system.groups[j], &self.infection.groups[i]));
            }

            // println!();

            attacker_defender.sort_by(|a, b| a.0.initiative.cmp(&b.0.initiative).reverse());

            for (attacker, defender) in attacker_defender {
                let damage = attacker.damage_against(defender);
                let kia = defender.receive_attack(damage);
                total_kia += kia;
                // dbg!(attacker);
                // dbg!(defender);
                // println!(
                //     "{} group {} attacks defending group {}, killing {} units",
                //     attacker.army, attacker.id, defender.id, kia
                // );
            }
            // println!();

            if total_kia == 0 {
                // println!("No units were killed in this round. We're stuck!");
                self.infection.empty();
                self.immune_system.empty();
                break;
            }
        }
        self.infection.units() + self.immune_system.units()
    }

    /// I'm not completely happy with this bisection algorithm. Feels like
    /// something isn't right with the in-between cases. But...it works.
    fn part2(&mut self) -> usize {
        let mut left = 1;
        let mut right = 100000;
        while left != right {
            let mut test = self.clone();
            let midpoint = (left + right) / 2;
            test.immune_system.boost(midpoint);
            // dbg!([left, midpoint, right]);
            test.part1();
            let immune_won = test.immune_system.units() > 0;
            if left == midpoint || right == midpoint {
                if !immune_won {
                    left = right;
                }
                break;
            }
            if immune_won {
                right = midpoint;
            } else {
                left = midpoint;
            }
        }
        self.immune_system.boost(left);
        self.part1()
    }
}

#[cfg(test)]
mod immune_system_simulator_20xx {
    use super::*;

    const SAMPLE: &str = include_str!("../../samples/day24.txt");

    #[test]
    fn test1() {
        let mut p = Puzzle::new(SAMPLE);
        // dbg!(&p);
        assert_eq!(p.part1(), 5216)
    }

    #[test]
    fn test2() {
        let mut p = Puzzle::new(SAMPLE);
        p.immune_system.boost(1570);
        assert_eq!(p.part1(), 51)
    }

    #[test]
    fn test3() {
        let mut p = Puzzle::new(SAMPLE);
        assert_eq!(p.part2(), 51);
    }
}
