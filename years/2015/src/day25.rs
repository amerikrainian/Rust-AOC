fn parse(input: &str) -> (u64, u64) {
    let input = input.trim();

    let row_start = input.find("row ").unwrap() + 4;
    let comma = input[row_start..].find(',').unwrap() + row_start;
    let row: u64 = input[row_start..comma].parse().unwrap();

    let col_start = input.find("column ").unwrap() + 7;
    let dot = input[col_start..].find('.').unwrap() + col_start;
    let col: u64 = input[col_start..dot].parse().unwrap();

    (row, col)
}

fn index(row: u64, col: u64) -> u64 {
    let d = row + col - 1;
    (d - 1) * d / 2 + col
}

fn mod_pow(mut base: u64, mut exp: u64, modu: u64) -> u64 {
    let mut result = 1;
    base %= modu;

    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % modu;
        }
        base = base * base % modu;
        exp >>= 1;
    }

    result
}

pub fn solve(input: &str) -> (String, String) {
    let (row, col) = parse(input);

    let n = index(row, col);
    let value = 20151125u64 * mod_pow(252533, n - 1, 33554393) % 33554393;

    (value.to_string(), "".to_string())
}
