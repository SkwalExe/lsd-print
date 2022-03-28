// Author : SkwalExe
// Github : https://github.com/SkwalExe

use rand::Rng;
use std::io::{self, BufRead};
use std::process;

const RED: &str = "\x1b[91m";
const GREEN: &str = "\x1b[92m";
const YELLOW: &str = "\x1b[93m";
const BLUE: &str = "\x1b[94m";
const MAGENTA: &str = "\x1b[95m";
const CYAN: &str = "\x1b[96m";
const WHITE: &str = "\x1b[97m";
const RESET: &str = "\x1b[0m";
const BG_RED: &str = "\x1b[41m";
const BG_GREEN: &str = "\x1b[42m";
const BG_YELLOW: &str = "\x1b[43m";
const BG_BLUE: &str = "\x1b[44m";
const BG_MAGENTA: &str = "\x1b[45m";
const BG_CYAN: &str = "\x1b[46m";

// this function will print a message with random colors
//
// parameters
// - text           : the message to print
// - background     : if the color should be in the background
// - color256       : if the color should be in the 256 colors
// - newline        : if the message should be printed with a newline
//

fn drug_print(text: &String, background: bool, colors256: bool, newline: bool) {
    let mut colored_text = String::new();

    for c in text.chars() {
        let color: String; // the color to use for the character
        if colors256 {
            color = format!(
                "\x1b[{};5;{}m",
                if background { "48" } else { "38" },
                rand::thread_rng().gen_range(0..231)
            );
        } else {
            if background {
                color = match rand::thread_rng().gen_range(0..6) {
                    0 => BG_RED.to_string(),
                    1 => BG_GREEN.to_string(),
                    2 => BG_YELLOW.to_string(),
                    4 => BG_MAGENTA.to_string(),
                    5 => BG_CYAN.to_string(),
                    3 => BG_BLUE.to_string(),
                    _ => BG_RED.to_string(),
                }
            } else {
                color = match rand::thread_rng().gen_range(0..6) {
                    0 => RED.to_string(),
                    1 => GREEN.to_string(),
                    2 => YELLOW.to_string(),
                    3 => BLUE.to_string(),
                    4 => MAGENTA.to_string(),
                    5 => CYAN.to_string(),
                    _ => RESET.to_string(),
                };
            }
        }

        colored_text.push_str(&color); // add the color to the colored text
        colored_text.push(c); // then push the character
        colored_text.push_str(RESET); // and reset the color
    }

    print!("{}{}", colored_text, if newline { "\n" } else { "" }); // and print the colored text
}

fn main() {
    let mut colors256 = false; // if the colors should be in the 256 colors
    let mut background = false; // if the colors should be in the background
    let mut command = "print"; // command to execute (print, version, help)
    let mut newline = true; // if a newline should be printed after the message
    let mut text = String::new(); // the text to print
    let mut args: Vec<String> = std::env::args().collect(); // arguments vector
    args.remove(0); // remove the program name

    while args.len() > 0 {
        match args[0].as_str() {
            "--" => {
                command = "print";
                args.remove(0);
                text = args.join(" "); // the text to print is set the everything after --
                break; // stop parsing arguments
            }
            "-256" => {
                // enables 256 colors
                colors256 = true;
                args.remove(0);
            }
            "-b" | "--background" => {
                // put the color in the background
                background = true;
                args.remove(0);
            }
            "--version" | "-v" => {
                command = "version";
                args.remove(0);
            }
            "--nonewline" | "-n" => {
                newline = false;
                args.remove(0);
            }
            "--help" | "-h" => {
                command = "help";
                args.remove(0);
            }
            _ => {
                println!(
                    "{}Invalid argument: {}{} {} {}",
                    RED, WHITE, BG_RED, args[0], RESET
                );
                process::exit(1);
            }
        }
    }

    match command {
        "print" => {
            if text.is_empty() {
                // if no text is directly given
                let stdin = io::stdin();
                let lines = stdin.lock().lines();

                let mut line_count = 0;

                for line in lines {
                    // try to read from pipe
                    line_count += 1;
                    let line = line.expect("Could not read line from standard in");
                    drug_print(&line, background.clone(), colors256, newline.clone());
                }

                if line_count == 0 {
                    // if there is no text being piped into the program
                    println!("{}No text specified{}", RED, RESET);
                    process::exit(1);
                }
            }

            drug_print(&text, background.clone(), colors256, newline.clone());

            // just lsd-print the text
        }
        "version" => println!(
            "{}lsd-print, by Skwal => {}{}{}",
            MAGENTA,
            RED,
            env!("CARGO_PKG_VERSION"),
            RESET
        ),
        "help" => {
            drug_print(&String::from("LSD print"), false, false, true);
            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
            println!("Author: {}SkwalExe{}", MAGENTA, RESET);
            println!("Github: {}https://github.com/SkwalExe{}", MAGENTA, RESET);
            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
            drug_print(
                &String::from("Just a print tool, but we gave it lsd"),
                false,
                false,
                true,
            );
            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
            println!("Options : ");
            println!(
                "\t{}--version, -v: {}Prints the version of the program{}",
                MAGENTA, YELLOW, RESET
            );
            println!(
                "\t{}--help, -h: {}Prints this help message{}",
                MAGENTA, YELLOW, RESET
            );
            println!(
                "\t{}-- [text]: {}Prints the text you give{}",
                MAGENTA, YELLOW, RESET
            );
            println!(
                "\t{}-256: {}Prints with 256 colors{}",
                MAGENTA, YELLOW, RESET
            );
            println!(
                "\t{}-b, --background: {}Prints color in the background{}",
                MAGENTA, YELLOW, RESET
            );
            println!(
                "{}\tprogram | lsd-print: {}Prints the output of program{}",
                MAGENTA, YELLOW, RESET
            );
            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
        }

        _ => {}
    }
}
