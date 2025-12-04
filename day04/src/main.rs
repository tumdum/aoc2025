use std::{collections::HashSet, io::Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

struct Map {
    rolls: HashSet<Pos>,
    max_x: i32,
    max_y: i32,
}

impl Map {
    fn is_in(&self, p: Pos) -> bool {
        p.x >= 0 && p.x <= self.max_x && p.y >= 0 && p.y <= self.max_y
    }

    fn can_access(&self, p: Pos) -> bool {
        let mut not_free = 0;
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                let other = Pos {
                    x: p.x + dx,
                    y: p.y + dy,
                };
                if p == other || !self.is_in(other) {
                    continue;
                }
                if self.rolls.contains(&other) {
                    not_free += 1;
                }
            }
        }

        not_free < 4
    }

    fn remove_accessible(&mut self) -> usize {
        let mut removed = 0;

        loop {
            let accessible: Vec<Pos> = self
                .rolls
                .iter()
                .filter(|p| self.can_access(**p))
                .copied()
                .collect();
            if accessible.is_empty() {
                break;
            }
            removed += accessible.len();
            for p in &accessible {
                self.rolls.remove(p);
            }
        }

        removed
    }
}

fn parse(input: &str) -> Map {
    let mut rolls = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, l) in input.lines().enumerate() {
        max_y = max_y.max(y);
        for (x, v) in l.chars().enumerate() {
            max_x = max_x.max(x);
            if v == '@' {
                rolls.insert(Pos {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    Map {
        rolls,
        max_x: max_x as i32,
        max_y: max_y as i32,
    }
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf).unwrap();
    let mut map = parse(&buf);
    dbg!(map.rolls.iter().filter(|p| map.can_access(**p)).count());
    dbg!(map.remove_accessible());
}
