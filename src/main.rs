use std::io::{self, Write};
use colored::*;  // Add this line to use the colored crate

fn main() {
    let mut passwd = String::new();
    println!("Please enter the password:");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed immediately
    io::stdin().read_line(&mut passwd).expect("Failed to read line");

    // Trim the newline character from the end
    let passwd = passwd.trim();

    // Pass by reference to avoid moving the ownership
    check_password(passwd);
}

// Use &str instead of String to avoid moving ownership
fn check_password(passwd: &str) {
    if passwd.len() > 12 {
        println!("{}", format!("Correct length: {}", passwd.len()).bright_green());
    } else {
        println!("{}", format!("Too short!!!! Length: {}", passwd.len()).red());
    }

    if contains_number(passwd) {
        println!("{}", "Password contains numbers.".bright_green());
    } else {
        println!("{}", "Password does not contain any numbers.".red());
    }
}

// Function to check if the string contains any numbers
fn contains_number(s: &str) -> bool {
    s.chars().any(|c| c.is_numeric())
}
