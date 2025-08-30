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

fn match_positive_character_group(input_line: &str, p: &str) -> bool {
    p.chars().any(|ch| input_line.contains(ch))
}

fn match_negative_character_group(input_line: &str, p: &str) -> bool {
    p.chars().any(|ch| !input_line.contains(ch))
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let mut patterns: Vec<String> = Vec::new();

    let mut is_adding_character_class = false;
    for ch in pattern.chars() {
        if ch == '\\' {
            is_adding_character_class = true;
            continue;
        }

        if is_adding_character_class {
            patterns.push(String::from("\\") + &ch.to_string());
            is_adding_character_class = false;
        } else {
            patterns.push(ch.clone().to_string());
        }
    }

    input_line.char_indices().fold(false, |_acc, (i, _ch)| {
        let slice = &input_line[i..input_line.len()];

        let res = patterns.iter().enumerate().fold(false, |_acc, (i, curr)| {
            let g = slice.chars().collect::<Vec<char>>();
            let maybe_proportional_char = g.get(i);
            let pp = maybe_proportional_char.unwrap_or(&' ');
            println!("{}, {}", pp, curr);
            match maybe_proportional_char {
                Some(ch) => match curr.as_str() {
                    "\\d" => {
                        // println!("IS DIGIT");
                        ch.is_digit(10)
                    }
                    "\\w" => {
                        // println!("IS ALPHANUMERIC");
                        ch.is_alphanumeric()
                    }
                    // p if p.starts_with("[^") && p.ends_with("]") => {
                    //     match_negative_character_group(input_line, p.trim_matches(&['[', '^', ']']))
                    // }
                    // p if p.starts_with("[") && p.ends_with("]") => {
                    //     match_positive_character_group(input_line, p.trim_matches(&['[', ']']))
                    // }
                    _ => {
                        // println!("{}", ch.to_string());
                        // println!("{}", *curr);
                        let res = ch.to_string() == *curr;
                        println!("{}", res);
                        res
                    }
                },
                None => true,
            }
        });

        println!("{}", res);
        res
    })

    // input_line.chars().enumerate().fold(false, |acc, (i, ch)| {
    //     let s: &char = &pattern.chars().nth(i).unwrap_or(' ');
    //     match s {
    //         "\\d" => match_digits_character_class(input_line),
    //         "\\w" => match_words_character_class(input_line),
    //         p if p.starts_with("[^") && p.ends_with("]") => {
    //             match_negative_character_group(input_line, p.trim_matches(&['[', '^', ']']))
    //         }
    //         p if p.starts_with("[") && p.ends_with("]") => {
    //             match_positive_character_group(input_line, p.trim_matches(&['[', ']']))
    //         }

    //         _ => input_line.contains(pattern),
    //     }
    // })
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
        println!("0");
        process::exit(0)
    } else {
        println!("1");
        process::exit(1)
    }
}
