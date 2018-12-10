use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
  Num(i32),
  Add(Box<Expr>, Box<Expr>),
  Sub(Box<Expr>, Box<Expr>),
  Mul(Box<Expr>, Box<Expr>),
  Div(Box<Expr>, Box<Expr>),
  Neg(Box<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  Num(i32),
  Add,
  Sub,
  Mul,
  Div,
  RParen,
  LParen,
  Eof,
}

impl Token {
  pub fn get_precedence(&self) -> Precedence {
    use self::Token::*;
    use self::Precedence::*;

    match *self {
      Add | Sub => Sum,
      Mul | Div => Product,
      // should be unreachable...
      _ => Lowest,
    }
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    use self::Token::*;

    match *self {
      Num(i) => write!(f, "Num({})", i),
      Add => write!(f, "Add"),
      Sub => write!(f, "Sub"),
      Mul => write!(f, "Mul"),
      Div => write!(f, "Div"),
      LParen => write!(f, "("),
      RParen => write!(f, ")"),
      Eof => write!(f, "Eof"),
    }
  }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
  Lowest,
  Sum,
  Product,
  Prefix,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_token() {
    let num = Token::Num(32);
    assert_eq!(Token::Num(32), num);
  }

  #[test]
  fn test_expr() {
    let addition = Expr::Add(box Expr::Num(32), box Expr::Num(16));
    assert_eq!(Expr::Add(box Expr::Num(32), box Expr::Num(16)), addition);
  }
}
