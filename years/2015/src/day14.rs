#[derive(Clone, Debug)]
struct Reindeer {
    speed: i32,
    fly: i32,
    rest: i32,
}

fn parse(input: &str) -> Vec<Reindeer> {
    input
        .lines()
        .map(|l| {
            let parts: Vec<_> = l.split_whitespace().collect();
            Reindeer {
                speed: parts[3].parse().unwrap(),
                fly: parts[6].parse().unwrap(),
                rest: parts[13].parse().unwrap(),
            }
        })
        .collect()
}

fn distance(r: &Reindeer, t: i32) -> i32 {
    let cycle = r.fly + r.rest;
    let full = t / cycle;
    let rem = t % cycle;

    let flying = full * r.fly + rem.min(r.fly);
    flying * r.speed
}

pub fn solve(input: &str) -> (String, String) {
    let rs = parse(input);
    let time = 2503;

    let part1 = rs.iter().map(|r| distance(r, time)).max().unwrap();

    let mut points = vec![0; rs.len()];
    let mut dists = vec![0; rs.len()];

    for t in 1..=time {
        for (i, r) in rs.iter().enumerate() {
            dists[i] = distance(r, t);
        }

        let lead = *dists.iter().max().unwrap();
        for (i, &d) in dists.iter().enumerate() {
            if d == lead {
                points[i] += 1;
            }
        }
    }

    let part2 = *points.iter().max().unwrap();

    (part1.to_string(), part2.to_string())
}
