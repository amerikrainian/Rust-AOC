pub fn solve(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input), part2(&input))
}

fn parse(input: &str) -> Vec<i32> {
    input
        .chars()
        .map(|c| if c == '(' { 1 } else { -1 })
        .collect()
}

fn part1(input: &[i32]) -> String {
    input.iter().sum::<i32>().to_string()
}

fn part2(input: &[i32]) -> String {
    input
        .iter()
        .scan(0, |floor, step| {
            *floor += step;
            Some(*floor)
        })
        .position(|f| f == -1)
        .map(|i| i + 1)
        .unwrap()
        .to_string()
}
