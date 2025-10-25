use rustyline::{DefaultEditor, Result};

pub mod constants;
use constants::HISTORY;

pub fn read(rl: &mut DefaultEditor) -> Result<String> {
    if rl.load_history(HISTORY).is_err() {
        println!("Couldn't read previous history!");
    }
    rl.readline("user> ")
}

pub fn eval(input: String) -> String {
    input
}

pub fn print(input: String) -> String {
    input
}
