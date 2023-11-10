// TRILLESKA > UWUFY
// See README.TXT for usage directions and info.

use std::env;
use std::error::Error;
use std::fs;
use regex::Regex;
// see also: https://docs.rs/regex/latest/regex/

// read a file
fn uwufy(filename: String) -> Result<String, Box<dyn Error>> {
    let pre_uwu: String = fs::read_to_string(filename)?;
    let wegex = Regex::new("[lr]").unwrap(); // regex because why not
    let post_uwu = Regex::replace_all(&wegex, &pre_uwu, "w");
    Ok(post_uwu.to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let option = &args[1];
    let file = &args[2];

    if option == "-h" {
        println!("usage: uwufy -f [FILE_PATH]\n");
        std::process::exit(0);
    } else if option == "-f" {
        match uwufy(file.to_string()) {
            Ok(uwufied) => {
                println!("{}", uwufied);
                std::process::exit(0);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }
    }
}
