use num_bigint::BigInt;
use num_rational::BigRational;
use crate::ast::ASTNode;
use crate::token::Token;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use num_traits::ToPrimitive;
use num_complex::Complex;

use crate::constants::*;

pub struct Interpreter {
    variables: HashMap<String, Complex<BigRational>>,
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
                guard.variables.insert(name, value.into());
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
                if condition_result != BigRational::from(BigInt::from(0)).into() {
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
                        variables.insert(param.clone(), value.into());
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

    pub fn evaluate(&mut self, node: ASTNode) -> Complex<BigRational> {
        match node {
            ASTNode::Float(value) => BigRational::from_float(value.to_f64().unwrap()).unwrap().into(),
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
                        if left_val.re > right_val.re { BigRational::from_integer(BigInt::from(1)).into() } else { BigRational::from_integer(BigInt::from(0)).into() }
                    }
                    Token::LessThan => {
                        if left_val.re < right_val.re { BigRational::from_integer(BigInt::from(1)).into() } else { BigRational::from_integer(BigInt::from(0)).into() }
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
                let temp_re = temp.re.clone();
                let alpha = ((a.clone() * temp_re.clone()) / (b.clone() + temp_re)) + BigRational::from_float(humidity.to_f64().unwrap().ln()).unwrap();
                ((b * alpha.clone()) / (a - alpha)).into()
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
                if qubit == BigRational::from_integer(BigInt::from(0)).into() {
                    BigRational::from_integer(BigInt::from(1)).into()
                } else {
                    BigRational::from_integer(BigInt::from(0)).into()
                }
            }
            ASTNode::PauliY(qubit) => {
                let qubit = self.evaluate(*qubit);
                if qubit == BigRational::from_integer(BigInt::from(0)).into() {
                    BigRational::from_integer(BigInt::from(1)).into()
                } else {
                    BigRational::from_integer(BigInt::from(-1)).into()
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
                if control == BigRational::from_integer(BigInt::from(1)).into() {
                    if target == BigRational::from_integer(BigInt::from(0)).into() {
                        BigRational::from_integer(BigInt::from(1)).into()
                    } else {
                        BigRational::from_integer(BigInt::from(0)).into()
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
                    result = (result * BigRational::from_integer(BigInt::from(2))) + state.re.clone();
                }
                result.into()
            }
            ASTNode::MeasureQubit(qubit) => {
                let qubit = self.evaluate(*qubit);
                if qubit == BigRational::from_integer(BigInt::from(0)).into() {
                    BigRational::from_integer(BigInt::from(0)).into()
                } else {
                    BigRational::from_integer(BigInt::from(1)).into()
                }
            }
            ASTNode::ResetQubit(qubit) => {
                let _ = self.evaluate(*qubit);
                BigRational::from_integer(BigInt::from(0)).into()
            }
            ASTNode::Toffoli(control1, control2, target) => {
                let control1 = self.evaluate(*control1);
                let control2 = self.evaluate(*control2);
                let target = self.evaluate(*target);
                if control1 == BigRational::from_integer(BigInt::from(1)).into() && control2 == BigRational::from_integer(BigInt::from(1)).into() {
                    if target == BigRational::from_integer(BigInt::from(0)).into() {
                        BigRational::from_integer(BigInt::from(1)).into()
                    } else {
                        BigRational::from_integer(BigInt::from(0)).into()
                    }
                } else {
                    target
                }
            }
            ASTNode::SWAP(qubit1_node, qubit2_node) => {
                let qubit1 = self.evaluate(*qubit1_node);
                let qubit2 = self.evaluate(*qubit2_node);
                &qubit1 + &qubit2 - (&qubit1 * &qubit2 * BigRational::from_integer(BigInt::from(2)))
            }
            ASTNode::Phase(qubit) => {
                let qubit = self.evaluate(*qubit);
                qubit * BigRational::from_integer(BigInt::from(-1))
            }
            ASTNode::SGate(qubit) => {
                // S gate applies a phase shift of π/2 (multiplication by i)
                let q = self.evaluate(*qubit);
                q * Complex::new(BigRational::from_integer(<BigInt as num_traits::Zero>::zero()), BigRational::from_integer(<BigInt as num_traits::One>::one()))
            }
            
            ASTNode::TGate(qubit) => {
                // T gate applies a phase shift of π/4
                let q = self.evaluate(*qubit);
                let one = BigRational::from_integer(<BigInt as num_traits::One>::one());
                let sqrt_two = BigRational::from_float(2f64.sqrt()).unwrap();
                let sqrt_two_over_two = &one / &sqrt_two;
                let phase = Complex::new(sqrt_two_over_two.clone(), sqrt_two_over_two);
                q * phase
            }
            ASTNode::Fredkin(control, target1, target2) => {
                let control = self.evaluate(*control);
                let target1 = self.evaluate(*target1);
                let target2 = self.evaluate(*target2);
                if control == BigRational::from_integer(BigInt::from(1)).into() {
                    target2
                } else {
                    target1
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
                                                    imported_interpreter.lock().unwrap().evaluate(node).re
                                                }).collect();
                results.last().cloned().unwrap_or_else(|| BigRational::from_integer(BigInt::from(0))).into()
            }
            ASTNode::Pi => pi_constant().into(),
            ASTNode::Kelvin => kelvin_constant().into(),
            ASTNode::RD => rd_constant().into(),
            ASTNode::CP => cp_constant().into(),
            ASTNode::P0 => p0_constant().into(),
            ASTNode::LV => lv_constant().into(),
            ASTNode::CW => cw_constant().into(),
            ASTNode::RhoAir => rho_air_constant().into(),
            ASTNode::RhoWater => rho_water_constant().into(),
            ASTNode::G => g_constant().into(),
            ASTNode::GreaterThan(left, right) => {
                let left_val = self.evaluate(*left);
                let right_val = self.evaluate(*right);
                if left_val.re > right_val.re { BigRational::from_integer(BigInt::from(1)).into() } else { BigRational::from_integer(BigInt::from(0)).into() }
            }
            ASTNode::LessThan(left, right) => {
                let left_val = self.evaluate(*left);
                let right_val = self.evaluate(*right);
                if left_val.re < right_val.re { BigRational::from_integer(BigInt::from(1)).into() } else { BigRational::from_integer(BigInt::from(0)).into() }
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