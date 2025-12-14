use std::collections::HashMap;

use aoc_utils::regex::Regex;

#[derive(Debug, Clone)]
struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
}

fn parse(input: &str) -> Vec<Room> {
    let re = Regex::new(r"^([a-z-]+)-(\d+)\[([a-z]{5})\]$").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Room {
                name: caps.get(1).unwrap().as_str().to_string(),
                sector_id: caps.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                checksum: caps.get(3).unwrap().as_str().to_string(),
            }
        })
        .collect()
}

fn is_real(room: &Room) -> bool {
    let mut freq: HashMap<char, u32> = HashMap::new();

    for c in room.name.chars().filter(|&c| c != '-') {
        *freq.entry(c).or_insert(0) += 1;
    }

    let mut letters: Vec<(char, u32)> = freq.into_iter().collect();

    letters.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    let expected: String = letters.iter().take(5).map(|(c, _)| *c).collect();

    expected == room.checksum
}

fn decrypt_name(room: &Room) -> String {
    let shift = (room.sector_id % 26) as u8;

    room.name
        .chars()
        .map(|c| match c {
            '-' => ' ',
            'a'..='z' => {
                let base = b'a';
                let rotated = (c as u8 - base + shift) % 26 + base;
                rotated as char
            }
            _ => c,
        })
        .collect()
}

pub fn solve(input: &str) -> (String, String) {
    let rooms = parse(input);

    let p1: u32 = rooms
        .iter()
        .filter(|room| is_real(room))
        .map(|room| room.sector_id)
        .sum();

    let p2: u32 = rooms
        .iter()
        .filter(|room| is_real(room))
        .find_map(|room| {
            let decrypted = decrypt_name(room);
            if decrypted.contains("pole") {
                Some(room.sector_id)
            } else {
                None
            }
        })
        .expect("north pole room not found");

    (p1.to_string(), p2.to_string())
}
