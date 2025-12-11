use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let years_dir = workspace_root.join("years");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("years.rs");

    let mut years = Vec::new();

    if let Ok(entries) = fs::read_dir(&years_dir) {
        for entry in entries.flatten() {
            if let Ok(file_name) = entry.file_name().into_string() {
                if let Ok(year) = file_name.parse::<u32>() {
                    years.push(year);
                }
            }
        }
    }

    years.sort();

    let mut output = String::new();

    output
        .push_str("pub fn run_solution(year: u32, day: u32, input: &str) -> (String, String) {\n");
    output.push_str("    match year {\n");

    for year in &years {
        output.push_str(&format!(
            "        {} => aoc{}::run(day, input),\n",
            year, year
        ));
    }

    output.push_str("        _ => panic!(\"Year not implemented\"),\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");

    output.push_str("pub fn get_available_years_numeric() -> Vec<u32> {\n");
    output.push_str("    vec![");
    for (i, year) in years.iter().enumerate() {
        if i > 0 {
            output.push_str(", ");
        }
        output.push_str(&year.to_string());
    }
    output.push_str("]\n");
    output.push_str("}\n");

    fs::write(dest_path, output).unwrap();

    println!("cargo:rerun-if-changed=../years");
}
