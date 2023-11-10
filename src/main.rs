// TRILLESKA > UWUFY
// See README.TXT for usage directions and info.

use std::error::Error;
use std::fs;
use clap::Parser;
use regex::Regex;
// see also: https://docs.rs/regex/latest/regex/

// arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'f', long = "file")]
    file: String,
}


// read a file
fn uwufy(filename: String) -> Result<String, Box<dyn Error>> {
    let pre_uwu: String = fs::read_to_string(filename)?;
    let wegex = Regex::new("[lr]").unwrap(); // regex because why not
    let post_uwu = Regex::replace_all(&wegex, &pre_uwu, "w");
    Ok(post_uwu.to_string())
}

fn main() {
    let args = Args::parse();

    match uwufy(args.file.to_string()) {
        Ok(uwufied) => {
            println!("{}", uwufied);
            std::process::exit(0);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
