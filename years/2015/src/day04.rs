use md5::{Digest, Md5};
use itoa;

pub fn solve(input: &str) -> (String, String) {
    let trimmed = input.trim();
    (
        find_with_leading_zeros(trimmed, 5).to_string(),
        find_with_leading_zeros(trimmed, 6).to_string(),
    )
}

fn find_with_leading_zeros(input: &str, zeros: usize) -> u64 {
    // How many full zero bytes and leftover bits?
    let full_zero_bytes = zeros / 2;
    let leftover_bits   = (zeros & 1) * 4;

    let input_bytes = input.as_bytes();

    let mut base = Md5::new();
    base.update(input_bytes);
    let base_state = base.clone();

    let mut itoa_buf = itoa::Buffer::new();

    for i in 0u64.. {
        let mut h = base_state.clone();

        let digits = itoa_buf.format(i);
        h.update(digits.as_bytes());

        let digest = h.finalize();

        let mut ok = true;
        for &b in &digest[..full_zero_bytes] {
            if b != 0 {
                ok = false;
                break;
            }
        }
        if !ok { continue; }

        if leftover_bits > 0 {
            if (digest[full_zero_bytes] >> 4) != 0 {
                continue;
            }
        }

        return i;
    }

    unreachable!()
}
