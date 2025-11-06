use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use rlisp::constants::HISTORY;
use rlisp::reader::parse_expr;
// use rlisp::types::Expr;
use rlisp::{eval, print, read};

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
                match parse_expr(&line) {
                    Err(e) => {
                        println!("Error: {:?}", e);
                        break;
                    }
                    Ok((_, expr)) => {
                        //  parser returns rest of input which should be empty
                        let result = eval(&expr);
                        println!("{}", print(result));
                    }
                }
            }
        };
    }
    let _ = rl.save_history(HISTORY);
    Ok(())
}
