use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> (Vec<String>, HashMap<(String, String), u32>) {
    let mut cities = HashSet::new();
    let mut dist = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let a = parts[0].to_string();
        let b = parts[2].to_string();
        let d: u32 = parts[4].parse().unwrap();

        cities.insert(a.clone());
        cities.insert(b.clone());
        dist.insert((a.clone(), b.clone()), d);
        dist.insert((b, a), d);
    }

    (cities.into_iter().collect(), dist)
}

fn route_distance(route: &[String], dist: &HashMap<(String, String), u32>) -> u32 {
    route
        .windows(2)
        .map(|w| dist[&(w[0].clone(), w[1].clone())])
        .sum()
}

fn solve_permutations(cities: &[String], dist: &HashMap<(String, String), u32>) -> (u32, u32) {
    let mut cities = cities.to_vec();

    let mut shortest = u32::MAX;
    let mut longest = 0u32;

    cities.sort();

    loop {
        let d = route_distance(&cities, dist);
        shortest = shortest.min(d);
        longest = longest.max(d);

        if !next_permutation(&mut cities) {
            break;
        }
    }

    (shortest, longest)
}

fn next_permutation<T: Ord>(arr: &mut [T]) -> bool {
    if arr.len() < 2 {
        return false;
    }
    let mut i = arr.len() - 1;
    while i > 0 && arr[i - 1] >= arr[i] {
        i -= 1;
    }
    if i == 0 {
        return false;
    }
    let mut j = arr.len() - 1;
    while arr[j] <= arr[i - 1] {
        j -= 1;
    }
    arr.swap(i - 1, j);
    arr[i..].reverse();
    true
}

pub fn solve(input: &str) -> (String, String) {
    let (cities, dist) = parse(input);

    let (shortest, longest) = solve_permutations(&cities, &dist);

    (shortest.to_string(), longest.to_string())
}
