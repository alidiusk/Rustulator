use calculator::calc::Calculator;

use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn repl() {
    let mut rl = Editor::<()>::new();
    let mut calculator = Calculator::new();
    println!("Welcome to Rustulator!");
    println!("Currently, arithmetic and abs, floor, log, ln, and trig functions are supported.");
    println!("Use Ctrl-C or type #quit to quit.");
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                // a more extensive command system can be added later if desired.
                if line == String::from("#quit") {
                    break;
                }

                match calculator.calculate(&line) {
                    Ok(val) => println!("{}", val),
                    Err(e) => println!("{}", e),
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
}
