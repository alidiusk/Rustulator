use crate::eval::{eval, EvalError};
use crate::parser::{Parser, ParseError};

use std::collections::HashMap;
use std::f64::consts::{PI, E};
use std::error;
use std::fmt;

type Environment = HashMap<String, f64>;
type Calculations = Vec<(String, f64)>;

pub struct Calculator {
  env: Environment,
  calcs: Calculations,
}

impl Calculator {
  pub fn from(env: Environment, calcs: Calculations) -> Self {
    Calculator {
      env,
      calcs,
    }
  }

  pub fn new() -> Self {
    let mut env: Environment = HashMap::new();
    env.insert("pi".to_string(), PI);
    env.insert("e".to_string(), E);
    Calculator {
      env,
      calcs: vec![],
    }
  }

  fn balance_parens<'a>(s: &'a str) -> String {
    let num = s.chars().fold(0, |acc, c| 
      if c == '(' { 
        acc + 1 
      } else if c == ')' { 
        acc - 1
      } else { 
        acc 
      });
    if num == 0 {
      s.to_string()
    } else if num > 0 {
      [s, ")".repeat(num).as_str()].concat()
    } else {
      s[0..s.len()-num].to_string()
    }
  }

  pub fn calculate<'a>(&mut self, calc: &'a str) -> Result<f64, CalculatorError> {
    let calc = Self::balance_parens(calc);
    let mut parser = Parser::new(&calc).unwrap();
    let val = eval(parser.parse()?, &mut self.env)?;
    self.calcs.push((calc.to_string(), val));
    Ok(val)
  }
}

#[derive(Debug, Clone)]
pub struct CalculatorError(String);

impl fmt::Display for CalculatorError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "{}", self.0);
  }
}

impl error::Error for CalculatorError {
  fn description(&self) -> &str {
    &self.0
  }
}

impl From<ParseError> for CalculatorError {
  fn from(err: ParseError) -> Self {
    match err {
      ParseError::ExpectErr(s) => CalculatorError(s),
      ParseError::UnknownAtom(s) => CalculatorError(s),
      ParseError::InvalidInput(s) => CalculatorError(s),
      ParseError::NoneError(s) => CalculatorError(s),
    }
  }
}

impl From<EvalError> for CalculatorError {
  fn from(err: EvalError) -> Self {
    match err {
      EvalError::UnknownVar(s) => CalculatorError(s),
    }
  }
}
