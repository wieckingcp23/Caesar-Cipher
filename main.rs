use std::io::{self, Write};

use caesar_cipher::{CaesarCipher, Cipher};

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Caesar Cipher");
    println!("Choose mode: [e]ncrypt or [d]ecrypt");
    print!("> ");
    io::stdout().flush()?;

    let mut mode = String::new();
    io::stdin().read_line(&mut mode)?;
    let mode = mode.trim().to_ascii_lowercase();

    if mode != "e" && mode != "encrypt" && mode != "d" && mode != "decrypt" {
        return Err("mode must be e/encrypt or d/decrypt".into());
    }

    println!("Enter text:");
    let mut text = String::new();
    io::stdin().read_line(&mut text)?;
    let text = text.trim_end_matches(['\r', '\n']);

    println!("Enter key (1-26):");
    print!("> ");
    io::stdout().flush()?;

    let mut key_input = String::new();
    io::stdin().read_line(&mut key_input)?;
    let key: u8 = key_input.trim().parse()?;
    let cipher = CaesarCipher::new(key)?;

    let result = match mode.as_str() {
        "e" | "encrypt" => cipher.encrypt(text),
        "d" | "decrypt" => cipher.decrypt(text),
        _ => unreachable!("mode is validated above"),
    };

    println!("Result: {result}");

    Ok(())
}
