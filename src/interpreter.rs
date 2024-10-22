use num_bigint::BigInt;
use num_rational::BigRational;
use crate::ast::ASTNode;
use crate::token::Token;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use num_traits::ToPrimitive;

use crate::constants::*;

pub struct Interpreter {
    variables: HashMap<String, BigRational>,
    functions: HashMap<String, ASTNode>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn execute(interpreter: Arc<Mutex<Self>>, node: ASTNode) {
        match node {
            ASTNode::Assignment(name, expr) => {
                let value = {
                    let mut guard = interpreter.lock().unwrap();
                    guard.evaluate(*expr)
                };
                let mut guard = interpreter.lock().unwrap();
                guard.variables.insert(name, value);
            }
            ASTNode::Print(expr) => {
                match *expr {
                    ASTNode::StringLiteral(value) => {
                        println!("{}", value);
                    }
                    _ => {
                        let value = {
                            let mut guard = interpreter.lock().unwrap();
                            guard.evaluate(*expr)
                        };
                        println!("{}", value.to_f64().unwrap());
                    }
                }
            }
            ASTNode::If(condition, then_branch, else_branch) => {
                let condition_result = {
                    let mut guard = interpreter.lock().unwrap();
                    guard.evaluate(*condition)
                };
                if condition_result != BigRational::from(BigInt::from(0)) {
                    Interpreter::execute(interpreter.clone(), *then_branch);
                } else if let Some(else_branch) = else_branch {
                    Interpreter::execute(interpreter.clone(), *else_branch);
                }
            }
            ASTNode::Block(nodes) => {
                for node in nodes {
                    Interpreter::execute(interpreter.clone(), node);
                }
            }
            ASTNode::Function(name, params, body) => {
                let mut guard = interpreter.lock().unwrap();
                let name_clone = name.clone();
                guard.functions.insert(name_clone, ASTNode::Function(name, params.clone(), body.clone()));
            }
            ASTNode::Call(name, args) => {
                let mut guard = interpreter.lock().unwrap();
                let function = guard.functions.get(&name).expect("Undefined function").clone();
                if let ASTNode::Function(_, params, body) = function {
                    let mut variables = guard.variables.clone();
                    for (param, arg) in params.iter().zip(args.iter()) {
                        let value = guard.evaluate(arg.clone());
                        variables.insert(param.clone(), value);
                    }
                    let interpreter = Interpreter {
                        variables,
                        functions: guard.functions.clone(),
                    };
                    Interpreter::execute(Arc::new(Mutex::new(interpreter)), *body);
                } else {
                    panic!("Expected function, got {:?}", function);
                }
            }
            ASTNode::Import(module_name) => {
                // Load and parse the module file
                let module_content = std::fs::read_to_string(module_name.clone()).expect("Failed to read module file");
                let lexer = crate::lexer::Lexer::new(module_content);
                let mut parser = crate::parser::Parser::new(lexer);
                let nodes = parser.parse();

                // Execute the parsed nodes
                let imported_interpreter = Arc::new(Mutex::new(Interpreter::new()));
                for node in nodes {
                    Interpreter::execute(imported_interpreter.clone(), node);
                }

                // Merge imported functions into the current interpreter
                let imported_guard = imported_interpreter.lock().unwrap();
                let functions_to_merge: Vec<_> = imported_guard.functions.clone().into_iter().collect();
                drop(imported_guard); // Release the lock before re-acquiring it
                let mut guard = interpreter.lock().unwrap();
                for (name, function) in functions_to_merge {
                    guard.functions.insert(name, function);
                }
            }
            _ => panic!("Unexpected AST node: {:?}", node),
        }
    }

    pub fn evaluate(&mut self, node: ASTNode) -> BigRational {
        match node {
            ASTNode::Float(value) => BigRational::from_float(value.to_f64().unwrap()).unwrap(),
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
                    Token::GreaterThan => {
                        if left_val > right_val { BigRational::from_integer(BigInt::from(1)) } else { BigRational::from_integer(BigInt::from(0)) }
                    }
                    Token::LessThan => {
                        if left_val < right_val { BigRational::from_integer(BigInt::from(1)) } else { BigRational::from_integer(BigInt::from(0)) }
                    }
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
            ASTNode::PauliX(qubit) => {
                let qubit = self.evaluate(*qubit);
                if qubit == BigRational::from_integer(BigInt::from(0)) {
                    BigRational::from_integer(BigInt::from(1))
                } else {
                    BigRational::from_integer(BigInt::from(0))
                }
            }
            ASTNode::PauliY(qubit) => {
                let qubit = self.evaluate(*qubit);
                if qubit == BigRational::from_integer(BigInt::from(0)) {
                    BigRational::from_integer(BigInt::from(1))
                } else {
                    BigRational::from_integer(BigInt::from(-1))
                }
            }
            ASTNode::PauliZ(qubit) => {
                let qubit = self.evaluate(*qubit);
                qubit
            }
            ASTNode::Hadamard(qubit) => {
                let qubit = self.evaluate(*qubit);
                (qubit + BigRational::from_integer(BigInt::from(1))) / BigRational::from_integer(BigInt::from(2))
            }
            ASTNode::CNot(control, target) => {
                let control = self.evaluate(*control);
                let target = self.evaluate(*target);
                if control == BigRational::from_integer(BigInt::from(1)) {
                    if target == BigRational::from_integer(BigInt::from(0)) {
                        BigRational::from_integer(BigInt::from(1))
                    } else {
                        BigRational::from_integer(BigInt::from(0))
                    }
                } else {
                    target
                }
            }
            // Create number of qubits with the given state
            ASTNode::Qubit(state, num_qubits) => {
                let state = self.evaluate(*state);
                let num_qubits = self.evaluate(*num_qubits);
                let mut result = BigRational::from_integer(BigInt::from(0));
                for _ in 0..num_qubits.to_usize().unwrap() {
                    result = (result * BigRational::from_integer(BigInt::from(2))) + &state;
                }
                result
            }
            ASTNode::MeasureQubit(qubit) => {
                let qubit = self.evaluate(*qubit);
                if qubit == BigRational::from_integer(BigInt::from(0)) {
                    BigRational::from_integer(BigInt::from(0))
                } else {
                    BigRational::from_integer(BigInt::from(1))
                }
            }
            ASTNode::Call(name, args) => {
                let function = self.functions.get(&name).expect("Undefined function").clone();
                if let ASTNode::Function(_, params, body) = function {
                    let mut variables = self.variables.clone();
                    for (param, arg) in params.iter().zip(args.iter()) {
                        let value = self.evaluate(arg.clone());
                        variables.insert(param.clone(), value);
                    }
                    let mut interpreter = Interpreter {
                        variables,
                        functions: self.functions.clone(),
                    };
                    interpreter.evaluate(*body)
                } else {
                    panic!("Expected function, got {:?}", function);
                }
            }
            ASTNode::Import(module_name) => {
                // Load and parse the module file
                let module_content = std::fs::read_to_string(module_name).expect("Failed to read module file");
                let lexer = crate::lexer::Lexer::new(module_content);
                let mut parser = crate::parser::Parser::new(lexer);
                let nodes = parser.parse();

                // Execute the parsed nodes
                let imported_interpreter = Arc::new(Mutex::new(Interpreter::new()));
                let results: Vec<BigRational> = nodes.into_iter().map(|node| {
                                    Interpreter::execute(imported_interpreter.clone(), node.clone());
                                    imported_interpreter.lock().unwrap().evaluate(node)
                                }).collect();
                results.last().cloned().unwrap_or_else(|| BigRational::from_integer(BigInt::from(0)))
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