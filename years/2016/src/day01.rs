use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Turn {
    L,
    R,
}

type Step = (Turn, i32);

fn parse(input: &str) -> Vec<Step> {
    input
        .split(", ")
        .map(|s| {
            let (t, n) = s.split_at(1);
            (
                match t {
                    "L" => Turn::L,
                    "R" => Turn::R,
                    _ => unreachable!(),
                },
                n.parse().unwrap(),
            )
        })
        .collect()
}

fn walk(steps: &[Step]) -> (i32, i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dir = 0;

    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut visited = HashSet::from([(0, 0)]);
    let mut first_revisit = None;

    for &(turn, dist) in steps {
        dir = match turn {
            Turn::L => (dir + 3) % 4,
            Turn::R => (dir + 1) % 4,
        };

        let (dx, dy) = dirs[dir];

        for _ in 0..dist {
            x += dx;
            y += dy;

            if first_revisit.is_none() && !visited.insert((x, y)) {
                first_revisit = Some(x.abs() + y.abs());
            }
        }
    }

    (x.abs() + y.abs(), first_revisit.unwrap())
}

pub fn solve(input: &str) -> (String, String) {
    let steps = parse(input);
    let (p1, p2) = walk(&steps);
    (p1.to_string(), p2.to_string())
}
