use std::io::{self, Write};
use colored::*;  
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use strip_ansi_escapes::strip;

fn main() {
    let mut passwd = String::new();
    println!("Please enter the password:");
    io::stdout().flush().unwrap(); // Ensure the prompt is printed immediately
    io::stdin().read_line(&mut passwd).expect("Failed to read line");

    // Trim the newline character from the end
    let passwd = passwd.trim();

    // Pass by reference to avoid moving the ownership
    let numbers = check_password(passwd);
    print_vote(passwd, numbers);
}

// Use &str instead of String to avoid moving ownership
fn check_password(passwd: &str) -> &str {
    if passwd.len() > 12 {
        println!("{}", format!("Correct length: {}", passwd.len()).bright_green());
    } else {
        println!("{}", format!("Too short!!!! Length: {}", passwd.len()).red());
    }

    if contains_number(passwd) {
        println!("{}", "Password contains numbers.".bright_green());
        "YES"
    } else {
        println!("{}", "Password does not contain any numbers.".red());
        "NO"
    }
}

fn print_vote(passwd: &str, numbers: &str) {
    let len_colored = if passwd.len() > 12 {
        format!("{}", passwd.len()).bright_green()
    } else {
        format!("{}", passwd.len()).red()
    };

    let len_stripped = strip(len_colored.to_string()).unwrap();
    let len_str = String::from_utf8(len_stripped).unwrap();

    let numbers_colored = if numbers == "YES" {
        "YES".bright_green()
    } else {
        "NO".red()
    };

    let numbers_stripped = strip(numbers_colored.to_string()).unwrap();
    let numbers_str = String::from_utf8(numbers_stripped).unwrap();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(40)
        .set_header(vec!["CHECKS", "OCCURRENCES", "VOTE"])
        .add_row(vec![
            Cell::new("LETTERS").set_alignment(CellAlignment::Center),
            Cell::new(len_str).set_alignment(CellAlignment::Center),
            Cell::new("Password length check").set_alignment(CellAlignment::Center),
        ])
        .add_row(vec![
            Cell::new("NUMBERS").set_alignment(CellAlignment::Center),
            Cell::new(numbers_str).set_alignment(CellAlignment::Center),
            Cell::new("Contains number check").set_alignment(CellAlignment::Center),
        ]);

    // Set the default alignment for the third column to right
    let column = table.column_mut(2).expect("Our table has three columns");
    column.set_cell_alignment(CellAlignment::Right);

    println!("{table}");
}

// Function to check if the string contains any numbers
fn contains_number(s: &str) -> bool {
    s.chars().any(|c| c.is_numeric())
}
