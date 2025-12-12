#[derive(Clone, Copy)]
enum Reg {
    A,
    B,
}

#[derive(Clone, Copy)]
enum Instr {
    Hlf(Reg),
    Tpl(Reg),
    Inc(Reg),
    Jmp(isize),
    Jie(Reg, isize),
    Jio(Reg, isize),
}

fn parse(input: &str) -> Vec<Instr> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts[0] {
                "hlf" => Instr::Hlf(parse_reg(parts[1])),
                "tpl" => Instr::Tpl(parse_reg(parts[1])),
                "inc" => Instr::Inc(parse_reg(parts[1])),
                "jmp" => Instr::Jmp(parts[1].parse().unwrap()),
                "jie" => {
                    let r = parse_reg(parts[1].trim_end_matches(','));
                    let off = parts[2].parse().unwrap();
                    Instr::Jie(r, off)
                }
                "jio" => {
                    let r = parse_reg(parts[1].trim_end_matches(','));
                    let off = parts[2].parse().unwrap();
                    Instr::Jio(r, off)
                }
                _ => unreachable!(),
            }
        })
        .collect()
}

fn parse_reg(s: &str) -> Reg {
    match s {
        "a" => Reg::A,
        "b" => Reg::B,
        _ => unreachable!(),
    }
}

#[inline]
fn reg_mut<'a>(r: Reg, a: &'a mut u64, b: &'a mut u64) -> &'a mut u64 {
    match r {
        Reg::A => a,
        Reg::B => b,
    }
}

fn run(program: &[Instr], mut a: u64, mut b: u64) -> u64 {
    let mut ip: isize = 0;

    while let Some(instr) = program.get(ip as usize) {
        match *instr {
            Instr::Hlf(r) => {
                *reg_mut(r, &mut a, &mut b) /= 2;
                ip += 1;
            }
            Instr::Tpl(r) => {
                *reg_mut(r, &mut a, &mut b) *= 3;
                ip += 1;
            }
            Instr::Inc(r) => {
                *reg_mut(r, &mut a, &mut b) += 1;
                ip += 1;
            }
            Instr::Jmp(off) => {
                ip += off;
            }
            Instr::Jie(r, off) => {
                if *reg_mut(r, &mut a, &mut b) % 2 == 0 {
                    ip += off;
                } else {
                    ip += 1;
                }
            }
            Instr::Jio(r, off) => {
                if *reg_mut(r, &mut a, &mut b) == 1 {
                    ip += off;
                } else {
                    ip += 1;
                }
            }
        }
    }

    b
}

pub fn solve(input: &str) -> (String, String) {
    let program = parse(input);

    let part1 = run(&program, 0, 0);
    let part2 = run(&program, 1, 0);

    (part1.to_string(), part2.to_string())
}
