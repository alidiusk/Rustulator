//! # Ast
//!
//! Contains all the types required to construct an AST for any mathematical expression.

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
/// Defines all the different kinds of mathematical expressions
/// as recursive types.
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Abs(Box<Expr>),
    Floor(Box<Expr>),
    Log(Box<Expr>),
    Ln(Box<Expr>),
    Sin(Box<Expr>),
    Cos(Box<Expr>),
    Tan(Box<Expr>),
    Arcsin(Box<Expr>),
    Arccos(Box<Expr>),
    Arctan(Box<Expr>),
    Ident(String),
    // First can only actually be Ident
    Assign(String, Box<Expr>),
}

#[derive(Debug, PartialEq, Copy, Clone)]
/// Defines all the supported functions.
pub enum Func {
    Abs,
    Floor,
    Log,
    Ln,
    Sin,
    Cos,
    Tan,
    Arcsin,
    Arccos,
    Arctan,
}

/// Takes a &str and returns a corresponding function token,
/// if there is one. Otherwise it returns None.
pub fn get_function_token<'a>(s: &'a str) -> Option<Token> {
    match s {
        "abs" => Some(Token::Func(Func::Abs)),
        "floor" => Some(Token::Func(Func::Floor)),
        "log" => Some(Token::Func(Func::Log)),
        "ln" => Some(Token::Func(Func::Ln)),
        "sin" => Some(Token::Func(Func::Sin)),
        "cos" => Some(Token::Func(Func::Cos)),
        "tan" => Some(Token::Func(Func::Tan)),
        "arcsin" => Some(Token::Func(Func::Arcsin)),
        "arccos" => Some(Token::Func(Func::Arccos)),
        "arctan" => Some(Token::Func(Func::Arctan)),
        _ => None,
    }
}

#[derive(Debug, PartialEq, Clone)]
/// Defines all the valid Token types.
pub enum Token {
    Num(f64),
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    RParen,
    LParen,
    Equals,
    Func(Func),
    Ident(String),
    Eof,
}

impl Token {
    /// Returns the precedence of this Token.
    pub fn get_precedence(&self) -> Precedence {
        use self::Precedence::*;
        use self::Token::*;

        match *self {
            Add | Sub => Sum,
            Mul | Div => Product,
            Pow => Power,
            Func(_) => Function,
            Equals => Assign,
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
            Pow => write!(f, "Pow"),
            LParen => write!(f, "("),
            RParen => write!(f, ")"),
            Equals => write!(f, "="),
            // Implement Display for func
            Func(func) => write!(f, "{:?}", func),
            Ident(ref s) => write!(f, "{}", s),
            Eof => write!(f, "Eof"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
/// Defines all the Precedence levels, from lowest to highest.
pub enum Precedence {
    Lowest,
    Sum,
    Product,
    Power,
    Function,
    Prefix,
    Assign,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token() {
        let num = Token::Num(32.0);
        assert_eq!(Token::Num(32.0), num);
    }

    #[test]
    fn test_expr() {
        let addition = Expr::Add(box Expr::Num(32.0), box Expr::Num(16.0));
        assert_eq!(
            Expr::Add(box Expr::Num(32.0), box Expr::Num(16.0)),
            addition
        );
    }
}
