use std::str::FromStr;
use std::{env, fs};
use toml::Table;

fn main() {
    let filenames: Vec<String> = env::args().collect();
    for filename in &filenames[1..] {
        let content = match fs::read_to_string(filename) {
            Ok(s) => s,
            Err(e) => {
                println!("Error reading file {filename} => {e}");
                continue;
            }
        };
        let toml_content = match Table::from_str(&*content) {
            Ok(t) => t,
            Err(e) => {
                println!("Error parsing file {filename} => {e}");
                continue;
            }
        };
        let new_content = toml::to_string_pretty(&toml_content).unwrap_or(String::from(""));
        match fs::write(filename, new_content) {
            Ok(_) => {
                println!("Successfully written file {filename}.");
            }
            Err(e) => {
                println!("Error writing to {filename} => {e}");
                continue;
            }
        };
    }
}
