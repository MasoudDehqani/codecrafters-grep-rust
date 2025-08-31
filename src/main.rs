use std::env;
use std::io;
use std::process;

fn match_positive_character_group(input_line: &str, p: &str) -> bool {
    p.chars().any(|ch| input_line.contains(ch))
}

fn match_negative_character_group(input_line: &str, p: &str) -> bool {
    p.chars().any(|ch| !input_line.contains(ch))
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    match pattern {
        p if p.starts_with("[^") && p.ends_with("]") => {
            return match_negative_character_group(input_line, p.trim_matches(&['[', '^', ']']))
        }
        p if p.starts_with("[") && p.ends_with("]") => {
            return match_positive_character_group(input_line, p.trim_matches(&['[', ']']))
        }
        p if p.starts_with("^") && p.ends_with("$") => {
            let trimmed_pattern = pattern.trim_matches(&['^', '$']);
            return input_line == trimmed_pattern;
        }
        p if p.starts_with("^") => return input_line.starts_with(pattern.trim_start_matches("^")),
        p if p.ends_with("$") => return input_line.ends_with(pattern.trim_end_matches("$")),
        p if p.contains("+") => match p.find("+") {
            Some(i) => {
                let (m, _) = p.split_at(i);
                println!("{}", m);
                return input_line.contains(m);
            }
            None => (),
        },
        _ => (),
    }

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

    let mut res = false;

    for (i, _ch) in input_line.char_indices() {
        let slice = &input_line[i..];

        if patterns.len() > slice.len() {
            res = false;
            break;
        }

        let i1 = slice.chars();
        let i2 = patterns.iter();

        let z = i1.zip(i2);

        let r = z.fold(vec![], |acc, (ch, p)| match p.as_str() {
            "\\d" => {
                let test = ch.is_digit(10);

                vec![acc, vec![test]].concat()
            }
            "\\w" => {
                let test = ch.is_alphanumeric() || ch == '_';
                vec![acc, vec![test]].concat()
            }
            _ => {
                let test = ch.to_string() == *p;
                vec![acc, vec![test]].concat()
            }
        });

        res = r.iter().all(|a| *a);

        if res {
            break;
        }
    }

    res
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
