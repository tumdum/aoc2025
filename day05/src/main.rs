use std::{io::Read, ops::RangeInclusive};

fn parse(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut ranges: Vec<RangeInclusive<u64>> = vec![];
    let mut ingredients: Vec<u64> = vec![];
    let mut ranges_ended = false;
    for line in input.lines() {
        if !ranges_ended {
            if line.is_empty() {
                ranges_ended = true;
                continue;
            }
            let mut s = line.split('-');
            let min = s.next().unwrap().parse().unwrap();
            let max = s.next().unwrap().parse().unwrap();
            ranges.push(min..=max);
        } else {
            ingredients.push(line.parse().unwrap());
        }
    }

    (ranges, ingredients)
}

fn is_fresh(v: u64, ranges: &[RangeInclusive<u64>]) -> bool {
    ranges.iter().any(|r| r.contains(&v))
}

fn count_distinct(ranges: &[RangeInclusive<u64>]) -> u64 {
    let mut ranges = ranges.to_vec();
    ranges.sort_unstable_by_key(|r| *r.start());

    let mut i = 0;
    loop {
        if i == ranges.len() - 1 {
            break;
        }
        if ranges[i].end() >= ranges[i + 1].start() {
            let end = *ranges.remove(i + 1).end();
            ranges[i] = *ranges[i].start()..=*ranges[i].end().max(&end);
        } else {
            i += 1;
        }
    }

    ranges.into_iter().map(|r| r.count() as u64).sum::<u64>()
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf).unwrap();
    let (ranges, ingredients) = parse(&buf);
    dbg!(
        ingredients
            .iter()
            .filter(|i| is_fresh(**i, &ranges))
            .count()
    );

    dbg!(count_distinct(&ranges));
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use proptest::prelude::*;

    fn naive(ranges: &[RangeInclusive<u64>]) -> u64 {
        ranges
            .iter()
            .cloned()
            .flatten()
            .collect::<HashSet<_>>()
            .len() as u64
    }

    proptest! {
        #[test]
        fn doesnt_crash(ranges: Vec<(u8,u8)>) {
            prop_assume!(!ranges.is_empty());
            let ranges : Vec<RangeInclusive<u64>> = ranges
                .into_iter()
                .map(|(l,r)| (l.min(r) as u64)..=(l.max(r) as u64))
                .collect();

            assert_eq!(naive(&ranges), count_distinct(&ranges), "failing for {ranges:?}");
        }
    }
}
