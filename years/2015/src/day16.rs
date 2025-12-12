use std::collections::HashMap;

fn parse(input: &str) -> Vec<HashMap<String, i32>> {
    input
        .lines()
        .map(|l| {
            let mut m = HashMap::new();

            let (_, rest) = l.split_once(": ").unwrap();
            for part in rest.split(", ") {
                let (k, v) = part.split_once(": ").unwrap();
                m.insert(k.to_string(), v.parse().unwrap());
            }

            m
        })
        .collect()
}

fn reference() -> HashMap<&'static str, i32> {
    HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ])
}

fn matches_part1(sue: &HashMap<String, i32>, refv: &HashMap<&str, i32>) -> bool {
    sue.iter().all(|(k, &v)| refv[k.as_str()] == v)
}

fn matches_part2(sue: &HashMap<String, i32>, refv: &HashMap<&str, i32>) -> bool {
    sue.iter().all(|(k, &v)| {
        let r = refv[k.as_str()];
        match k.as_str() {
            "cats" | "trees" => v > r,
            "pomeranians" | "goldfish" => v < r,
            _ => v == r,
        }
    })
}

pub fn solve(input: &str) -> (String, String) {
    let sues = parse(input);
    let refv = reference();

    let part1 = sues.iter().position(|s| matches_part1(s, &refv)).unwrap() + 1;

    let part2 = sues.iter().position(|s| matches_part2(s, &refv)).unwrap() + 1;

    (part1.to_string(), part2.to_string())
}
