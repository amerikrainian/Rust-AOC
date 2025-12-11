pub fn solve(input: &str) -> (String, String) {
    let input = parse(input);
    (part1(&input), part2(&input))
}

fn parse(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .lines()
        .map(|l| {
            let nums: Vec<usize> = l.split("x").map(|i| i.parse::<usize>().unwrap()).collect();
            (nums[0], nums[1], nums[2])
        })
        .collect()
}

fn part1(input: &[(usize, usize, usize)]) -> String {
    input
        .iter()
        .map(|(l, w, h)| {
            let side1 = l * w;
            let side2 = w * h;
            let side3 = h * l;
            2 * side1 + 2 * side2 + 2 * side3 + side1.min(side2).min(side3)
        })
        .sum::<usize>()
        .to_string()
}

fn part2(input: &[(usize, usize, usize)]) -> String {
    input
        .iter()
        .map(|(l, w, h)| {
            let mut sides = vec![*l, *w, *h];
            sides.sort_unstable();
            let wrap = 2 * sides[0] + 2 * sides[1];
            let bow = l * w * h;
            wrap + bow
        })
        .sum::<usize>()
        .to_string()
}
