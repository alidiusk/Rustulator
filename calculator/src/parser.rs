use std::error;
use std::option::NoneError;

use std::fmt;

use crate::ast::{Expr, Func, Precedence, Token};
use crate::lexer::{Lexer};

#[derive(Debug)]
pub struct Parser<'a> {
  lexer: Lexer<'a>,
  current_token: Token,
  // this parser actually works as an LL(0), so peek_token
  // technically isn't even necessary, but I'm keeping it 
  // in case it is needed for future additions.
  peek_token: Token,
}

impl<'a> Parser<'a> {
  pub fn new(source: &'a str) -> Result<Self, ParseError> {
    let mut lexer = Lexer::new(source);
    let cur = lexer.next()?;
    let peek = lexer.next()?;

    Ok(Parser {
      lexer: lexer,
      current_token: cur,
      peek_token: peek,
    })
  }

  fn next_token(&mut self) -> Result<(), ParseError> {
    self.current_token = self.peek_token.clone();
    self.peek_token = self.lexer.next()?;
    Ok(())
  }
  
  pub fn parse(&mut self) -> Result<Expr, ParseError> {
    let expr = self.parse_expr(Precedence::Lowest);
    match expr {
      Ok(e) => Ok(e),
      Err(e) => Err(e),
    }
  }

  fn parse_expr(&mut self, prec: Precedence) -> Result<Expr, ParseError> {
    let mut left = self.parse_atom()?;
    while prec < self.current_token.get_precedence() {
      if self.current_token == Token::Eof {
        break;
      }

      let right = self.parse_infix_op(left.clone())?;
      left = right;
    }
    Ok(left)
  }

  fn parse_infix_op(&mut self, left: Expr) -> Result<Expr, ParseError> {
    match self.current_token {
      Token::Add => {
        self.next_token()?;
        let right = self.parse_expr(Precedence::Sum)?;
        Ok(Expr::Add(box left, box right))
      },
      Token::Sub => {
        self.next_token()?;
        let right = self.parse_expr(Precedence::Sum)?;
        Ok(Expr::Sub(box left, box right))
      },
      Token::Mul => {
        self.next_token()?;
        let right = self.parse_expr(Precedence::Product)?;
        Ok(Expr::Mul(box left, box right))
      },
      Token::Div => {
        self.next_token()?;
        let right = self.parse_expr(Precedence::Product)?;
        Ok(Expr::Div(box left, box right))
      },
      Token::Pow => {
        self.next_token()?;
        let right = self.parse_expr(Precedence::Power)?;
        Ok(Expr::Pow(box left, box right))
      },
      _ => {
        Err(ParseError::InvalidInput(
          format!("parse_infix_op: Expected infix_op, got {}", self.current_token)))
      },
    }
  }

  fn parse_atom(&mut self) -> Result<Expr, ParseError> {
    let token = self.current_token.clone();
    match token {
      Token::Num(i) => { 
        self.next_token()?;
        match self.current_token {
          Token::LParen => {
            let right = self.parse_expr(Precedence::Product)?;
            return Ok(Expr::Mul(box Expr::Num(i), box right));
          },
          Token::Func(f) => {
            self.next_token()?;
            let right = self.parse_expr(Precedence::Product)?;
            return Ok(Expr::Mul(box Expr::Num(i), box Self::function_expr(f, right)));
          },
          _ => Ok(Expr::Num(i)),
        }
      },
      Token::Func(f) => {
        self.next_token()?;
        let expr = self.parse_expr(Precedence::Function)?;
        Ok(Self::function_expr(f, expr))
      }
      Token::LParen => {
        self.next_token()?;
        let expr = self.parse_expr(Precedence::Lowest)?;
        self.expect(Token::RParen)?;
        if self.current_token == Token::LParen {
          let right = self.parse_expr(Precedence::Product)?;
          return Ok(Expr::Mul(box expr, box right));
        }

        Ok(expr)
      },
      _ => {
        Err(ParseError::UnknownAtom(format!("Unknown atom: {}", token)))
      }
    }
  }

  fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
    if expected == self.current_token {
      self.next_token()?;
      Ok(())
    } else {
      Err(ParseError::ExpectErr(format!("Expected {}, got {}", expected, self.current_token)))
    }
  }

  // Takes a Func::Func and an expression and 
  // returns the corresponding function expr. 
  fn function_expr(f: Func, e: Expr) -> Expr {
    match f {
      Func::Abs => Expr::Abs(box e),
      Func::Floor => Expr::Floor(box e),
      Func::Log => Expr::Log(box e),
      Func::Ln => Expr::Ln(box e),
      Func::Sin => Expr::Sin(box e),
      Func::Cos => Expr::Cos(box e),
      Func::Tan => Expr::Tan(box e),
      Func::Arcsin => Expr::Arcsin(box e),
      Func::Arccos => Expr::Arccos(box e),
      Func::Arctan => Expr::Arctan(box e),
    }
  }
}

// Error handling will need to be improved.

#[derive(Debug)]
pub enum ParseError {
  ExpectErr(String),
  UnknownAtom(String),
  InvalidInput(String),
  NoneError(String),
}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::ParseError::*;

    match *self {
      ExpectErr(ref e) => write!(f, "Error: {}", e),
      UnknownAtom(ref e) => write!(f, "Error: {}", e),
      InvalidInput(ref e) => write!(f, "Error: {}", e),
      NoneError(ref e) => write!(f, "Error: {}", e),
    }
  }
}

impl error::Error for ParseError {
  fn description(&self) -> &str {
    use self::ParseError::*;

    match *self {
      ExpectErr(ref e) => e,
      UnknownAtom(ref e) => e,
      InvalidInput(ref e) => e,
      NoneError(ref e) => e,
    }
  }
}

impl From<NoneError> for ParseError {
  fn from(_err: NoneError) -> Self {
    ParseError::NoneError(String::from("Invalid character entered."))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic_add() {
    let mut parser = Parser::new("1+1").unwrap();
    let expected_expr = Expr::Add(box Expr::Num(1.0), box Expr::Num(1.0));
    assert_eq!(parser.parse().unwrap(), expected_expr);
  }

  #[test]
  fn basic_sub() {
    let mut parser = Parser::new("1-1").unwrap();
    let expected_expr = Expr::Sub(box Expr::Num(1.0), box Expr::Num(1.0));
    assert_eq!(parser.parse().unwrap(), expected_expr);
  }

  #[test]
  fn basic_mul() {
    let mut parser = Parser::new("1*1").unwrap();
    let expected_expr = Expr::Mul(box Expr::Num(1.0), box Expr::Num(1.0));
    assert_eq!(parser.parse().unwrap(), expected_expr);
  }

  #[test]
  fn basic_div() {
    let mut parser = Parser::new("1/1").unwrap();
    let expected_expr = Expr::Div(box Expr::Num(1.0), box Expr::Num(1.0));
    assert_eq!(parser.parse().unwrap(), expected_expr);
  }

  #[test]
  fn basic_paren() {
    let mut parser = Parser::new("1*(1+1)").unwrap();
    let expected_expr = Expr::Mul(box Expr::Num(1.0), box Expr::Add(box Expr::Num(1.0), box Expr::Num(1.0)));
    assert_eq!(parser.parse().unwrap(), expected_expr);
  }

  #[test]
  fn basic_func() {
    let mut parser = Parser::new("sin(1 + 1)").unwrap();
    let expected_expr = Expr::Sin(box Expr::Add(box Expr::Num(1.0), box Expr::Num(1.0)));
    assert_eq!(parser.parse().unwrap(), expected_expr);
  }

  #[test]
  fn func_multiplication() {
    let mut parser = Parser::new("5sin(1 + 1)").unwrap();
    let expected_expr = Expr::Mul(box Expr::Num(5.0), box Expr::Sin(box Expr::Add(box Expr::Num(1.0), box Expr::Num(1.0))));
    assert_eq!(parser.parse().unwrap(), expected_expr);
  }

  #[test]
  fn ord_of_ops() {
    let mut parser = Parser::new("1/(1+1)*1+2").unwrap();
    let expected_expr = Expr::Add(
      box Expr::Mul(
        box Expr::Div(
          box Expr::Num(1.0), 
          box Expr::Add(box Expr::Num(1.0), box Expr::Num(1.0))),
        box Expr::Num(1.0)),
      box Expr::Num(2.0));
    assert_eq!(parser.parse().unwrap(), expected_expr);
  }
}
