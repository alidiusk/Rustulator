use crate::ast::Expr;

use std::collections::HashMap;
use std::error;
use std::fmt;

pub fn eval(ast: Expr, env: &mut HashMap<String, f64>) -> Result<f64, EvalError> {
  use crate::ast::Expr::*;

  match ast {
    Num(i) => Ok(i),
    Ident(s) => {
      match env.get(&s) {
        Some(f) => Ok(*f),
        None    => Err(EvalError::UnknownVar(format!("Unknown variable: {}", s))),
      }
    },
    Assign(s, e) => {
      let val = eval(*e, env)?;
      env.insert(s, val);
      Ok(val)
    },
    Add(e1, e2) => Ok(eval(*e1, env)? + eval(*e2, env)?),
    Sub(e1, e2) => Ok(eval(*e1, env)? - eval(*e2, env)?),
    Mul(e1, e2) => Ok(eval(*e1, env)? * eval(*e2, env)?),
    Div(e1, e2) => Ok(eval(*e1, env)? / eval(*e2, env)?),
    Pow(e1, e2) => Ok((eval(*e1, env)?).powf(eval(*e2, env)?)),
    Neg(e)      => Ok(-(eval(*e, env)?)),
    Abs(e)      => Ok((eval(*e, env)?).abs()),
    Floor(e)    => Ok((eval(*e, env)?).floor()),
    Log(e)      => Ok((eval(*e, env)?).log10()),
    Ln(e)       => Ok((eval(*e, env)?).ln()),
    Sin(e)      => Ok((eval(*e, env)?).sin()),
    Cos(e)      => Ok((eval(*e, env)?).cos()),
    Tan(e)      => Ok((eval(*e, env)?).tan()),
    Arcsin(e)   => Ok((eval(*e, env)?).asin()),
    Arccos(e)   => Ok((eval(*e, env)?).acos()),
    Arctan(e)   => Ok((eval(*e, env)?).atan()),
  }
}

#[derive(Debug)]
pub enum EvalError {
  UnknownVar(String),
}

impl fmt::Display for EvalError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::EvalError::*;

    match *self {
      UnknownVar(ref e) => write!(f, "{}", e),
    }
  }
}

impl error::Error for EvalError {
  fn description(&self) -> &str {
    use self::EvalError::*;
    
    match *self {
      UnknownVar(ref e) => e, 
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::parser::Parser;
  use super::*;

  #[test]
  fn test_eval() {
    let ast = Parser::new("1*(1+1)/2+2-1").unwrap().parse().unwrap();
    let mut env = HashMap::new();
    let evaled = eval(ast, &mut env).unwrap();
    assert_eq!(2.0, evaled);
  }

  #[test]
  fn test_eval2() {
    let ast = Parser::new("12*2/24 + 1 / 25 - 1.04").unwrap().parse().unwrap();
    let mut env = HashMap::new();
    let evaled = eval(ast, &mut env).unwrap();
    assert_eq!(0.0, evaled);
  }

  #[test]
  fn exp_eval() {
    let ast = Parser::new("2^2").unwrap().parse().unwrap();
    println!("{:?}", ast);
    let mut env = HashMap::new();
    let evaled = eval(ast, &mut env).unwrap();
    assert_eq!(4.0, evaled);
  }

  #[test]
  fn ord_of_op_test() {
    let ast = Parser::new("3*2^2-1/2").unwrap().parse().unwrap();
    println!("{:?}", ast);
    let mut env = HashMap::new();
    let evaled = eval(ast, &mut env).unwrap();
    assert_eq!(11.5, evaled);
 }

  #[test]
  fn distribute_prop() {
    let ast = Parser::new("3(2+1)").unwrap().parse().unwrap();
    println!("{:?}", ast);
    let mut env = HashMap::new();
    let evaled = eval(ast, &mut env).unwrap();
    assert_eq!(9.0, evaled);
 }

  #[test]
  fn paren_expr_mul() {
    let ast = Parser::new("(3+4)(2+1)").unwrap().parse().unwrap();
    println!("{:?}", ast);
    let mut env = HashMap::new();
    let evaled = eval(ast, &mut env).unwrap();
    assert_eq!(21.0, evaled);
 }
}
