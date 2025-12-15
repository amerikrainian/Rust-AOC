use itoa;
use md5::{Digest, Md5};

pub fn solve(input: &str) -> (String, String) {
    let door_id = input.trim();
    (solve_part1(door_id), solve_part2(door_id))
}

fn solve_part1(door_id: &str) -> String {
    let mut password = String::with_capacity(8);

    let mut base = Md5::new();
    base.update(door_id.as_bytes());
    let base_state = base.clone();

    let mut itoa_buf = itoa::Buffer::new();

    for i in 0u64.. {
        let mut h = base_state.clone();
        h.update(itoa_buf.format(i).as_bytes());
        let digest = h.finalize();

        if digest[0] == 0 && digest[1] == 0 && (digest[2] >> 4) == 0 {
            let nibble = digest[2] & 0x0F;
            password.push(nibble_to_hex(nibble));

            if password.len() == 8 {
                return password;
            }
        }
    }

    unreachable!()
}

fn solve_part2(door_id: &str) -> String {
    let mut password = [None; 8];
    let mut filled = 0;

    let mut base = Md5::new();
    base.update(door_id.as_bytes());
    let base_state = base.clone();

    let mut itoa_buf = itoa::Buffer::new();

    for i in 0u64.. {
        let mut h = base_state.clone();
        h.update(itoa_buf.format(i).as_bytes());
        let digest = h.finalize();

        if digest[0] == 0 && digest[1] == 0 && (digest[2] >> 4) == 0 {
            let pos = (digest[2] & 0x0F) as usize;
            if pos < 8 && password[pos].is_none() {
                let value = digest[3] >> 4;
                password[pos] = Some(nibble_to_hex(value));
                filled += 1;

                if filled == 8 {
                    return password.iter().map(|c| c.unwrap()).collect();
                }
            }
        }
    }

    unreachable!()
}

#[inline]
fn nibble_to_hex(n: u8) -> char {
    match n {
        0..=9 => (b'0' + n) as char,
        10..=15 => (b'a' + (n - 10)) as char,
        _ => unreachable!(),
    }
}
