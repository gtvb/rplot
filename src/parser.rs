#![allow(unused)]

use crate::lexer::{Associativity, Lexer, OperatorType, TokenType};
use std::collections::VecDeque;

pub struct Parser<'a> {
    // TODO: make this a reference, tokens won't be modified
    tokens: Vec<TokenType>,
    output_queue: VecDeque<&'a TokenType>,
    operator_stack: Vec<&'a TokenType>,
}

impl<'a> Parser<'a> {
    pub fn new(expression: &'a str) -> Parser {
        let mut lexer = Lexer::new(expression);
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

                        if stack_operator_type.precedence() >= operator_type.precedence()
                            && operator_type.associativity() == Associativity::Left
                        {
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

                    if let Some(stack_operator) = self.operator_stack.last() {
                        match stack_operator {
                            TokenType::Function(_) =>  {
                                self.output_queue.push_back(stack_operator);
                                self.operator_stack.pop();
                            }
                            _ => (),
                        }
                    }
                }
            }
        }

        while let Some(stack_operator) = self.operator_stack.pop() {
            self.output_queue.push_back(stack_operator);
        }

        println!("STACK: {}, QUEUE: {}", self.operator_stack.len(), self.output_queue.len());

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
                TokenType::Function(func) => match func.as_str() {
                    "sin" => {
                        let arg = stack.pop().unwrap();
                        stack.push(arg.sin());
                    }
                    "cos" => {
                        let arg = stack.pop().unwrap();
                        stack.push(arg.cos());
                    }
                    "tan" => {
                        let arg = stack.pop().unwrap();
                        stack.push(arg.tan());
                    }
                    "sqrt" => {
                        let arg = stack.pop().unwrap();
                        stack.push(arg.sqrt());
                    }
                    "log2" => {
                        let arg = stack.pop().unwrap();
                        stack.push(arg.log2());
                    }
                    _ => unimplemented!(),
                },
                _ => break,
            }
        }

        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval_expression(input: &str) -> f64 {
        let mut parser = Parser::new(input);
        parser.eval()
    }

    #[test]
    fn sum() {
        assert_eq!(eval_expression("1 + 1"), 2.0);
        assert_eq!(eval_expression("3 + 5"), 8.0);
        assert_eq!(eval_expression("44 + 11"), 55.0);
        assert_eq!(eval_expression("1001.5 + 2.5"), 1004.0);
    }

    #[test]
    fn subtraction() {
        assert_eq!(eval_expression("1 - 1"), 0.0);
        assert_eq!(eval_expression("1000 - 200"), 800.0);
        assert_eq!(eval_expression("3 - 4.5"), -1.5);
        assert_eq!(eval_expression("555 - 200"), 355.0);
        assert_eq!(eval_expression("-600 - 300"), -900.0);
    }

    #[test]
    fn multiplication() {
        assert_eq!(eval_expression("5 * 0"), 0.0);
        assert_eq!(eval_expression("9 * 6"), 54.0);
        assert_eq!(eval_expression("-4 * 2.4"), -9.6);
        assert_eq!(eval_expression("555 - 200"), 355.0);
    }

    #[test]
    fn division() {
        assert_eq!(eval_expression("25 / 5"), 5.0);
        assert_eq!(eval_expression("1 / 2"), 0.5);
        assert_eq!(eval_expression("-8 / 4"), -2.0);
        assert_eq!(eval_expression("-99 / -3"), 33.0);
    }

    #[test]
    fn power() {
        assert_eq!(eval_expression("2^3"), 8.0);
        assert_eq!(eval_expression("999^0"), 1.0);
        assert_eq!(eval_expression("2^2^2"), 16.0);
        assert_eq!(eval_expression("5^3"), 125.0);
    }

    #[test]
    fn functions() {
        assert_eq!(eval_expression("sin(0) * 4"), 0.0);
        assert_eq!(eval_expression("cos(0) * -9.99"), -9.99);
        assert_eq!(eval_expression("sqrt(2 + 2)"), 2.0);
        assert_eq!(eval_expression("log2(8)"), 3.0);
    }


    #[test]
    fn mix() {
        assert_eq!(eval_expression("4^2 - (1 - 5)^2^1"), 0.0);
        assert_eq!(eval_expression("4 + 18 / (9 - 3)"), 7.0);
        assert_eq!(eval_expression("(2*4) / (2^2 + 4^2)"), 0.4);
        // assert_eq!(eval_expression(""),);
        // assert_eq!(eval_expression(""),);
        // assert_eq!(eval_expression(""),);
        // assert_eq!(eval_expression(""),);
        // assert_eq!(eval_expression(""),);
        // assert_eq!(eval_expression(""),);
    }
}
