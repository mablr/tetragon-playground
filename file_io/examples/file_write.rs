use std::env;
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if a file path was provided
    if args.len() < 2 {
        eprintln!("Usage: cargo run --example file_write -- <file_path>");
        std::process::exit(1);
    }
    
    let file_path = &args[1];
    println!("Writing to file: {}", file_path);
    
    // Create or open the file for writing
    let mut file = File::create(file_path)?;
    
    // Content to write
    let content = "Hello, this is some text that will be written to the file.\n\
                  This is the second line of text.";
    
    // Write content to the file
    file.write_all(content.as_bytes())?;
    
    println!("Successfully wrote to {}", file_path);
    
    Ok(())
} 