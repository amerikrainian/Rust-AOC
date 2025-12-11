use std::collections::HashSet;

pub fn solve(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input), part2(&input))
}

fn parse(input: &str) -> Vec<(i32, i32)> {
    input
        .chars()
        .map(|c| match c {
            '^' => (0, 1),
            'v' => (0, -1),
            '>' => (1, 0),
            '<' => (-1, 0),
            _ => unreachable!(),
        })
        .collect()
}

fn scan_path(moves: impl Iterator<Item = (i32, i32)>) -> impl Iterator<Item = (i32, i32)> {
    std::iter::once((0, 0)).chain(moves.scan((0, 0), |pos, (dx, dy)| {
        pos.0 += dx;
        pos.1 += dy;
        Some(*pos)
    }))
}

fn part1(input: &[(i32, i32)]) -> String {
    let visited: HashSet<_> = scan_path(input.iter().copied()).collect();
    visited.len().to_string()
}

fn part2(input: &[(i32, i32)]) -> String {
    let santa = scan_path(input.iter().copied().step_by(2));
    let robo = scan_path(input.iter().copied().skip(1).step_by(2));

    let visited: HashSet<_> = santa.chain(robo).collect();
    visited.len().to_string()
}
