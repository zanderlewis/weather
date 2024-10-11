use num_bigint::BigInt;
use num_rational::BigRational;
use rayon::prelude::*;
use crate::ast::ASTNode;
use crate::token::Token;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use num_traits::ToPrimitive;

use crate::constants::*;

pub struct Interpreter {
    variables: HashMap<String, BigRational>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn execute(interpreter: Arc<Mutex<Self>>, node: ASTNode) {
        match node {
            ASTNode::Assignment(name, expr) => {
                let value = {
                    let mut interpreter = interpreter.lock().unwrap();
                    interpreter.evaluate(*expr)
                };
                let mut interpreter = interpreter.lock().unwrap();
                interpreter.variables.insert(name, value);
            }
            ASTNode::Print(expr) => {
                match *expr {
                    ASTNode::StringLiteral(value) => {
                        println!("{}", value);
                    }
                    _ => {
                        let value = {
                            let mut interpreter = interpreter.lock().unwrap();
                            interpreter.evaluate(*expr)
                        };
                        println!("{}", value.to_f64().unwrap());
                    }
                }
            }
            ASTNode::If(condition, then_branch, else_branch) => {
                let condition_result = {
                    let mut interpreter = interpreter.lock().unwrap();
                    interpreter.evaluate(*condition)
                };
                if condition_result != BigRational::from(BigInt::from(0)) {
                    Interpreter::execute(interpreter.clone(), *then_branch);
                } else if let Some(else_branch) = else_branch {
                    Interpreter::execute(interpreter.clone(), *else_branch);
                }
            }
            ASTNode::Block(nodes) => {
                nodes.into_par_iter().for_each(|node| {
                    Interpreter::execute(interpreter.clone(), node);
                });
            }
            _ => panic!("Unexpected AST node: {:?}", node),
        }
    }

    pub fn evaluate(&mut self, node: ASTNode) -> BigRational {
        match node {
            ASTNode::Number(value) => BigRational::from(value),
            ASTNode::Identifier(name) => {
                let value = self.variables.get(&name).expect("Undefined variable").clone();
                value
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
                let a = BigRational::new(BigInt::from(1727), BigInt::from(100));
                let b = BigRational::new(BigInt::from(2377), BigInt::from(10));
                let alpha = ((a.clone() * temp.clone()) / (b.clone() + temp.clone())) + BigRational::from_float(humidity.to_f64().unwrap().ln()).unwrap();
                (b * alpha.clone()) / (a - alpha)
            }
            ASTNode::FToC(fahrenheit) => {
                let fahrenheit = self.evaluate(*fahrenheit);
                (fahrenheit - BigRational::from_integer(BigInt::from(32))) * BigRational::new(BigInt::from(5), BigInt::from(9))
            }
            ASTNode::CToF(celsius) => {
                let celsius = self.evaluate(*celsius);
                (celsius * BigRational::new(BigInt::from(9), BigInt::from(5))) + BigRational::from_integer(BigInt::from(32))
            }
            ASTNode::CToK(celsius) => {
                let celsius = self.evaluate(*celsius);
                celsius + kelvin_constant()
            }
            ASTNode::KToC(kelvin) => {
                let kelvin = self.evaluate(*kelvin);
                kelvin - kelvin_constant()
            }
            ASTNode::FToK(fahrenheit) => {
                let fahrenheit = self.evaluate(*fahrenheit);
                (fahrenheit - BigRational::from_integer(BigInt::from(32))) * BigRational::new(BigInt::from(5), BigInt::from(9)) + kelvin_constant()
            }
            ASTNode::KToF(kelvin) => {
                let kelvin = self.evaluate(*kelvin);
                (kelvin - kelvin_constant()) * BigRational::new(BigInt::from(9), BigInt::from(5)) + BigRational::from_integer(BigInt::from(32))
            }
            ASTNode::Pi => pi_constant(),
            ASTNode::Kelvin => kelvin_constant(),
            ASTNode::RD => rd_constant(),
            ASTNode::CP => cp_constant(),
            ASTNode::P0 => p0_constant(),
            ASTNode::LV => lv_constant(),
            ASTNode::CW => cw_constant(),
            ASTNode::RhoAir => rho_air_constant(),
            ASTNode::RhoWater => rho_water_constant(),
            ASTNode::G => g_constant(),
            ASTNode::GreaterThan(left, right) => {
                let left_val = self.evaluate(*left);
                let right_val = self.evaluate(*right);
                if left_val > right_val { BigRational::from_integer(BigInt::from(1)) } else { BigRational::from_integer(BigInt::from(0)) }
            }
            ASTNode::LessThan(left, right) => {
                let left_val = self.evaluate(*left);
                let right_val = self.evaluate(*right);
                if left_val < right_val { BigRational::from_integer(BigInt::from(1)) } else { BigRational::from_integer(BigInt::from(0)) }
            }
            _ => panic!("Unexpected AST node: {:?}", node),
        }
    }

    pub fn interpret(&mut self, nodes: Vec<ASTNode>) {
        let interpreter = Arc::new(Mutex::new(Interpreter::new()));
        nodes.into_iter().for_each(|node| {
            Interpreter::execute(interpreter.clone(), node);
        });
    }
}