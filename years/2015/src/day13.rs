use aoc_utils::itertools::Itertools;

use std::collections::HashMap;

type Happiness = HashMap<String, HashMap<String, i32>>;

fn parse(input: &str) -> Happiness {
    let mut h: Happiness = HashMap::new();

    for line in input.lines() {
        let line = line.trim_end_matches('.');
        let parts: Vec<_> = line.split_whitespace().collect();

        let a = parts[0].to_string();
        let sign = parts[2];
        let amount: i32 = parts[3].parse().unwrap();
        let b = parts[10].to_string();

        let delta = if sign == "gain" { amount } else { -amount };

        h.entry(a).or_default().insert(b, delta);
    }

    h
}

fn best_score(h: &Happiness) -> i32 {
    let mut people: Vec<&String> = h.keys().collect();
    let first = people.pop().unwrap();

    people
        .iter()
        .permutations(people.len())
        .map(|perm| {
            let mut order = Vec::with_capacity(perm.len() + 1);
            order.push(first);
            order.extend(perm);

            let n = order.len();
            (0..n)
                .map(|i| {
                    let a = order[i];
                    let b = order[(i + 1) % n];
                    h[a][b] + h[b][a]
                })
                .sum()
        })
        .max()
        .unwrap()
}

pub fn solve(input: &str) -> (String, String) {
    let mut h = parse(input);

    let part1 = best_score(&h);

    h.insert("You".into(), HashMap::new());
    let keys: Vec<String> = h.keys().cloned().collect();
    for k in keys {
        h.get_mut("You").unwrap().insert(k.clone(), 0);
        h.get_mut(&k).unwrap().insert("You".into(), 0);
    }

    let part2 = best_score(&h);

    (part1.to_string(), part2.to_string())
}
