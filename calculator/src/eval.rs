use crate::ast::Expr;

pub fn eval(ast: Expr) -> f64 {
  use crate::ast::Expr::*;

  match ast {
    Num(i) => i,
    Add(e1, e2) => eval(*e1) + eval(*e2),
    Sub(e1, e2) => eval(*e1) - eval(*e2),
    Mul(e1, e2) => eval(*e1) * eval(*e2),
    Div(e1, e2) => eval(*e1) / eval(*e2),
    Pow(e1, e2) => (eval(*e1)).powf(eval(*e2)),
    Neg(e) => -(eval(*e)),
  }
}

#[cfg(test)]
mod tests {
  use crate::parser::Parser;
  use super::*;

  #[test]
  fn test_eval() {
    let ast = Parser::new("1*(1+1)/2+2-1").unwrap().parse().unwrap();
    let evaled = eval(ast);
    assert_eq!(2.0, evaled);
  }

  #[test]
  fn test_eval2() {
    let ast = Parser::new("12*2/24 + 1 / 25 - 1.04").unwrap().parse().unwrap();
    let evaled = eval(ast);
    assert_eq!(0.0, evaled);
  }

  #[test]
  fn exp_eval() {
    let ast = Parser::new("2^2").unwrap().parse().unwrap();
    println!("{:?}", ast);
    let evaled = eval(ast);
    assert_eq!(4.0, evaled);
  }

  #[test]
  fn ord_of_op_test() {
    let ast = Parser::new("3*2^2-1/2").unwrap().parse().unwrap();
    println!("{:?}", ast);
    let evaled = eval(ast);
    assert_eq!(11.5, evaled);
 }

  #[test]
  fn distribute_prop() {
    let ast = Parser::new("3(2+1)").unwrap().parse().unwrap();
    println!("{:?}", ast);
    let evaled = eval(ast);
    assert_eq!(9.0, evaled);
 }

  #[test]
  fn paren_expr_mul() {
    let ast = Parser::new("(3+4)(2+1)").unwrap().parse().unwrap();
    println!("{:?}", ast);
    let evaled = eval(ast);
    assert_eq!(21.0, evaled);
 }
}
