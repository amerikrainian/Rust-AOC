use aoc_utils::itertools::Itertools;

fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn solve(input: &str) -> (String, String) {
    let containers = parse(input);
    let target = 150;

    let mut total = 0;
    let mut best_len = usize::MAX;
    let mut best_count = 0;

    for k in 1..=containers.len() {
        for combo in containers.iter().combinations(k) {
            let sum: i32 = combo.into_iter().copied().sum();

            if sum == target {
                total += 1;

                if k < best_len {
                    best_len = k;
                    best_count = 1;
                } else if k == best_len {
                    best_count += 1;
                }
            }
        }
    }

    (total.to_string(), best_count.to_string())
}
