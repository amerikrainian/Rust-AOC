use std::fs;
use std::path::PathBuf;

pub fn load_input(year: u32, day: u32) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // go up from util/ to workspace root
    path.push("inputs");
    path.push(year.to_string());
    path.push(format!("{:02}.in", day));

    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", path.display()))
}
