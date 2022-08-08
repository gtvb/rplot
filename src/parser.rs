use crate::lexer::{Lexer, OperatorType, TokenType};
use std::collections::VecDeque;

pub struct Parser<'a> {
    tokens: Vec<TokenType>,
    output_queue: VecDeque<&'a TokenType>,
    operator_stack: Vec<&'a TokenType>,
}

impl<'a> Parser<'a> {
    pub fn new(expression: &'a str) -> Parser {
        let mut lexer = Lexer::new(String::from(expression));
        let tokens = lexer.scan();

        Parser {
            tokens,
            output_queue: VecDeque::new(),
            operator_stack: Vec::new(),
        }
    }

    pub fn shunting_yard(&'a mut self) -> Vec<&'a TokenType> {
        for token_type in self.tokens.iter() {
            match token_type {
                TokenType::Number(_) => self.output_queue.push_back(token_type),
                TokenType::Function(_) => self.operator_stack.push(token_type),
                TokenType::LeftParen => self.operator_stack.push(token_type),
                TokenType::Operator(operator_type) => {
                    while let Some(stack_operator) = self.operator_stack.last() {
                        let stack_operator_type = match stack_operator {
                            TokenType::Operator(val) => val,
                            _ => break,
                        };

                        if stack_operator_type.precedence() >= operator_type.precedence() {
                            self.output_queue.push_back(stack_operator);
                            self.operator_stack.pop();
                        } else {
                            break;
                        }
                    }

                    self.operator_stack.push(token_type);
                }
                TokenType::RightParen => {
                    while let Some(stack_operator) = self.operator_stack.last() {
                        match stack_operator {
                            TokenType::LeftParen => break,
                            _ => {
                                self.output_queue.push_back(stack_operator);
                                self.operator_stack.pop();
                            }
                        }
                    }

                    self.operator_stack.pop();

                    if let Some(stack_operator) = self.operator_stack.pop() {
                        self.output_queue.push_back(stack_operator);
                    }
                }
            }
        }

        self.operator_stack
            .iter()
            .for_each(|op| self.output_queue.push_back(op));

        self.output_queue.clone().into()
    }

    pub fn eval(&'a mut self) -> f64 {
        let mut stack: Vec<f64> = Vec::new();
        let tokens = self.shunting_yard();

        for token in tokens {
            match token {
                TokenType::Number(number) => stack.push(*number),
                TokenType::Operator(operator) => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();

                    match operator {
                        OperatorType::Plus => stack.push(lhs + rhs),
                        OperatorType::Minus => stack.push(lhs - rhs),
                        OperatorType::Multiplication => stack.push(lhs * rhs),
                        OperatorType::Division => stack.push(lhs / rhs),
                        OperatorType::Pow => stack.push(lhs.powf(rhs)),
                    }
                }
                TokenType::Function(func) => {
                    println!("{}", func);
                    match func.as_str() {
                        "sin" => {
                            let arg = stack.pop().unwrap();
                            stack.push(arg.sin());
                        },
                        "cos" => {
                            let arg = stack.pop().unwrap();
                            stack.push(arg.cos());
                        },
                        "tan" => {
                            let arg = stack.pop().unwrap();
                            stack.push(arg.tan());
                        },
                        "sqrt" => {
                            let arg = stack.pop().unwrap();
                            stack.push(arg.sqrt());
                        },
                        "log2" => {
                            let arg = stack.pop().unwrap();
                            stack.push(arg.log2());
                        },
                        _ => unimplemented!(),
                    }
                },
                _ => break,
            }
        }

        stack.pop().unwrap()
    }
}
