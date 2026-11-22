use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::fs::File;

fn get_resource_pack(path: &Path) -> String {
    let options_path = path.join("options.txt");
    let file = File::open(options_path).expect("Couldn't open options.txt");
    let reader = BufReader::new(file);

    for line in reader.lines().flatten() {
        if line.starts_with("resourcePacks:") {
            return line;
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
        
        let pack = get_resource_pack(&path);
        println!("Resource Pack: {}", pack);
    } else {
        println!("Path not found! ({})", path.display());
    }
}
