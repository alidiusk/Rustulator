use crate::parser::Parser;
use crate::ast::Expr;

pub fn eval(ast: Expr) -> i32 {
  use crate::ast::Expr::*;

  match ast {
    Num(i) => i,
    Add(e1, e2) => eval(*e1) + eval(*e2),
    Sub(e1, e2) => eval(*e1) - eval(*e2),
    Mul(e1, e2) => eval(*e1) * eval(*e2),
    Div(e1, e2) => eval(*e1) / eval(*e2),
    Neg(e) => -(eval(*e)),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_eval() {
    let ast = Parser::new("1*(1+1)/2+2-1").parse().unwrap();
    let evaled = eval(ast);
    assert_eq!(2, evaled);
  }
}
