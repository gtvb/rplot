// Break up the sequences into recognizable tokens.
// 1 + 2 - sin(30) / (2 * 4)
// Number(1) Sum Number(2) Minus Function("sin") LeftParen Number(30) RightParen Division LeftParen
// Number(2) Multiplication Number(4)
#![allow(unused)]

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
    LeftParen,
    RightParen,
}

pub struct Lexer<'a> {
    source: String,
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.into(),
            chars: source.chars().peekable(),
        }
    }

    pub fn scan(&mut self) -> Vec<TokenType> {
        let mut tokens: Vec<TokenType> = Vec::new();

        while let Some(token) = self.chars.next() {
            match token {
                '+' => tokens.push(TokenType::Operator(OperatorType::Plus)),
                '-' => {
                    if self.chars.peek().unwrap().is_numeric() {
                        tokens.push(TokenType::Number(self.number(token)));
                        continue;
                    }
                    tokens.push(TokenType::Operator(OperatorType::Minus))
                },
                '*' => tokens.push(TokenType::Operator(OperatorType::Multiplication)),
                '/' => tokens.push(TokenType::Operator(OperatorType::Division)),
                '^' => tokens.push(TokenType::Operator(OperatorType::Pow)),
                '(' => tokens.push(TokenType::LeftParen),
                ')' => tokens.push(TokenType::RightParen),
                token if token.is_numeric() => tokens.push(TokenType::Number(self.number(token))),
                token if token.is_alphabetic() => tokens.push(TokenType::Function(self.function(token))),
                _ => (),
            }
        }
        tokens
    }

    fn number(&mut self, first: char) -> f64 {
        let mut number = String::new();
        number.push(first);

        while let Some(&next) = self.chars.peek() {
            if next != '.' && !next.is_numeric() {
                break;
            }

            if next == '.' || next.is_numeric() {
                number.push(next);
                self.chars.next();
            }
        }

        number.parse::<f64>().unwrap()
    }

    fn function(&mut self, first: char) -> String {
        let mut function = String::new();
        function.push(first);

        while let Some(&next) = self.chars.peek() {
            if !next.is_alphabetic() && function.eq("log") && next.is_numeric() {
                function.push(next);
                self.chars.next();
            }

            if !next.is_alphabetic() {
                break;
            }

            if next.is_alphabetic() {
                function.push(next);
                self.chars.next();
            }
        }

        function
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
