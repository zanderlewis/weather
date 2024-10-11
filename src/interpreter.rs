use crate::ast::ASTNode;
use crate::token::Token;
use std::collections::HashMap;

pub struct Interpreter {
    variables: HashMap<String, f64>,
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
                if self.evaluate(*condition) != 0.0 {
                    self.execute(*then_branch);
                } else if let Some(else_branch) = else_branch {
                    self.execute(*else_branch);
                }
            }
            _ => panic!("Unexpected AST node: {:?}", node),
        }
    }

    pub fn evaluate(&mut self, node: ASTNode) -> f64 {
        match node {
            ASTNode::Number(value) => value,
            ASTNode::Identifier(name) => *self.variables.get(&name).expect("Undefined variable"),
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
                let alpha = ((a * temp) / (b + temp)) + humidity.ln();
                (b * alpha) / (a - alpha)
            }
            ASTNode::FToC(fahrenheit) => {
                let fahrenheit = self.evaluate(*fahrenheit);
                (fahrenheit - 32.0) * 5.0 / 9.0
            }
            ASTNode::CToF(celsius) => {
                let celsius = self.evaluate(*celsius);
                (celsius * 9.0 / 5.0) + 32.0
            }
            ASTNode::GreaterThan(left, right) => {
                let left_val = self.evaluate(*left);
                let right_val = self.evaluate(*right);
                if left_val > right_val { 1.0 } else { 0.0 }
            }
            ASTNode::LessThan(left, right) => {
                let left_val = self.evaluate(*left);
                let right_val = self.evaluate(*right);
                if left_val < right_val { 1.0 } else { 0.0 }
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