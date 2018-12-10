use crate::parser::Parser;
use crate::eval::eval;

use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn repl() {
  let mut rl = Editor::<()>::new();
  println!("Welcome to Rusty Calculator!");
  println!("Currently, integer addition, subtraction, multiplication, and division are supported.");
  println!("Use Ctrl-C or type #quit to quit.");
  loop {
    let readline = rl.readline(">> ");
    match readline {
      Ok(line) => {
        // a more extensive command system can be added later if desired.
        if line == String::from("#quit") {
          break
        }

        let ast = Parser::new(&line).parse();
        match ast {
          Ok(e) => println!("{}", eval(e)),
          Err(e) => println!("{:?}", e),
        }
      },
      Err(ReadlineError::Interrupted) => {
        println!("CTRL-C");
        break
      },
      Err(ReadlineError::Eof) => {
        println!("CTRL-D");
        break
      },
      Err(err) => {
        println!("Error: {:?}", err);
        break
      },
    }
  }
}
