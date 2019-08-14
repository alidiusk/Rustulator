//! # Lexer
//! 
//! Handles the tokenization of raw &str input.

use crate::ast::{Token, get_function_token};

use std::str::Chars;
use std::iter::Peekable;

use std::fmt;
use std::error;

#[derive(Debug)]
/// Iterator that emits Tokens.
pub struct Lexer<'a> {
  source: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
  pub fn new(source: &'a str) -> Self {
    Lexer {
      source: source.chars().peekable(),
    }
  }

  pub fn set_source(&mut self, source: &'a str) {
    self.source = source.chars().peekable();
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token;

  fn next(&mut self) -> Option<Token> {
    let c = self.source.next();
    match c {
      Some('0'..='9') => {
        // safe unwrap - we checked that c was Some() to get hree
        let mut num = c.unwrap().to_string();
        while let Some(n) = self.source.peek() {
          if n.is_numeric() || n == &'.' {
            // safe unwrap - we checked that n was Some() to get here
            num.push(self.source.next().unwrap());
          } else {
            break;
          }
        }

        Some(Token::Num(num.parse::<f64>().unwrap()))
      },
      Some('a'..='z') | Some('A'..='Z') => {
        let mut ident = c.unwrap().to_string();
        while let Some(ch) = self.source.peek() {
          if ch.is_alphabetic() {
            ident.push(self.source.next().unwrap());
          } else {
            break;
          }
        }

        match get_function_token(ident.as_str()) {
          Some(Token::Func(f)) => Some(Token::Func(f)),
          _                    => Some(Token::Ident(ident)),
        }
      },
      Some('=')      => Some(Token::Equals),
      Some('+')      => Some(Token::Add),
      Some('-')      => Some(Token::Sub),
      Some('*')      => Some(Token::Mul),
      Some('/')      => Some(Token::Div),
      Some('^')      => Some(Token::Pow),
      Some('(')      => Some(Token::LParen),
      Some(')')      => Some(Token::RParen),
      None           => Some(Token::Eof),
      Some(' ')      => self.next(),
      Some('\n')     => self.next(),
      Some('\t')     => self.next(),
      Some(_)        => None,
    }
  }
}

#[derive(Debug)]
/// Defines the various errors that can occur during evaluation.
pub enum LexError {
  InvalidChar(String),
}

impl fmt::Display for LexError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::LexError::*;

    match *self {
      InvalidChar(ref e) => write!(f, "Lexing error: {}", e),
    }
  }
}

impl error::Error for LexError {
  fn description(&self) -> &str {
    use self::LexError::*;

    match *self {
      InvalidChar(ref e) => e,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_num_token() {
    let mut lexer = Lexer::new("345671");
    assert_eq!(Token::Num(345671.0), lexer.next().unwrap());
  }

  #[test]
  fn test_mult_num_tokens() {
    let mut lexer = Lexer::new("7560 2371 2903");
    assert_eq!(Token::Num(7560.0), lexer.next().unwrap());
    assert_eq!(Token::Num(2371.0), lexer.next().unwrap());
    assert_eq!(Token::Num(2903.0), lexer.next().unwrap());
  }

  #[test]
  fn test_add() {
    let mut lexer = Lexer::new("+");
    assert_eq!(Token::Add, lexer.next().unwrap());
  }

  #[test]
  fn test_sub() {
    let mut lexer = Lexer::new("-");
    assert_eq!(Token::Sub, lexer.next().unwrap());
  }

  #[test]
  fn test_mul() {
    let mut lexer = Lexer::new("*");
    assert_eq!(Token::Mul, lexer.next().unwrap());
  }

  #[test]
  fn test_div() {
    let mut lexer = Lexer::new("/");
    assert_eq!(Token::Div, lexer.next().unwrap());
  }

  #[test]
  fn test_parens() {
    let mut lexer = Lexer::new("()");
    assert_eq!(Token::LParen, lexer.next().unwrap());
    assert_eq!(Token::RParen, lexer.next().unwrap());
  }

  #[test]
  fn test_ident() {
    let mut lexer = Lexer::new("lol");
    assert_eq!(Token::Ident("lol".to_string()), lexer.next().unwrap());
  }
}
