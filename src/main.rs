use std::env;
use std::fs;
use std::process;
use tomli_rust;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: tomli-rust <path/to/file.toml>");
        process::exit(1);
    }
    
    let file_path = &args[1];
    
    let contents = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            process::exit(1);
        }
    };
    
    println!("Parsing {}...\n", file_path);
    
    match tomli_rust::parse(&contents) {
        Ok(table) => {
            println!("✅ Successfully parsed TOML!\n");
            println!("{:#?}", table);
        }
        Err(e) => {
            eprintln!("❌ Parse Error: {}", e);
            process::exit(1);
        }
    }
}
