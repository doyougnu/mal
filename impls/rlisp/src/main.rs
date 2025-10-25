use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

mod constants;
mod step0_repl;

use constants::HISTORY;
use step0_repl::{eval, print, read};

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    loop {
        match read(&mut rl) {
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                let result = eval(line);
                print(result);
            }
        };
    }
    let _ = rl.save_history(HISTORY);
    Ok(())
}
