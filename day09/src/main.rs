use rayon::prelude::*;
use std::io::Read;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn area(&self, b: &Pos) -> i64 {
        ((self.x - b.x).abs() + 1) as i64 * ((self.y - b.y).abs() + 1) as i64
    }

    fn line(&self, to: Pos) -> Vec<Pos> {
        assert!(self.x == to.x || self.y == to.y);
        assert!(*self != to);

        let mut ret = vec![];
        if self.x == to.x {
            let dy = if self.y < to.y { 1 } else { -1 };
            let mut y = self.y;
            loop {
                ret.push(Pos { x: self.x, y });
                if y == to.y {
                    break;
                }
                y += dy;
            }
        } else {
            let dx = if self.x < to.x { 1 } else { -1 };
            let mut x = self.x;
            loop {
                ret.push(Pos { x, y: self.y });
                if x == to.x {
                    break;
                }
                x += dx;
            }
        }
        ret
    }
}

fn parse(input: &str) -> Vec<Pos> {
    input
        .lines()
        .map(|l| {
            let mut s = l.split(',');
            let x: i32 = s.next().unwrap().parse().unwrap();
            let y: i32 = s.next().unwrap().parse().unwrap();
            Pos { x, y }
        })
        .collect()
}

fn border(points: &[Pos]) -> Vec<Pos> {
    let mut ret = vec![];
    for pair in points.windows(2) {
        let l = pair[0].line(pair[1]);
        ret.extend_from_slice(&l);
    }
    ret.extend_from_slice(&points.last().unwrap().line(*points.first().unwrap()));
    ret
}

fn is_inside(rect: [Pos; 2], p: Pos) -> bool {
    let min_x = rect.iter().map(|p| p.x).min().unwrap();
    let min_y = rect.iter().map(|p| p.y).min().unwrap();
    let max_x = rect.iter().map(|p| p.x).max().unwrap();
    let max_y = rect.iter().map(|p| p.y).max().unwrap();

    p.x > min_x && p.x < max_x && p.y > min_y && p.y < max_y
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf).unwrap();
    let map = parse(&buf);
    let mut best = 0;
    for a in &map {
        for b in &map {
            if a != b {
                best = best.max(a.area(b));
            }
        }
    }
    dbg!(best);

    let border = border(&map);

    fn correct_area(a: Pos, b: Pos, border: &[Pos]) -> Option<i64> {
        for p in border {
            if is_inside([a, b], *p) {
                return None;
            }
        }
        Some(a.area(&b))
    }

    let best = map
        .iter()
        .cartesian_product(map.iter())
        .par_bridge()
        .filter(|(a, b)| a != b)
        .flat_map(|(a, b)| correct_area(*a, *b, &border))
        .max()
        .unwrap();
    dbg!(best);
}
