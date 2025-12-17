use std::{env, fs, io};
use sha2::{Digest, Sha256};
use base64::{Engine as _, engine::general_purpose};

fn main() -> Result<(), io::Error> {
    // 1. Get the file path from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run --bin generate_hash <PATH_TO_FILE>");
        eprintln!("Example: cargo run --bin generate_hash static/assets/index-1234.js");
        return Ok(());
    }
    
    let file_path = &args[1];

    // 2. Read the file contents
    let file_contents = fs::read(file_path)?;

    // 3. Compute the SHA-256 hash
    let mut hasher = Sha256::new();
    hasher.update(&file_contents);
    let hash_bytes = hasher.finalize();

    // 4. Encode the hash to Base64 (Standard Alphabet)
    // The format required for CSP is: sha256-BASE64_ENCODED_HASH
    let encoded_hash = general_purpose::STANDARD.encode(hash_bytes);
    
    let csp_hash_string = format!("sha256-{}", encoded_hash);

    // 5. Print the result for use in main.rs
    println!("File: {}", file_path);
    println!("CSP Hash (to use in main.rs): {}", csp_hash_string);
    
    Ok(())
}