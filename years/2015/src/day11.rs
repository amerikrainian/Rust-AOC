use std::collections::HashSet;

fn next_pass(password: &str) -> String {
    let mut chars: Vec<char> = password.chars().collect();
    let mut i = chars.len() - 1;

    loop {
        if chars[i] == 'z' {
            chars[i] = 'a';
            if i == 0 {
                break;
            }
            i -= 1;
        } else {
            chars[i] = ((chars[i] as u8) + 1) as char;
            break;
        }
    }

    chars.into_iter().collect()
}

fn has_straight(p: &str) -> bool {
    let bytes = p.as_bytes();
    for w in bytes.windows(3) {
        if w[0] + 1 == w[1] && w[1] + 1 == w[2] {
            return true;
        }
    }
    false
}

fn has_no_invalid_chars(p: &str) -> bool {
    !p.contains('i') && !p.contains('o') && !p.contains('l')
}

fn has_two_pairs(p: &str) -> bool {
    let mut found = HashSet::new();
    let chars: Vec<char> = p.chars().collect();

    let mut i = 0;
    while i + 1 < chars.len() {
        if chars[i] == chars[i + 1] {
            found.insert(chars[i]);
            i += 2;
        } else {
            i += 1;
        }
    }

    found.len() >= 2
}

fn valid(p: &str) -> bool {
    has_straight(p) && has_no_invalid_chars(p) && has_two_pairs(p)
}

fn find_next_valid(mut p: String) -> String {
    loop {
        p = next_pass(&p);
        if valid(&p) {
            return p;
        }
    }
}

pub fn solve(input: &str) -> (String, String) {
    let first = find_next_valid(input.to_string());
    let second = find_next_valid(first.clone());
    (first, second)
}
