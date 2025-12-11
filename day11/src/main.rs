use std::{
    collections::HashMap,
    collections::hash_map::Entry::{Occupied, Vacant},
    io::Read,
};

type Map = HashMap<String, Vec<String>>;

fn parse(input: &str) -> Map {
    let mut m: Map = Default::default();
    for l in input.lines() {
        let mut s = l.split(": ");
        let from = s.next().unwrap();
        let targets = s.next().unwrap();
        let targets = targets.split_whitespace().map(|s| s.to_owned()).collect();
        m.insert(from.to_owned(), targets);
    }

    m
}

fn dfs(map: &Map, starting: &str, ending: &str) -> u64 {
    let mut current: HashMap<String, u64> = Default::default();
    current.insert(starting.to_owned(), 1);
    let mut ret = 0;
    while !current.is_empty() {
        let mut next_current: HashMap<String, u64> = Default::default();

        for (from, count) in current {
            if from == ending {
                ret += count;
            }
            if let Some(targets) = map.get(&from) {
                for target in targets {
                    *next_current.entry(target.clone()).or_default() += count;
                }
            }
        }

        current = next_current;
    }

    ret
}
fn dfs2(map: &Map, starting: &str, ending: &str) -> u64 {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
    struct Entry {
        fft: bool,
        dac: bool,
    }
    let mut current: HashMap<String, HashMap<Entry, u64>> = Default::default();
    current
        .entry(starting.to_owned())
        .or_default()
        .insert(Entry::default(), 1);

    let mut ret = 0;
    while !current.is_empty() {
        let mut next_current: HashMap<String, HashMap<Entry, u64>> = Default::default();

        for (from, counts) in current {
            if from == ending {
                for (entry, count) in &counts {
                    if entry.fft && entry.dac {
                        ret += count;
                    }
                }
            }

            if let Some(targets) = map.get(&from) {
                for target in targets {
                    for (entry, count) in &counts {
                        let mut entry = entry.clone();
                        if target == "fft" {
                            entry.fft = true;
                        } else if target == "dac" {
                            entry.dac = true;
                        }
                        *next_current
                            .entry(target.clone())
                            .or_default()
                            .entry(entry)
                            .or_default() += count;
                    }
                }
            }
        }

        current = next_current;
    }

    ret
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf).unwrap();
    let map = parse(&buf);

    dbg!(dfs(&map, "you", "out"));
    dbg!(dfs2(&map, "svr", "out"));
}
