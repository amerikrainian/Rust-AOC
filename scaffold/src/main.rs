use clap::Parser;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use toml_edit::{DocumentMut, Item, Value};

#[derive(Parser)]
#[command(name = "scaffold")]
#[command(about = "Generate Advent of Code year scaffolding", long_about = None)]
struct Args {
    year: u32,
}

fn main() {
    let args = Args::parse();
    let year = args.year;

    let workspace_root = get_workspace_root();
    let year_path = workspace_root.join("years").join(year.to_string());

    if year_path.exists() {
        eprintln!(
            "Error: Year {} already exists at {}",
            year,
            year_path.display()
        );
        std::process::exit(1);
    }

    create_year_structure(&year_path, year);
    update_workspace_cargo(&workspace_root, year);
    update_runner_cargo(&workspace_root, year);

    println!("Successfully created scaffolding for year {}", year);
    println!("Run 'cargo build' to compile the new year crate.");
    println!("Note: The runner will automatically detect and include this year.");
}

fn get_workspace_root() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path
}

fn create_year_structure(year_path: &Path, year: u32) {
    fs::create_dir_all(year_path.join("src")).expect("Failed to create year directories");

    let cargo_toml = format!(
        r#"[package]
name = "aoc{}"
version = "0.1.0"
edition.workspace = true

[dependencies]

[build-dependencies]
"#,
        year
    );
    fs::write(year_path.join("Cargo.toml"), cargo_toml).expect("Failed to write Cargo.toml");

    let build_rs = r##"use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let src_dir = Path::new("src");
    let out_dir = env::var("OUT_DIR").unwrap();
    let days_path = Path::new(&out_dir).join("days.rs");
    let mods_path = Path::new(&out_dir).join("mods.rs");

    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut days = Vec::new();

    if let Ok(entries) = fs::read_dir(src_dir) {
        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            if file_name_str.starts_with("day") && file_name_str.ends_with(".rs") {
                if let Some(day_str) = file_name_str.strip_prefix("day").and_then(|s| s.strip_suffix(".rs")) {
                    if let Ok(day_num) = day_str.parse::<u32>() {
                        days.push((day_num, file_name_str.to_string()));
                    }
                }
            }
        }
    }

    days.sort_by_key(|(num, _)| *num);

    let mut mods_output = String::new();
    for (_, file_name) in &days {
        let mod_name = file_name.strip_suffix(".rs").unwrap();
        let src_path = Path::new(&cargo_manifest_dir).join("src").join(file_name);
        mods_output.push_str(&format!("#[path = \"{}\"]\n", src_path.display()));
        mods_output.push_str(&format!("pub mod {};\n", mod_name));
    }
    fs::write(mods_path, mods_output).unwrap();

    let mut days_output = String::new();
    days_output.push_str("pub fn run(day: u32, input: &str) -> (String, String) {\n");
    days_output.push_str("    match day {\n");

    for (day_num, file_name) in &days {
        let mod_name = file_name.strip_suffix(".rs").unwrap();
        days_output.push_str(&format!("        {} => crate::{}::solve(input),\n", day_num, mod_name));
    }

    days_output.push_str("        _ => panic!(\"Day {} not implemented\", day),\n");
    days_output.push_str("    }\n");
    days_output.push_str("}\n");

    fs::write(days_path, days_output).unwrap();

    println!("cargo:rerun-if-changed=src/");
}
"##;
    fs::write(year_path.join("build.rs"), build_rs).expect("Failed to write build.rs");

    let lib_rs = r#"include!(concat!(env!("OUT_DIR"), "/mods.rs"));
include!(concat!(env!("OUT_DIR"), "/days.rs"));
"#;
    fs::write(year_path.join("src/lib.rs"), lib_rs).expect("Failed to write lib.rs");

    println!("Created year structure at {}", year_path.display());
}

fn update_workspace_cargo(workspace_root: &Path, year: u32) {
    let cargo_path = workspace_root.join("Cargo.toml");
    let content = fs::read_to_string(&cargo_path).expect("Failed to read workspace Cargo.toml");

    let mut doc = content
        .parse::<DocumentMut>()
        .expect("Failed to parse workspace Cargo.toml");

    let members = doc["workspace"]["members"]
        .as_array_mut()
        .expect("workspace.members is not an array");

    let year_member = format!("years/{}", year);

    if members.iter().any(|m| m.as_str() == Some(&year_member)) {
        return;
    }

    let mut value = toml_edit::Value::from(year_member);
    value.decor_mut().set_prefix("\n    ");

    members.push_formatted(value);
    members.set_trailing("\n");
    members.set_trailing_comma(true);

    fs::write(&cargo_path, doc.to_string()).expect("Failed to write workspace Cargo.toml");
    println!("Updated workspace Cargo.toml");
}

fn update_runner_cargo(workspace_root: &Path, year: u32) {
    let cargo_path = workspace_root.join("runner/Cargo.toml");
    let content = fs::read_to_string(&cargo_path).expect("Failed to read runner Cargo.toml");

    let mut doc = content
        .parse::<DocumentMut>()
        .expect("Failed to parse runner Cargo.toml");

    let dependencies = doc["dependencies"]
        .as_table_mut()
        .expect("dependencies is not a table");

    let dep_name = format!("aoc{}", year);

    if dependencies.contains_key(&dep_name) {
        return;
    }

    let mut dep_table = toml_edit::InlineTable::new();
    dep_table.insert("path", Value::from(format!("../years/{}", year)));

    dependencies.insert(&dep_name, Item::Value(Value::InlineTable(dep_table)));

    fs::write(&cargo_path, doc.to_string()).expect("Failed to write runner Cargo.toml");
    println!("Updated runner Cargo.toml");
}
