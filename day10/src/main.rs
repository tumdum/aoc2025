use rayon::prelude::*;
use regex::Regex;
use std::io::Read;
use z3::ast::Int;
use z3::{Optimize, SatResult};

#[derive(Debug, Clone)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u16>,
}

impl Machine {
    fn next_step(&self, start: &[bool]) -> Vec<Vec<bool>> {
        self.buttons
            .iter()
            .map(|w| {
                let mut next = start.to_vec();
                for i in w {
                    next[*i] = !next[*i];
                }
                next
            })
            .collect()
    }

    fn find_step_count(&self) -> usize {
        let mut start = vec![vec![false; self.lights.len()]];
        for n in 1.. {
            let next: Vec<Vec<bool>> = start.into_iter().flat_map(|v| self.next_step(&v)).collect();
            if next.iter().any(|l| l == &self.lights) {
                return n;
            }
            start = next;
        }
        unreachable!()
    }

    fn solve_p2(&self) -> u64 {
        let opt = Optimize::new();
        let buttons: Vec<_> = (0..self.buttons.len())
            .map(|n| Int::new_const(format!("b{n}")))
            .collect();
        for b in &buttons {
            opt.assert(&b.ge(&0.into()));
        }
        for id in 0..self.joltage.len() {
            let mut bs = vec![];
            for (bid, b) in self.buttons.iter().enumerate() {
                if b.contains(&id) {
                    bs.push(buttons[bid].clone());
                }
            }

            opt.assert(&Int::add(&bs).eq(&Int::from_u64(self.joltage[id] as u64)));
        }

        let steps = Int::fresh_const("steps");
        opt.assert(&steps.eq(Int::add(&buttons)));

        opt.minimize(&steps);
        assert_eq!(opt.check(&[]), SatResult::Sat);
        let model = opt.get_model().unwrap();
        model.eval(&steps, true).unwrap().as_u64().unwrap()
    }
}

fn parse_one_wiring(s: &str) -> Vec<usize> {
    assert_eq!(Some('('), s.chars().next());
    assert_eq!(Some(')'), s.chars().last());

    let s = &s[1..s.len() - 1];
    s.split(',').map(|s| s.parse().unwrap()).collect()
}

fn parse(input: &str) -> Vec<Machine> {
    let mut machines = vec![];
    let r = Regex::new(r#"\[(?<lights>.*)\] (?<wiring>.*) \{(?<joltage>.*)\}"#).unwrap();
    for line in input.lines() {
        if let Some(cap) = r.captures(line) {
            let lights: Vec<bool> = cap["lights"].chars().map(|c| c == '#').collect();
            let wiring: Vec<Vec<usize>> = cap["wiring"].split(' ').map(parse_one_wiring).collect();
            let joltage: Vec<u16> = cap["joltage"]
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();

            machines.push(Machine {
                lights,
                buttons: wiring,
                joltage,
            });
        }
    }

    machines
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf).unwrap();
    let machines = parse(&buf);

    let part1: usize = machines.par_iter().map(|m| m.find_step_count()).sum();
    dbg!(part1);

    let part2: u64 = machines.iter().map(|m| m.solve_p2()).sum();
    dbg!(part2);
}
