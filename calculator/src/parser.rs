//! # Parser
//!
//! Handles the parsing of a mathematical expression to construct
//! an AST that can be evaluated.

use std::error;
use std::fmt;
use std::option::NoneError;

use crate::ast::{Expr, Func, Precedence, Token};
use crate::lexer::Lexer;

#[derive(Debug)]
/// Object that takes a `&str` and returns a AST of `Expr`.
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    // this parser actually works as an LL(0), so peek_token
    // technically isn't even necessary, but I'm keeping it
    // in case it is needed for future additions.
    peek_token: Token,
    // Constants and then variable assignment
}

impl<'a> Parser<'a> {
    /// Takes a `&str` and returns a `Parser`. If the source is somehow invalid,
    /// it will return a `ParseError`.
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

    /// Steps into the next token.
    fn next_token(&mut self) -> Result<(), ParseError> {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next()?;
        Ok(())
    }

    /// Returns a single Expr that represents the AST of the entire computation.
    /// If there was an error during parsing, returns a `ParseError`.
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

    /// Parses an expression with an infix operator. Takes the left expression and
    /// then parses the right before returning an `Expr` that combines the two.
    fn parse_infix_op(&mut self, left: Expr) -> Result<Expr, ParseError> {
        match self.current_token {
            Token::Add => {
                // Consume the token
                self.next_token()?;
                // Get the right expression
                let right = self.parse_expr(Precedence::Sum)?;
                // Return an Add of the left and right
                Ok(Expr::Add(box left, box right))
            }
            Token::Sub => {
                self.next_token()?;
                let right = self.parse_expr(Precedence::Sum)?;
                Ok(Expr::Sub(box left, box right))
            }
            Token::Mul => {
                self.next_token()?;
                let right = self.parse_expr(Precedence::Product)?;
                Ok(Expr::Mul(box left, box right))
            }
            Token::Div => {
                self.next_token()?;
                let right = self.parse_expr(Precedence::Product)?;
                Ok(Expr::Div(box left, box right))
            }
            Token::Pow => {
                self.next_token()?;
                let right = self.parse_expr(Precedence::Power)?;
                Ok(Expr::Pow(box left, box right))
            }
            _ => Err(ParseError::InvalidInput(format!(
                "parse_infix_op: Expected infix_op, got {}",
                self.current_token
            ))),
        }
    }

    /// Parses a single atom of an expression. This can be just a number or
    /// an entire sub expression.
    fn parse_atom(&mut self) -> Result<Expr, ParseError> {
        let token = self.current_token.clone();
        match token {
            Token::Sub => {
                self.next_token()?;
                let expr = self.parse_expr(Precedence::Prefix)?;
                Ok(Expr::Neg(box expr))
            }
            Token::Num(i) => {
                self.next_token()?;
                match self.current_token {
                    Token::LParen | Token::Func(_) | Token::Ident(_) => {
                        let right = self.parse_expr(Precedence::Product)?;
                        return Ok(Expr::Mul(box Expr::Num(i), box right));
                    }
                    _ => Ok(Expr::Num(i)),
                }
            }
            Token::Func(f) => {
                self.next_token()?;
                let expr = self.parse_expr(Precedence::Function)?;
                Ok(Self::function_expr(f, expr))
            }
            Token::Ident(ident) => {
                self.next_token()?;
                match self.current_token {
                    Token::Equals => {
                        self.next_token()?;
                        let expr = self.parse_expr(Precedence::Assign)?;
                        return Ok(Expr::Assign(ident, box expr));
                    }
                    Token::LParen | Token::Num(_) | Token::Func(_) => {
                        let right = self.parse_expr(Precedence::Product)?;
                        return Ok(Expr::Mul(box Expr::Ident(ident), box right));
                    }
                    _ => Ok(Expr::Ident(ident)),
                }
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
            }
            _ => Err(ParseError::UnknownAtom(format!("Unknown atom: {}", token))),
        }
    }

    /// Consume the current `Token` if it is what is expected, else return a
    /// `ParseError`.
    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        if expected == self.current_token {
            self.next_token()?;
            Ok(())
        } else {
            Err(ParseError::ExpectErr(format!(
                "Expected {}, got {}",
                expected, self.current_token
            )))
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
/// Defines the various errors that can occur during parsing.
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
        let expected_expr = Expr::Mul(
            box Expr::Num(1.0),
            box Expr::Add(box Expr::Num(1.0), box Expr::Num(1.0)),
        );
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
        let expected_expr = Expr::Mul(
            box Expr::Num(5.0),
            box Expr::Sin(box Expr::Add(box Expr::Num(1.0), box Expr::Num(1.0))),
        );
        assert_eq!(parser.parse().unwrap(), expected_expr);
    }

    #[test]
    fn constant() {
        let mut parser = Parser::new("pi").unwrap();
        let expected_expr = Expr::Ident("pi".to_string());
        assert_eq!(parser.parse().unwrap(), expected_expr);
    }

    #[test]
    fn assignment() {
        let mut parser = Parser::new("a = 5").unwrap();
        let expected_expr = Expr::Assign("a".to_string(), box Expr::Num(5.0));
        assert_eq!(parser.parse().unwrap(), expected_expr);
    }

    #[test]
    fn ord_of_ops() {
        let mut parser = Parser::new("1/(1+1)*1+2").unwrap();
        let expected_expr = Expr::Add(
            box Expr::Mul(
                box Expr::Div(
                    box Expr::Num(1.0),
                    box Expr::Add(box Expr::Num(1.0), box Expr::Num(1.0)),
                ),
                box Expr::Num(1.0),
            ),
            box Expr::Num(2.0),
        );
        assert_eq!(parser.parse().unwrap(), expected_expr);
    }
}
