use std::collections::HashSet;

fn parse(input: &str) -> (Vec<(String, String)>, String) {
    let mut lines = input.lines();

    let mut rules = Vec::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let (a, b) = line.split_once(" => ").unwrap();
        rules.push((a.to_string(), b.to_string()));
    }

    let molecule = lines.next().unwrap().to_string();
    (rules, molecule)
}

fn part1(rules: &[(String, String)], mol: &str) -> usize {
    let mut out = HashSet::new();

    for (from, to) in rules {
        let len = from.len();
        for i in 0..=mol.len() - len {
            if &mol[i..i + len] == from {
                let mut s = String::new();
                s.push_str(&mol[..i]);
                s.push_str(to);
                s.push_str(&mol[i + len..]);
                out.insert(s);
            }
        }
    }

    out.len()
}

fn part2(mol: &str) -> usize {
    let atoms = mol.chars().filter(|c| c.is_ascii_uppercase()).count();

    let rn = mol.matches("Rn").count();
    let ar = mol.matches("Ar").count();
    let y = mol.matches('Y').count();

    atoms - rn - ar - 2 * y - 1
}

pub fn solve(input: &str) -> (String, String) {
    let (rules, molecule) = parse(input);

    let p1 = part1(&rules, &molecule);
    let p2 = part2(&molecule);

    (p1.to_string(), p2.to_string())
}
