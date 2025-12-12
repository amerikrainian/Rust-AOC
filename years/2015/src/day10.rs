fn simulate(input: &str, steps: usize) -> usize {
    let mut current = input.to_string();
    for _ in 0..steps {
        let mut next = String::new();
        let mut chars = current.chars().peekable();
        while let Some(c) = chars.next() {
            let mut count = 1;
            while let Some(&next_c) = chars.peek() {
                if next_c == c {
                    count += 1;
                    chars.next();
                } else {
                    break;
                }
            }
            next.push_str(&count.to_string());
            next.push(c);
        }
        current = next;
    }
    current.len()
}

pub fn solve(input: &str) -> (String, String) {
    (
        simulate(input, 40).to_string(),
        simulate(input, 50).to_string(),
    )
}
