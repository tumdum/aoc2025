use std::io::Read;

#[derive(Copy, Clone, Debug)]
enum Rotation {
    Left(i16),
    Right(i16),
}

fn rotate(current: i16, dir: Rotation) -> (i16, usize) {
    match dir {
        Rotation::Left(n) => left(current, n),
        Rotation::Right(n) => right(current, n),
    }
}

fn run_rotations(mut current: i16, rotations: &[Rotation]) -> (Vec<i16>, usize) {
    let mut pos = vec![current];
    let mut zeros = 0;
    for rotation in rotations {
        let (new_current, zero) = rotate(current, *rotation);
        current = new_current;
        zeros += zero;
        pos.push(current);
    }
    (pos, zeros)
}

fn parse_rotation(input: &str) -> Rotation {
    if input.starts_with('R') {
        let n = input[1..].parse().unwrap();
        Rotation::Right(n)
    } else if input.starts_with('L') {
        let n = input[1..].parse().unwrap();
        Rotation::Left(n)
    } else {
        unreachable!();
    }
}

fn solve(input: &str) -> (usize, usize) {
    let rotations: Vec<Rotation> = input.lines().map(parse_rotation).collect();
    let (positions, zeros) = run_rotations(50, &rotations);
    (positions.into_iter().filter(|p| *p == 0).count(), zeros)
}

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf).unwrap();
    dbg!(solve(&buf));
}

fn left(current: i16, n: i16) -> (i16, usize) {
    let result = (current - n).rem_euclid(100);
    let zeros = (current - n).div_euclid(100).abs() as usize - if current == 0 { 1 } else { 0 }
        + if result == 0 { 1 } else { 0 };
    (result, zeros)
}

fn right(current: i16, n: i16) -> (i16, usize) {
    ((current + n) % 100, (current + n) as usize / 100)
}
