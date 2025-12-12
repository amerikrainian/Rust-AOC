fn parse(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}

fn decoded_len(s: &str) -> usize {
    let inner = &s[1..s.len() - 1];
    let mut chars = inner.chars().peekable();
    let mut count = 0;

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('\\') | Some('"') => {
                    count += 1;
                }
                Some('x') => {
                    chars.next();
                    chars.next();
                    count += 1;
                }
                _ => unreachable!(),
            }
        } else {
            count += 1;
        }
    }

    count
}

fn encoded_len(s: &str) -> usize {
    let mut count = 2;
    for c in s.chars() {
        match c {
            '"' | '\\' => count += 2,
            _ => count += 1,
        }
    }
    count
}

fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|line| line.len() - decoded_len(line))
        .sum()
}

fn part2(input: &[String]) -> usize {
    input
        .iter()
        .map(|line| encoded_len(line) - line.len())
        .sum()
}

pub fn solve(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input).to_string(), part2(&input).to_string())
}
