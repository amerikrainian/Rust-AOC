pub fn solve(input: &str) -> (String, String) {
    let width = input.lines().next().unwrap().len();
    let mut counts = vec![[0usize; 26]; width];

    for line in input.lines() {
        for (i, b) in line.bytes().enumerate() {
            counts[i][(b - b'a') as usize] += 1;
        }
    }

    let p1: String = counts
        .iter()
        .map(|col| {
            (0..26)
                .max_by_key(|&i| col[i])
                .map(|i| (b'a' + i as u8) as char)
                .unwrap()
        })
        .collect();

    let p2: String = counts
        .iter()
        .map(|col| {
            (0..26)
                .min_by_key(|&i| col[i])
                .filter(|&i| col[i] > 0)
                .map(|i| (b'a' + i as u8) as char)
                .unwrap()
        })
        .collect();

    (p1, p2)
}
