use std::fs;

fn print_directory_tree(path: &str, indentation: String) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                let entry_name = entry_path.file_name().unwrap().to_string_lossy().into_owned();

                println!("{}{}", indentation, entry_name);

                if entry_path.is_dir() {
                    let new_indentation = format!("{}  |", indentation);
                    print_directory_tree(entry_path.to_str().unwrap(), new_indentation);
                }
            }
        }
    }
}

fn main() {
    let root_directory = ".";

    println!("{}", root_directory);
    print_directory_tree(root_directory, String::from("|-- "));
}
