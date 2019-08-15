//! # Calc
//!
//! Exposes a Calculator that calculates any given mathematical expression.

use crate::eval::{eval, EvalError};
use crate::parser::{Parser, ParseError};

use std::collections::HashMap;
use std::f64::consts::{PI, E};
use std::error;
use std::fmt;

/// A state of all defined constants/variables.
pub type Environment = HashMap<String, f64>;

/// A log of all prior calculations and their result.
pub type Calculations = Vec<(String, f64)>;

#[derive(Debug)]
/// A calculator that maintains a state of all prior calculations
/// as well as the currently defined constants/variables.
pub struct Calculator {
  env: Environment,
  calcs: Calculations,
}

impl Calculator {
  /// Returns a `Calculator` with a given `Environment` and prior
  /// log of `Calculations`.
  pub fn from(env: Environment, calcs: Calculations) -> Self {
    Calculator {
      env,
      calcs,
    }
  }

  /// Returns a `Calculator` with an empty log of `Calculations` 
  /// and an `Environment` of predefined common constants.
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

  /// Returns the log of `Calculations`, which is just a 
  /// `Vec(String, f64)>`.
  pub fn get_log(&self) -> Calculations {
    self.calcs.clone()
  }

  /// Takes a `&str` that represents a mathematical expression and returns the value.
  /// Returns a `CalculatorError` in the event evaluation fails.
  pub fn calculate<'a>(&mut self, calc: &'a str) -> Result<f64, CalculatorError> {
    let calc = Self::balance_parens(calc);
    let mut parser = Parser::new(&calc)?;
    let val = eval(parser.parse()?, &mut self.env)?;
    self.calcs.push((calc.to_string(), val));
    Ok(val)
  }
}

#[derive(Debug, Clone)]
/// Defines a calculator error. 
/// Contains a `String` that describes the error.
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
