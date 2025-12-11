use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(name = "aoc-runner")]
#[command(about = "Advent of Code runner", long_about = None)]
struct Args {
    first: Option<u32>,
    second: Option<u32>,
}

include!(concat!(env!("OUT_DIR"), "/years.rs"));

fn main() {
    let args = Args::parse();

    let (year, day) = match (args.first, args.second) {
        (Some(first), Some(second)) => (first, second),
        (Some(first), None) => {
            let latest_year = match find_latest_year() {
                Some(y) => y,
                None => {
                    eprintln!("Error: No years found in years/ directory");
                    std::process::exit(1);
                }
            };
            (latest_year, first)
        }
        (None, _) => {
            let latest_year = match find_latest_year() {
                Some(y) => y,
                None => {
                    eprintln!("Error: No years found in years/ directory");
                    std::process::exit(1);
                }
            };
            let latest_day = match find_latest_day(latest_year) {
                Some(d) => d,
                None => {
                    eprintln!("Error: No days found for year {}", latest_year);
                    std::process::exit(1);
                }
            };
            (latest_year, latest_day)
        }
    };

    if !is_year_implemented(year) {
        eprintln!("Error: Year {} is not implemented", year);
        eprintln!("Available years: {}", get_available_years().join(", "));
        std::process::exit(1);
    }

    let input = match std::panic::catch_unwind(|| util::load_input(year, day)) {
        Ok(input) => input,
        Err(_) => {
            eprintln!(
                "Error: Could not load input file for year {} day {}",
                year, day
            );
            eprintln!("Expected location: inputs/{}/{:02}.in", year, day);
            std::process::exit(1);
        }
    };

    let (part1, part2) = match std::panic::catch_unwind(|| run_solution(year, day, &input)) {
        Ok(result) => result,
        Err(_) => {
            eprintln!("Error: Day {} is not implemented for year {}", day, year);
            std::process::exit(1);
        }
    };

    println!("{}\n{}", part1, part2);
}

fn is_year_implemented(year: u32) -> bool {
    get_available_years_numeric().contains(&year)
}

fn get_available_years() -> Vec<String> {
    get_available_years_numeric()
        .iter()
        .map(|y| y.to_string())
        .collect()
}

fn find_latest_year() -> Option<u32> {
    let years_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("years");

    if let Ok(entries) = fs::read_dir(years_dir) {
        let mut years: Vec<u32> = entries
            .filter_map(|entry| {
                entry
                    .ok()
                    .and_then(|e| e.file_name().to_string_lossy().parse::<u32>().ok())
            })
            .collect();

        years.sort();
        years.last().copied()
    } else {
        None
    }
}

fn find_latest_day(year: u32) -> Option<u32> {
    let year_src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("years")
        .join(year.to_string())
        .join("src");

    if let Ok(entries) = fs::read_dir(year_src_dir) {
        let mut days: Vec<u32> = entries
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let file_name = e.file_name().to_string_lossy().to_string();
                    if file_name.starts_with("day") && file_name.ends_with(".rs") {
                        file_name
                            .strip_prefix("day")
                            .and_then(|s| s.strip_suffix(".rs"))
                            .and_then(|s| s.parse::<u32>().ok())
                    } else {
                        None
                    }
                })
            })
            .collect();

        days.sort();
        days.last().copied()
    } else {
        None
    }
}
