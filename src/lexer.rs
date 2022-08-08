// Break up the sequences into recognizable tokens.
// 1 + 2 - sin(30) / (2 * 4)
// Number(1) Sum Number(2) Minus Function("sin") LeftParen Number(30) RightParen Division LeftParen
// Number(2) Multiplication Number(4)
#![allow(unused)]

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
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number(f64),
    Function(String),
    Operator(OperatorType),
    LeftParen,
    RightParen,
}

pub struct Lexer {
    source: String,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer { source }
    }

    pub fn scan(&mut self) -> Vec<TokenType> {
        let mut tokens: Vec<TokenType> = Vec::new();
        let mut it = self.source.chars().peekable();

        while let Some(token) = it.next() {
            match token {
                '+' => tokens.push(TokenType::Operator(OperatorType::Plus)),
                '-' => tokens.push(TokenType::Operator(OperatorType::Minus)),
                '*' => tokens.push(TokenType::Operator(OperatorType::Multiplication)),
                '/' => tokens.push(TokenType::Operator(OperatorType::Division)),
                '^' => tokens.push(TokenType::Operator(OperatorType::Pow)),
                '(' => tokens.push(TokenType::LeftParen),
                ')' => tokens.push(TokenType::RightParen),
                token if token.is_numeric() => {
                    let mut number = String::new();
                    number.push(token);

                    while let Some(&next) = it.peek() {
                        if next != '.' && !next.is_numeric() {
                            break;
                        }

                        if next == '.' || next.is_numeric() {
                            number.push(next);
                            it.next();
                        }
                    }

                    let number: f64 = number.parse().unwrap();
                    tokens.push(TokenType::Number(number));
                }
                token if token.is_alphabetic() => {
                    let mut function = String::new();

                    function.push(token);

                    while let Some(&next) = it.peek() {
                        if !next.is_alphabetic() && function.eq("log") && next.is_numeric() {
                            function.push(next);
                            it.next();
                        }

                        if !next.is_alphabetic() {
                            break;
                        }

                        if next.is_alphabetic() {
                            function.push(next);
                            it.next();
                        }
                    }

                    tokens.push(TokenType::Function(function));
                }
                _ => (),
            }
        }
        tokens
    }
}
