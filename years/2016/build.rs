use std::env;
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
                if let Some(day_str) = file_name_str
                    .strip_prefix("day")
                    .and_then(|s| s.strip_suffix(".rs"))
                {
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
        days_output.push_str(&format!(
            "        {} => crate::{}::solve(input),\n",
            day_num, mod_name
        ));
    }

    days_output.push_str("        _ => panic!(\"Day {} not implemented\", day),\n");
    days_output.push_str("    }\n");
    days_output.push_str("}\n");

    fs::write(days_path, days_output).unwrap();

    println!("cargo:rerun-if-changed=src/");
}
