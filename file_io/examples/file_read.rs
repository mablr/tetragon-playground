use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

fn main() -> io::Result<()> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if a file path was provided
    if args.len() < 2 {
        eprintln!("Usage: cargo run --example file_read -- <file_path>");
        std::process::exit(1);
    }
    
    let file_path = &args[1];
    println!("Reading file: {}", file_path);
    
    // Check if the file exists
    if !Path::new(file_path).exists() {
        eprintln!("Error: File '{}' does not exist", file_path);
        std::process::exit(1);
    }
    
    // Open the file
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    
    // Read the file content
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    
    // Print the content
    println!("File content:");
    println!("{}", content);
    
    Ok(())
} 