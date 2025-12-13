fn parse(input: &str) -> Vec<(i32, i32, i32)> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut nums = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());
            (
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
            )
        })
        .collect()
}

fn possible(input: &[(i32, i32, i32)]) -> usize {
    input
        .iter()
        .filter(|(a, b, c)| a + b > *c && a + c > *b && b + c > *a)
        .count()
}

fn possible_vertical(input: &[(i32, i32, i32)]) -> usize {
    input
        .chunks_exact(3)
        .flat_map(|chunk| {
            let (a1, b1, c1) = chunk[0];
            let (a2, b2, c2) = chunk[1];
            let (a3, b3, c3) = chunk[2];

            [(a1, a2, a3), (b1, b2, b3), (c1, c2, c3)]
        })
        .filter(|(a, b, c)| a + b > *c && a + c > *b && b + c > *a)
        .count()
}

pub fn solve(input: &str) -> (String, String) {
    let parsed = parse(input);

    let part1 = possible(&parsed);
    let part2 = possible_vertical(&parsed);

    (part1.to_string(), part2.to_string())
}
