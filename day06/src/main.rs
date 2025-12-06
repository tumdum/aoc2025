use std::io::Read;

#[derive(Debug)]
struct Task {
    op: char,
    nums: Vec<i64>,
}

impl Task {
    fn value(&self) -> i64 {
        if self.op == '+' {
            self.nums.iter().sum()
        } else {
            self.nums.iter().product()
        }
    }
}

fn parse(input: &str) -> (Vec<Task>, Vec<Task>) {
    let input_raw: Vec<String> = input.lines().map(|s| s.to_owned()).collect();
    let input: Vec<Vec<String>> = input_raw
        .iter()
        .map(|l| {
            let nums: Vec<String> = l.split_whitespace().map(|s| s.to_owned()).collect();
            nums
        })
        .collect();

    let mut problems: Vec<Vec<String>> = vec![vec![]; input[0].len()];
    for row in input {
        for (idx, n) in row.into_iter().enumerate() {
            problems[idx].push(n);
        }
    }

    let mut tasks: Vec<Task> = vec![];
    for row in problems {
        let l = row.len();
        let nums = row.iter().take(l - 1).map(|s| s.parse().unwrap()).collect();
        let op = row.last().unwrap().chars().next().unwrap();
        tasks.push(Task { op, nums });
    }

    let mut tasks2: Vec<Task> = vec![];
    let mut nums = vec![];

    let input_raw: Vec<Vec<char>> = input_raw.into_iter().map(|s| s.chars().collect()).collect();
    for c in (0..input_raw[0].len()).rev() {
        let input: String = input_raw.iter().map(|row| row[c]).collect();
        let n = input.trim();
        if n.is_empty() {
            continue;
        }
        if n.chars().all(|c| c.is_digit(10)) {
            nums.push(n.parse().unwrap());
        } else {
            nums.push(n[..n.len() - 1].trim().parse().unwrap());
            tasks2.push(Task {
                op: n.chars().last().unwrap(),
                nums,
            });
            nums = vec![];
        }
    }

    (tasks, tasks2)
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf).unwrap();
    let tasks = parse(&buf);
    let _part1: i64 = dbg!(tasks.0.iter().map(|t| t.value()).sum());
    let _part2: i64 = dbg!(tasks.1.iter().map(|t| t.value()).sum());
}
