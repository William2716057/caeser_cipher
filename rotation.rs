use std::io::{self, Write};

fn caesar(text: &str, shift: i32) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                let base = b'A';
                let pos = c as u8 - base;
                let new_pos = (pos as i32 + shift).rem_euclid(26);
                (base + new_pos as u8) as char
            } else if c.is_ascii_lowercase() {
                let base = b'a';
                let pos = c as u8 - base;
                let new_pos = (pos as i32 + shift).rem_euclid(26);
                (base + new_pos as u8) as char
            } else {
                
            }
        })
        .collect()
}

fn main() {
    let mut text = String::new();
    let mut shift = String::new();

    print!("Enter text: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut text).unwrap();

    print!("Enter shift: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut shift).unwrap();

    let shift: i32 = shift.trim().parse().unwrap();

    let encrypted = caesar(text.trim(), shift);

    println!("Result: {}", encrypted);
}
