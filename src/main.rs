#![feature(uniform_paths)]
#![feature(box_syntax)]

mod ast;
mod lexer;
mod parser;
mod eval;
mod repl;

use crate::repl::repl;

fn main() {
  repl();
}
