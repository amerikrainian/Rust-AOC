use aoc_utils::*;

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    On(Point, Point),
    Off(Point, Point),
    Toggle(Point, Point),
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();

            match parts[0] {
                "turn" => {
                    let start = Point::from_str(parts[2]); // "0,0"
                    let end = Point::from_str(parts[4]); // "999,999"

                    if parts[1] == "on" {
                        Instruction::On(start, end)
                    } else {
                        Instruction::Off(start, end)
                    }
                }

                "toggle" => {
                    let start = Point::from_str(parts[1]); // "0,0"
                    let end = Point::from_str(parts[3]); // "999,999"

                    Instruction::Toggle(start, end)
                }

                _ => panic!("Invalid instruction"),
            }
        })
        .collect()
}

pub fn solve(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input), part2(&input))
}

fn part1(instructions: &[Instruction]) -> String {
    let mut grid = Grid::new(1000, 1000, false);

    for instruction in instructions {
        match *instruction {
            Instruction::On(start, end) => {
                for y in start.y..=end.y {
                    for x in start.x..=end.x {
                        grid[Point::new(x, y)] = true;
                    }
                }
            }
            Instruction::Off(start, end) => {
                for y in start.y..=end.y {
                    for x in start.x..=end.x {
                        grid[Point::new(x, y)] = false;
                    }
                }
            }
            Instruction::Toggle(start, end) => {
                for y in start.y..=end.y {
                    for x in start.x..=end.x {
                        let p = Point::new(x, y);
                        grid[p] = !grid[p];
                    }
                }
            }
        }
    }

    grid.coords().filter(|&p| grid[p]).count().to_string()
}

fn part2(instructions: &[Instruction]) -> String {
    let mut grid = Grid::new(1000, 1000, 0u32);

    for instruction in instructions {
        match *instruction {
            Instruction::On(start, end) => {
                for y in start.y..=end.y {
                    for x in start.x..=end.x {
                        grid[Point::new(x, y)] += 1;
                    }
                }
            }
            Instruction::Off(start, end) => {
                for y in start.y..=end.y {
                    for x in start.x..=end.x {
                        let p = Point::new(x, y);
                        if grid[p] > 0 {
                            grid[p] -= 1;
                        }
                    }
                }
            }
            Instruction::Toggle(start, end) => {
                for y in start.y..=end.y {
                    for x in start.x..=end.x {
                        grid[Point::new(x, y)] += 2;
                    }
                }
            }
        }
    }

    grid.coords().map(|p| grid[p]).sum::<u32>().to_string()
}
