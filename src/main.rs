use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::fs::File;

fn get_username(path: &Path) -> String {
    let log_path = path.join("logs").join("latest.log");
    let file = File::open(log_path).expect("Couldn't open latest.log");
    let mut reader = BufReader::new(file);

    let mut first_line = String::new();
    reader
        .read_line(&mut first_line)
        .expect("Couldn't read the first line");

    first_line
        .split_once("Setting user: ")
        .map(|(_, user)| user.trim().to_string())
        .unwrap_or_else(|| String::from("Unknown"))
}

fn get_resource_pack(path: &Path) -> String {
    let options_path = path.join("options.txt");
    let file = File::open(options_path).expect("Couldn't open options.txt");
    let reader = BufReader::new(file);

    for line in reader.lines().flatten() {
        if let Some(raw) = line.strip_prefix("resourcePacks:") {
            let cleaned = raw
                .trim()
                .trim_start_matches('[')
                .trim_end_matches(']')
                .trim_matches('"');

            if !cleaned.is_empty() {
                return cleaned.to_string();
            }
        }
    }

    String::from("Default")
}

fn main() {
    println!("Enter a Minecraft path: ");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let path = PathBuf::from(input.trim());

    if path.exists() {
        println!("Path Found: {}", path.display());

        clearscreen::clear().expect("failed to clear screen");
        
        let username = get_username(&path);
        println!("Username: {}", username);

        let pack = get_resource_pack(&path);
        println!("Resource Pack(s): {}", pack);

    } else {
        println!("Path not found! ({})", path.display());
    }
}
