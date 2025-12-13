use aoc_utils::*;

fn parse(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn travel(grid: &Grid<char>, path: &[Vec<Direction>]) -> String {
    let mut pos = grid.find('5').unwrap();
    let mut code = String::new();

    for line in path {
        for d in line {
            let next = pos + *d;
            if grid.in_bounds(next) && grid[next] != ' ' {
                pos = next;
            }
        }
        code.push(grid[pos]);
    }

    code
}

pub fn solve(input: &str) -> (String, String) {
    let input = parse(input);

    let grid1 = Grid::from(
        "123
456
789",
    );
    let grid2 = Grid::from(
        "  1  
 234 
56789
 ABC 
  D  ",
    );
    let p1 = travel(&grid1, &input);
    let p2 = travel(&grid2, &input);

    (p1, p2)
}
