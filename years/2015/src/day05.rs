pub fn solve(input: &str) -> (String, String) {
    (part1(input), part2(input))
}

fn part1(input: &str) -> String {
    let count = input
        .lines()
        .filter(|line| {
            let vowels = line.chars().filter(|&c| "aeiou".contains(c)).count();

            let has_double = line.chars().zip(line.chars().skip(1)).any(|(a, b)| a == b);

            let has_forbidden = ["ab", "cd", "pq", "xy"]
                .iter()
                .any(|&forb| line.contains(forb));

            vowels >= 3 && has_double && !has_forbidden
        })
        .count();

    count.to_string()
}

fn part2(input: &str) -> String {
    let count = input
        .lines()
        .filter(|line| {
            let has_pair = (0..line.len().saturating_sub(1)).any(|i| {
                let pair = &line[i..i + 2];
                line[i + 2..].contains(pair)
            });

            let has_repeat = line.chars().zip(line.chars().skip(2)).any(|(a, b)| a == b);

            has_pair && has_repeat
        })
        .count();

    count.to_string()
}
