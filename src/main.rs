// Author : SkwalExe
// Github : https://github.com/SkwalExe

#![allow(dead_code)]

use rand::Rng;
use snailquote::unescape;
use std::io;
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

fn main() {
    let mut colors = "8";
    let mut background = false;
    let mut command = "pipe";
    let mut text = String::new();
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    while args.len() > 0 {
        match args[0].as_str() {
            "--" => {
                command = "print";
                args.remove(0);
                text = args.join(" ");
                break;
            }
            "-256" => {
                colors = "256";
                args.remove(0);
            }
            "-b" | "--background" => {
                background = true;
                args.remove(0);
            }
            "--version" | "-v" => {
                command = "version";
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
        "pipe" => loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read from pipe");
            input = input.trim().to_string();
            if input == "" {
                break;
            }
            drug_print(&unescape(&input).unwrap(), background.clone(), &colors);
        },
        "print" => {
            drug_print(&text, background.clone(), &colors);
        }
        "version" => println!(
            "{}lsd-print, by Skwal => {}{}{}",
            MAGENTA,
            RED,
            env!("CARGO_PKG_VERSION"),
            RESET
        ),
        "help" => {
            drug_print(&String::from("LSD print"), false, &"8");
            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
            println!("Author: {}SkwalExe{}", MAGENTA, RESET);
            println!("Github: {}https://github.com/SkwalExe{}", MAGENTA, RESET);
            println!("{}━━━━━━━━━━━━━━━━━{}", MAGENTA, RESET);
            drug_print(
                &String::from("Just a print tool, but we gave it lsd"),
                false,
                &"8",
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
fn drug_print(text: &String, bg: bool, colors: &str) {
    let mut color_text = String::new();
    for c in text.chars() {
        let color: String;
        if colors == "256" {
            color = format!(
                "\x1b[{};5;{}m",
                if bg { "48" } else { "38" },
                rand::thread_rng().gen_range(0..231)
            );
        } else {
            if bg {
                color = match rand::thread_rng().gen_range(0..6) {
                    0 => BG_RED.to_string(),
                    1 => BG_GREEN.to_string(),
                    2 => BG_YELLOW.to_string(),
                    4 => BG_MAGENTA.to_string(),
                    5 => BG_CYAN.to_string(),
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

        color_text.push_str(&color);
        color_text.push(c);
        color_text.push_str(RESET);
    }
    println!("{}", color_text);
}
