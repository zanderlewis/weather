use num_bigint::BigInt;

use crate::ast::ASTNode;
use crate::token::Token;
use std::collections::HashMap;
use num_traits::ToPrimitive;

pub struct Interpreter {
    variables: HashMap<String, BigInt>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn execute(&mut self, node: ASTNode) {
        match node {
            ASTNode::Assignment(name, expr) => {
                let value = self.evaluate(*expr);
                self.variables.insert(name, value);
            }
            ASTNode::Print(expr) => {
                match *expr {
                    ASTNode::StringLiteral(value) => {
                        println!("{}", value);
                    }
                    _ => {
                        let value = self.evaluate(*expr);
                        println!("{}", value);
                    }
                }
            }
            ASTNode::If(condition, then_branch, else_branch) => {
                if self.evaluate(*condition) != BigInt::from(0) {
                    self.execute(*then_branch);
                } else if let Some(else_branch) = else_branch {
                    self.execute(*else_branch);
                }
            }
            ASTNode::Block(nodes) => {
                for node in nodes {
                    self.execute(node);
                }
            }
            _ => panic!("Unexpected AST node: {:?}", node),
        }
    }

    pub fn evaluate(&mut self, node: ASTNode) -> BigInt {
        match node {
            ASTNode::Number(value) => value,
            ASTNode::Identifier(name) => {
                let value = self.variables.get(&name).expect("Undefined variable").clone();
                BigInt::from(value)
            },
            ASTNode::BinaryOp(left, op, right) => {
                let left_val = self.evaluate(*left);
                let right_val = self.evaluate(*right);
                match op {
                    Token::Plus => left_val + right_val,
                    Token::Minus => left_val - right_val,
                    Token::Star => left_val * right_val,
                    Token::Slash => left_val / right_val,
                    _ => panic!("Unexpected operator: {:?}", op),
                }
            }
            ASTNode::DewPoint(temp, humidity) => {
                let temp = self.evaluate(*temp);
                let humidity = self.evaluate(*humidity);
                // Dew point calculation formula
                let a = 17.27;
                let b = 237.7;
                let temp_f64 = temp.to_f64().unwrap();
                let humidity_f64 = humidity.to_f64().unwrap();
                let alpha = ((a * temp_f64) / (b + temp_f64)) + humidity_f64.ln();
                let dew_point = (b * alpha) / (a - alpha);
                BigInt::from(dew_point as i64)
            }
            ASTNode::FToC(fahrenheit) => {
                let fahrenheit = self.evaluate(*fahrenheit);
                (fahrenheit - num_bigint::ToBigInt::to_bigint(&32).unwrap()) * num_bigint::ToBigInt::to_bigint(&5).unwrap() / num_bigint::ToBigInt::to_bigint(&9).unwrap()
            }
            ASTNode::CToF(celsius) => {
                let celsius = self.evaluate(*celsius);
                (celsius * num_bigint::ToBigInt::to_bigint(&9).unwrap() / num_bigint::ToBigInt::to_bigint(&5).unwrap()) + num_bigint::ToBigInt::to_bigint(&32).unwrap()
            }
            ASTNode::GreaterThan(left, right) => {
                let left_val = self.evaluate(*left);
                let right_val = self.evaluate(*right);
                if left_val > right_val { BigInt::from(1) } else { BigInt::from(0) }
            }
            ASTNode::LessThan(left, right) => {
                let left_val = self.evaluate(*left);
                let right_val = self.evaluate(*right);
                if left_val < right_val { BigInt::from(1) } else { BigInt::from(0) }
            }
            _ => panic!("Unexpected AST node: {:?}", node),
        }
    }

    pub fn interpret(&mut self, nodes: Vec<ASTNode>) {
        for node in nodes {
            self.execute(node);
        }
    }
}