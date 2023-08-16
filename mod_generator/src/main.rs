use walkdir::WalkDir;
use std::fs;
use std::path::Path;

fn generate_mod_files(directory: &str) {
    let mut modules = vec![];

    for entry in WalkDir::new(directory).min_depth(1).max_depth(1).into_iter() {
        let entry = entry.unwrap();
        if entry.file_type().is_dir() {
            let module_name = entry.file_name().to_string_lossy();
            modules.push(module_name.to_string());

            // Recurse into subdirectory
            generate_mod_files(&entry.path().to_string_lossy());
        } else if entry.path().extension() == Some(Path::new("rs").as_os_str()) {
            let module_name = entry.path().file_stem().unwrap().to_string_lossy();
            if module_name != "mod" && module_name != "main" && module_name != "lib" {
                modules.push(module_name.to_string());
            }
        }
    }

    if !modules.is_empty() {
        let mod_contents: Vec<String> = modules.iter().map(|m| format!("pub mod {};", m)).collect();
        fs::write(
            format!("{}/mod.rs", directory),
            mod_contents.join("\n")
        ).unwrap();
    }
}

fn main() {
    let dir = "./src";  // Start from the src directory
    generate_mod_files(dir);
}
