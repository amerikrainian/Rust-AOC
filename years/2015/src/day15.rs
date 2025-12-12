use aoc_utils::itertools::Itertools;

#[derive(Clone, Copy)]
struct Ingredient {
    cap: i32,
    dur: i32,
    fla: i32,
    tex: i32,
    cal: i32,
}

fn parse(input: &str) -> Vec<Ingredient> {
    input
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split_whitespace().collect();
            Ingredient {
                cap: parts[2].trim_end_matches(',').parse().unwrap(),
                dur: parts[4].trim_end_matches(',').parse().unwrap(),
                fla: parts[6].trim_end_matches(',').parse().unwrap(),
                tex: parts[8].trim_end_matches(',').parse().unwrap(),
                cal: parts[10].parse().unwrap(),
            }
        })
        .collect()
}

fn score(ings: &[Ingredient], amounts: &[i32]) -> (i32, i32) {
    let mut cap = 0;
    let mut dur = 0;
    let mut fla = 0;
    let mut tex = 0;
    let mut cal = 0;

    for (ing, &a) in ings.iter().zip(amounts) {
        cap += ing.cap * a;
        dur += ing.dur * a;
        fla += ing.fla * a;
        tex += ing.tex * a;
        cal += ing.cal * a;
    }

    cap = cap.max(0);
    dur = dur.max(0);
    fla = fla.max(0);
    tex = tex.max(0);

    (cap * dur * fla * tex, cal)
}

pub fn solve(input: &str) -> (String, String) {
    let ings = parse(input);
    let n = ings.len();

    let mut best1 = 0;
    let mut best2 = 0;

    for cuts in (0..=100).combinations(n - 1) {
        let mut amounts = Vec::with_capacity(n);
        let mut prev = 0;

        for &c in &cuts {
            amounts.push(c - prev);
            prev = c;
        }
        amounts.push(100 - prev);

        let (s, cal) = score(&ings, &amounts);

        best1 = best1.max(s);
        if cal == 500 {
            best2 = best2.max(s);
        }
    }

    (best1.to_string(), best2.to_string())
}
