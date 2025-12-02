use std::{io::Read, ops::RangeInclusive};

fn parse_range(s: &str) -> RangeInclusive<u64> {
    let mut s = s.split('-');
    let first = s.next().unwrap().parse().unwrap();
    let last = s.next().unwrap().parse().unwrap();
    first..=last
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf).unwrap();
    let ranges = buf.trim().split(',').map(parse_range).collect::<Vec<_>>();
    dbg!(
        ranges
            .iter()
            .flat_map(|r| r.clone().filter(|v| is_invalid(*v)))
            .sum::<u64>()
    );
    dbg!(
        ranges
            .into_iter()
            .flat_map(|r| r.filter(|v| is_invalid2(*v)))
            .sum::<u64>()
    );
}

fn is_invalid(n: u64) -> bool {
    let s = n.to_string();
    if s.len() % 2 != 0 {
        return false;
    }
    let n1 = s.chars().take(s.len() / 2);
    let n2 = s.chars().skip(s.len() / 2);
    n1.zip(n2).all(|(c1, c2)| c1 == c2)
}

fn is_invalid2(n: u64) -> bool {
    let chars: Vec<_> = n.to_string().chars().collect();
    for size in 1..chars.len() {
        if chars.len() % size != 0 {
            continue;
        }
        let mut chunks = chars.chunks(size).collect::<Vec<_>>();
        chunks.sort_unstable();
        if chunks.first() == chunks.last() {
            return true;
        }
    }
    return false;
}
