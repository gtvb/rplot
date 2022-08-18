use crate::lexer::{self, Associativity, OperatorType, TokenType};
use std::collections::VecDeque;

pub fn shunting_yard(tokens: &[TokenType]) -> Vec<&TokenType> {
    let mut output_queue = VecDeque::new();
    let mut operator_stack = Vec::new();

    for token_type in tokens.iter() {
        match token_type {
            TokenType::Number(_) | TokenType::Variable => output_queue.push_back(token_type),
            TokenType::Function(_) => operator_stack.push(token_type),
            TokenType::LeftParen => operator_stack.push(token_type),
            TokenType::Operator(operator_type) => {
                while let Some(stack_operator) = operator_stack.last() {
                    let stack_operator_type = match stack_operator {
                        TokenType::Operator(val) => val,
                        _ => break,
                    };

                    if stack_operator_type.precedence() >= operator_type.precedence()
                        && operator_type.associativity() == Associativity::Left
                    {
                        output_queue.push_back(stack_operator);
                        operator_stack.pop();
                    } else {
                        break;
                    }
                }

                operator_stack.push(token_type);
            }
            TokenType::RightParen => {
                while let Some(stack_operator) = operator_stack.last() {
                    match stack_operator {
                        TokenType::LeftParen => break,
                        _ => {
                            output_queue.push_back(stack_operator);
                            operator_stack.pop();
                        }
                    }
                }

                operator_stack.pop();

                if let Some(stack_operator) = operator_stack.last() {
                    match stack_operator {
                        TokenType::Function(_) => {
                            output_queue.push_back(stack_operator);
                            operator_stack.pop();
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    while let Some(stack_operator) = operator_stack.pop() {
        output_queue.push_back(stack_operator);
    }

    output_queue.into()
}

pub fn substitute(expression: &str, value: f64) -> String {
    expression.replace("@", &value.to_string())
}

pub fn eval_with_variable(expression: &str, variable_substitute: f64) -> f64 {
    eval(&substitute(expression, variable_substitute))
}

pub fn eval_with_variables<'a>(expression: &'a str, variable_substitutes: &[f64]) -> Vec<f64> {
    let mut results = Vec::new();
    for vs in variable_substitutes {
        results.push(eval(&substitute(expression, *vs)));
    }
    results
}

pub fn eval(expression: &str) -> f64 {
    let mut stack: Vec<f64> = Vec::new();
    let tokens = lexer::scan(expression);
    let tokens = shunting_yard(&tokens);

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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn sum() {
//         assert_eq!(eval("1 + 1"), 2.0);
//         assert_eq!(eval("3 + 5"), 8.0);
//         assert_eq!(eval("44 + 11"), 55.0);
//         assert_eq!(eval("1001.5 + 2.5"), 1004.0);
//     }

//     #[test]
//     fn subtraction() {
//         assert_eq!(eval("1 - 1"), 0.0);
//         assert_eq!(eval("1000 - 200"), 800.0);
//         assert_eq!(eval("3 - 4.5"), -1.5);
//         assert_eq!(eval("555 - 200"), 355.0);
//         assert_eq!(eval("-600 - 300"), -900.0);
//     }

//     #[test]
//     fn multiplication() {
//         assert_eq!(eval("5 * 0"), 0.0);
//         assert_eq!(eval("9 * 6"), 54.0);
//         assert_eq!(eval("-4 * 2.4"), -9.6);
//         assert_eq!(eval("555 - 200"), 355.0);
//     }

//     #[test]
//     fn division() {
//         assert_eq!(eval("25 / 5"), 5.0);
//         assert_eq!(eval("1 / 2"), 0.5);
//         assert_eq!(eval("-8 / 4"), -2.0);
//         assert_eq!(eval("-99 / -3"), 33.0);
//     }

//     #[test]
//     fn power() {
//         assert_eq!(eval("2^3"), 8.0);
//         assert_eq!(eval("999^0"), 1.0);
//         assert_eq!(eval("2^2^2"), 16.0);
//         assert_eq!(eval("5^3"), 125.0);
//     }

//     #[test]
//     fn functions() {
//         assert_eq!(eval("sin(0) * 4"), 0.0);
//         assert_eq!(eval("cos(0) * -9.99"), -9.99);
//         assert_eq!(eval("sqrt(2 + 2)"), 2.0);
//         assert_eq!(eval("log2(8)"), 3.0);
//     }

//     #[test]
//     fn mix() {
//         assert_eq!(eval("4^2 - (1 - 5)^2^1"), 0.0);
//         assert_eq!(eval("4 + 18 / (9 - 3)"), 7.0);
//         assert_eq!(eval("(2*4) / (2^2 + 4^2)"), 0.4);
//         // assert_eq!(eval(""),);
//         // assert_eq!(eval(""),);
//         // assert_eq!(eval(""),);
//         // assert_eq!(eval(""),);
//         // assert_eq!(eval(""),);
//         // assert_eq!(eval(""),);
//     }
// }
