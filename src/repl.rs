use crate::evaluator::Engine;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Repl {
    engine: Engine,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
        }
    }

    pub fn run(&mut self) {
        let mut rl = Editor::<()>::new();
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }
        loop {
            let readline = rl.readline("(lex): ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    match self.engine.parse_line(&line) {
                        Ok(token) => println!("=> {}", token),
                        Err(_) => (),
                    }
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
}
