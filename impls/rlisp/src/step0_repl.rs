// use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crate::constants::HISTORY;

pub fn read(rl: &mut DefaultEditor) -> Result<String> {
    if rl.load_history(HISTORY).is_err() {
        println!("Couldn't read previous history!");
    }
    rl.readline("> ")
}

pub fn eval(input: String) -> String {
    println!("Eval: {}", input);
    input
}

pub fn print(input: String) -> String {
    println!("Print: {}", input);
    input
}
