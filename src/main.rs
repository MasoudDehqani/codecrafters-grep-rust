use std::env;
use std::io;
use std::process;

fn match_digits_character_class(input_line: &str) -> bool {
    match input_line.parse::<i32>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn match_words_character_class(input_line: &str) -> bool {
    input_line
        .chars()
        .any(|ch| ch.is_alphanumeric() || ch == '_')
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    println!("{}", input_line);
    match pattern {
        "\\d" => match_digits_character_class(input_line),
        "\\w" => match_words_character_class(input_line),
        _ => input_line.contains(pattern),
    }
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
