use std::io::Read;

fn find2(line: &Vec<u32>) -> u64 {
    let mut output = 0u64;
    let l = line.len();
    let mut start = 0;
    for bat in 0..12 {
        let input = &line[start..l - (11 - bat)];
        let digit = input.iter().max().unwrap();
        let idx = start + input.iter().position(|v| v == digit).unwrap();
        output = output * 10 + *digit as u64;
        start = idx + 1;
    }
    output
}

fn find(line: &Vec<u32>) -> u32 {
    let l = line.len();
    let first = line.iter().take(l - 1).max().unwrap();
    let idx = line.iter().position(|v| v == first).unwrap();
    let last = line[idx + 1..].iter().max().unwrap();

    first * 10 + last
}

fn solve(input: &str) -> (u32, u64) {
    let lines: Vec<_> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let part1 = lines.iter().map(find).sum();
    let part2 = lines.iter().map(find2).sum();
    (part1, part2)
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf).unwrap();
    dbg!(solve(&buf));
}
