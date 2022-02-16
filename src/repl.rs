use rustyline::{error::ReadlineError, Editor};

use crate::{brim::run, prelude::make_prelude};

pub fn read_from_prompt() {
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let mut env = make_prelude();

    loop {
        let readline = rl.readline("brim> ");
        match readline {
            Ok(line) => {
                let (result, env_) = run(line.as_str(), env);
                println!("{}", result);
                env = env_
            }
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
        }
    }
    rl.save_history("history.txt").unwrap();
}
