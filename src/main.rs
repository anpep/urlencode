use std::env;
use std::io;
use std::process;

fn main() {
    for argument in env::args().skip(1) {
        if argument == "-h" {
            display_help();
            return;
        } else {
            eprintln!("error: unrecognized command-line argument: `{}'", argument);
            process::exit(1);
        }
    }

    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        if stdin.read_line(&mut line).unwrap_or_else(|_| {
            eprintln!("error: could not read from stdin");
            process::exit(1);
        }) == 0
        {
            break;
        }

        println!("{}", urlencode(&line[..line.len() - 1]));
    }
}

fn display_help() {
    let program_name = env::args()
        .next()
        .unwrap_or_else(|| "urlencode".to_string());
    println!("usage: {} [-h]", program_name);
    println!("\n\t-h\tdisplay this help screen\n");
    println!("urlencode will take all lines from standard input and perform");
    println!("URL-encoding on all of them, printing them to the standard output");
}

fn urlencode(string: &str) -> String {
    string
        .split("")
        .map(|string| {
            if string.is_empty() {
                return string.to_string();
            }

            let character = string.chars().next().unwrap();
            match character {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => character.to_string(),
                reserved_character => {
                    let mut buffer = [0; 4];
                    let mut percent_encoded_string = String::new();

                    for byte in reserved_character.encode_utf8(&mut buffer).as_bytes() {
                        percent_encoded_string += &format!("%{:02X}", byte).to_string();
                    }

                    percent_encoded_string
                }
            }
        })
        .collect()
}
