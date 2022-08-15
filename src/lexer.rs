use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Associativity {
    Right,
    Left,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperatorType {
    Plus,
    Minus,
    Multiplication,
    Division,
    Pow,
}

impl OperatorType {
    pub fn precedence(&self) -> u8 {
        use OperatorType::*;
        match self {
            Plus => 1,
            Minus => 1,
            Multiplication => 2,
            Division => 2,
            Pow => 3,
        }
    }

    pub fn associativity(&self) -> Associativity {
        use OperatorType::*;
        match self {
            Pow => Associativity::Right,
            _ => Associativity::Left,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number(f64),
    Function(String),
    Operator(OperatorType),
    Variable,
    LeftParen,
    RightParen,
}

pub fn scan(expression: &str) -> Vec<TokenType> {
    let mut tokens: Vec<TokenType> = Vec::new();
    let expression = String::from(expression);
    let mut it = expression.chars().peekable();

    while let Some(token) = it.next() {
        match token {
            '+' => tokens.push(TokenType::Operator(OperatorType::Plus)),
            '-' => {
                if it.peek().unwrap().is_numeric() {
                    tokens.push(TokenType::Number(number(&mut it, token)));
                } else {
                    tokens.push(TokenType::Operator(OperatorType::Minus));
                }
            }
            '*' => tokens.push(TokenType::Operator(OperatorType::Multiplication)),
            '/' => tokens.push(TokenType::Operator(OperatorType::Division)),
            '^' => tokens.push(TokenType::Operator(OperatorType::Pow)),
            '(' => tokens.push(TokenType::LeftParen),
            ')' => tokens.push(TokenType::RightParen),
            '$' => tokens.push(TokenType::Variable),
            token if token.is_numeric() => tokens.push(TokenType::Number(number(&mut it, token))),
            token if token.is_alphabetic() => {
                tokens.push(TokenType::Function(function(&mut it, token)))
            }
            _ => (),
        }
    }
    tokens
}

fn number(iterator: &mut Peekable<Chars>, first: char) -> f64 {
    let mut number = String::new();
    number.push(first);

    while let Some(&next) = iterator.peek() {
        if next != '.' && !next.is_numeric() {
            break;
        }

        if next == '.' || next.is_numeric() {
            number.push(next);
            iterator.next();
        }
    }

    number.parse::<f64>().unwrap()
}

fn function(iterator: &mut Peekable<Chars>, first: char) -> String {
    let mut function = String::new();
    function.push(first);

    while let Some(&next) = iterator.peek() {
        if !next.is_alphabetic() && function.eq("log") && next.is_numeric() {
            function.push(next);
            iterator.next();
        }

        if !next.is_alphabetic() {
            break;
        }

        if next.is_alphabetic() {
            function.push(next);
            iterator.next();
        }
    }

    function
}
