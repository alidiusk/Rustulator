use calculator::parser::Parser;
use calculator::eval::eval;

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

        let parser = Parser::new(&line);
        let ast = match parser {
          Err(e)     => Err(e),
          Ok(mut parser) => parser.parse(),
        };

        match ast {
          Ok(e) => println!("{:.2}", eval(e)),
          Err(e) => println!("{}", e),
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
