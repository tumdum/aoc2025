use rayon::prelude::*;
use smallvec::ToSmallVec;
use std::{collections::HashSet, io::Read};

use regex::Regex;

type SVec = smallvec::SmallVec<[u16; 10]>;

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    wiring: Vec<Vec<usize>>,
    joltage: Vec<u16>,
}

impl Machine {
    fn next_step(&self, start: &[bool]) -> Vec<Vec<bool>> {
        self.wiring
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

    fn next_step_j(&self, start: &[u16]) -> Vec<SVec> {
        self.wiring
            .iter()
            .flat_map(|w| {
                let mut next: SVec = start.to_smallvec();
                for i in w {
                    next[*i] += 1;
                    if next[*i] > self.joltage[*i] {
                        return None;
                    }
                }
                Some(next)
            })
            .collect()
    }

    fn find_step_count_j(&self) -> usize {
        let mut start = vec![smallvec::smallvec![0; self.joltage.len()]];
        for n in 1.. {
            let mut next: Vec<SVec> = start
                .into_iter()
                .flat_map(|v| self.next_step_j(&v))
                .collect();
            next.sort_unstable();
            next.dedup();
            if next.iter().any(|l| l.as_slice() == self.joltage) {
                return n;
            }
            start = next;
        }
        unreachable!()
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
        if let Some(cap) = dbg!(r.captures(line)) {
            let lights: Vec<bool> = cap["lights"].chars().map(|c| c == '#').collect();
            let wiring: Vec<Vec<usize>> = cap["wiring"].split(' ').map(parse_one_wiring).collect();
            let joltage: Vec<u16> = cap["joltage"]
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();

            machines.push(Machine {
                lights,
                wiring,
                joltage,
            });
        }
    }

    machines
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf).unwrap();
    let machines = dbg!(parse(&buf));
    /*
    let mut total = 0;
    for m in &machines {
        total += dbg!(m.find_step_count());
    }
    dbg!(total);
    */

    dbg!(machines.iter().map(|m| m.joltage.len()).max());

    let machines: Vec<(usize, Machine)> = machines.into_iter().enumerate().collect();
    let part2: usize = machines
        .par_iter()
        .map(|(id, m)| {
            let ret = m.find_step_count_j();
            println!("{id}: {ret}");
            ret
        })
        .sum();
    dbg!(part2);
}
