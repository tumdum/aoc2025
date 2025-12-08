use std::{cmp::Reverse, collections::HashSet, io::Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
    fn dist(&self, other: &Pos) -> u64 {
        let dx = (self.x - other.x).abs() as u64;
        let dy = (self.y - other.y).abs() as u64;
        let dz = (self.z - other.z).abs() as u64;
        (dx * dx) + (dy * dy) + (dz * dz)
    }
}

fn parse(input: &str) -> Vec<Pos> {
    let mut ret = vec![];
    for line in input.lines() {
        let mut s = line.split(',');
        let x = s.next().unwrap().parse().unwrap();
        let y = s.next().unwrap().parse().unwrap();
        let z = s.next().unwrap().parse().unwrap();
        ret.push(Pos { x, y, z });
    }
    ret
}

fn find_best_n(pos: &[Pos], n: usize) -> HashSet<(Pos, Pos)> {
    let mut distances: Vec<(u64, Pos, Pos)> = vec![];

    for i in 0..pos.len() {
        for j in (i + 1)..pos.len() {
            distances.push((pos[i].dist(&pos[j]), pos[i], pos[j]));
        }
    }

    distances.sort_unstable_by_key(|&(d, _, _)| d);

    distances
        .into_iter()
        .take(n)
        .map(|(_, p1, p2)| (p1, p2))
        .collect()
}

fn extract_one_set(conns: &mut HashSet<(Pos, Pos)>) -> HashSet<Pos> {
    let mut ret: HashSet<Pos> = Default::default();
    if conns.is_empty() {
        return ret;
    }

    let (a, b) = conns.iter().next().unwrap();

    ret.insert(*a);
    ret.insert(*b);

    loop {
        if conns.is_empty() {
            break;
        }

        let mut found: Option<(Pos, Pos)> = None;
        for p in &ret {
            for ab in conns.iter() {
                if ab.0 == *p || ab.1 == *p {
                    found = Some(ab.clone());
                    break;
                }
            }
        }
        match found {
            Some(ab) => {
                ret.insert(ab.0);
                ret.insert(ab.1);
                conns.remove(&ab);
            }
            None => {
                break;
            }
        }
    }

    ret
}

fn find_closest(sets: &[HashSet<Pos>]) -> (Option<(usize, usize)>, Option<(Pos, Pos)>) {
    let mut best = None;
    let mut best_pos = None;
    let mut ret = None;
    for i in 0..sets.len() {
        for j in (i + 1)..sets.len() {
            for a in &sets[i] {
                for b in &sets[j] {
                    let d = a.dist(&b);

                    if best.is_none() || best.unwrap() > d {
                        best = Some(d);
                        if i < j {
                            ret = Some((i, j));
                        } else {
                            ret = Some((j, i));
                        }
                        best_pos = Some((*a, *b));
                    }
                }
            }
        }
    }

    (ret, best_pos)
}

fn find_best(pos: &[Pos]) -> u64 {
    let mut circuits: Vec<HashSet<Pos>> = pos.iter().cloned().map(|p| [p].into()).collect();
    let mut last = 0;
    loop {
        let to_merge = find_closest(&circuits);
        if let (Some((i, j)), Some((pa, pb))) = to_merge {
            let mut a = circuits.remove(j);
            let b = circuits.remove(i);
            a.extend(b);

            circuits.push(a);

            last = pa.x as u64 * pb.x as u64;
        } else {
            return last;
        }
    }
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf).unwrap();
    let map = parse(&buf);

    let mut connections = find_best_n(&map, 1000);

    let mut sets = vec![];
    loop {
        let set = extract_one_set(&mut connections);
        if set.is_empty() {
            break;
        }

        sets.push(set);
    }

    sets.sort_unstable_by_key(|s| Reverse(s.len()));

    let _part1: usize = dbg!(sets.iter().take(3).map(|s| s.len()).product());
    let _part2 = dbg!(find_best(&map));
}
