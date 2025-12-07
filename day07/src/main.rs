use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn next(&self) -> Pos {
        Pos {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn split(&self) -> [Pos; 2] {
        [
            Pos {
                x: self.x - 1,
                y: self.y,
            },
            Pos {
                x: self.x + 1,
                y: self.y,
            },
        ]
    }
}

#[derive(Debug)]
struct Map {
    splitters: HashSet<Pos>,
    start: Pos,
    max_y: i32,
}

fn parse(input: &str) -> Map {
    let mut splitters = HashSet::new();
    let mut start = Pos { x: 0, y: 0 };
    let mut max_y = 0i32;

    for (y, l) in input.lines().enumerate() {
        max_y = max_y.max(y as i32);
        for (x, v) in l.chars().enumerate() {
            if v == '^' {
                splitters.insert(Pos {
                    x: x as i32,
                    y: y as i32,
                });
            } else if v == 'S' {
                start = Pos {
                    x: x as i32,
                    y: y as i32,
                };
            }
        }
    }

    Map {
        splitters,
        start,
        max_y,
    }
}

impl Map {
    fn run(&self) {
        let mut splits = 0;
        let mut current = HashMap::new();
        current.insert(self.start, 1);

        loop {
            let mut next_current = HashMap::new();

            for (pos, count) in current {
                let next = pos.next();
                if self.splitters.contains(&next) {
                    splits += 1;
                    let s = next.split();
                    *next_current.entry(s[0]).or_default() += count;
                    *next_current.entry(s[1]).or_default() += count;
                } else {
                    *next_current.entry(next).or_default() += count;
                }
            }

            current = next_current;
            if current.iter().all(|(p, _)| p.y > self.max_y) {
                break;
            }
        }

        dbg!(splits);
        let _: u64 = dbg!(current.values().sum());
    }
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf).unwrap();
    let map = parse(&buf);

    map.run();
}
