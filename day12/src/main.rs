use std::io::Read;

#[derive(Debug)]
struct Problem {
    width: usize,
    height: usize,
    shape_counts: Vec<usize>,
}

impl Problem {
    fn size(&self) -> u32 {
        (self.width * self.height) as u32
    }
}

fn parse(input: &str) -> Vec<Problem> {
    let lines: Vec<&str> = input.lines().collect();

    let sections: Vec<&[&str]> = lines.split(|l| l.trim().is_empty()).collect();

    let mut boards = vec![];
    for section in sections {
        if section[0].ends_with(':') {
            // shape
        } else {
            for line in section {
                let mut s = line.split(": ");
                let sizes: Vec<usize> = s
                    .next()
                    .unwrap()
                    .split('x')
                    .map(|v| v.parse().unwrap())
                    .collect();
                let shape_ids: Vec<usize> = s
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                boards.push(Problem {
                    width: sizes[0],
                    height: sizes[1],
                    shape_counts: shape_ids,
                })
            }
        }
    }

    boards
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf).unwrap();
    let problems = parse(&buf);
    let part1 = problems
        .iter()
        .filter(|p| p.shape_counts.iter().sum::<usize>() * 9 <= p.size() as usize)
        .count();

    dbg!(part1);
}
