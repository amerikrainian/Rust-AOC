use aoc_utils::itertools::Itertools;

fn best_qe(weights: &[u64], target: u64) -> u64 {
    for size in 1..=weights.len() {
        let mut best: Option<u64> = None;

        for group in weights.iter().copied().combinations(size) {
            let sum: u64 = group.iter().sum();
            if sum != target {
                continue;
            }

            let qe: u64 = group.iter().product();

            match best {
                Some(b) if qe >= b => {}
                _ => best = Some(qe),
            }
        }

        if let Some(ans) = best {
            return ans;
        }
    }

    unreachable!()
}

pub fn solve(input: &str) -> (String, String) {
    let mut weights: Vec<u64> = input.lines().map(|l| l.parse().unwrap()).collect();

    weights.sort_by(|a, b| b.cmp(a));

    let total: u64 = weights.iter().sum();
    let target = total / 3;

    let part1 = best_qe(&weights, target);

    let target2 = total / 4;
    let part2 = best_qe(&weights, target2);

    (part1.to_string(), part2.to_string())
}
