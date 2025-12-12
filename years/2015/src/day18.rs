use aoc_utils::*;

fn parse(input: &str) -> Grid<bool> {
    let lines: Vec<_> = input.lines().collect();
    let h = lines.len();
    let w = lines[0].len();

    let mut g = Grid::new(w, h, false);

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            g[Point::new(x as isize, y as isize)] = c == '#';
        }
    }

    g
}

fn step(grid: &Grid<bool>) -> Grid<bool> {
    grid.map(|p, &on| {
        let n = grid.neighbors8(p).filter(|&q| grid[q]).count();

        match (on, n) {
            (true, 2 | 3) => true,
            (false, 3) => true,
            _ => false,
        }
    })
}

fn force_corners(grid: &mut Grid<bool>) {
    let w = grid.width() as isize - 1;
    let h = grid.height() as isize - 1;

    grid[Point::new(0, 0)] = true;
    grid[Point::new(w, 0)] = true;
    grid[Point::new(0, h)] = true;
    grid[Point::new(w, h)] = true;
}

pub fn solve(input: &str) -> (String, String) {
    let mut g1 = parse(input);
    let mut g2 = g1.clone();

    for _ in 0..100 {
        g1 = step(&g1);
    }

    let part1 = g1.count(|&b| b);

    force_corners(&mut g2);
    for _ in 0..100 {
        g2 = step(&g2);
        force_corners(&mut g2);
    }

    let part2 = g2.count(|&b| b);

    (part1.to_string(), part2.to_string())
}
