fn part1(target: usize) -> usize {
    let limit = target / 10;
    let mut houses = vec![0usize; limit + 1];

    for elf in 1..=limit {
        for house in (elf..=limit).step_by(elf) {
            houses[house] += elf * 10;
        }
    }

    houses
        .iter()
        .enumerate()
        .skip(1)
        .find(|&(_, &p)| p >= target)
        .unwrap()
        .0
}

fn part2(target: usize) -> usize {
    let limit = target / 11;
    let mut houses = vec![0usize; limit + 1];

    for elf in 1..=limit {
        let max_house = (elf * 50).min(limit);
        for house in (elf..=max_house).step_by(elf) {
            houses[house] += elf * 11;
        }
    }

    houses
        .iter()
        .enumerate()
        .skip(1)
        .find(|&(_, &p)| p >= target)
        .unwrap()
        .0
}

pub fn solve(input: &str) -> (String, String) {
    let target: usize = input.trim().parse().unwrap();

    let part1 = part1(target);
    let part2 = part2(target);

    (part1.to_string(), part2.to_string())
}
