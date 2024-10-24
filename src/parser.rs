use crate::lexer::Lexer;
use crate::token::Token;
use crate::ast::ASTNode;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    line: usize,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let line = lexer.line;
        Self { lexer, current_token , line }
    }

    fn consume(&mut self, expected: Token) {
        if self.current_token == expected {
            self.current_token = self.lexer.next_token();
            self.line = self.lexer.line;
        } else {
            panic!("Expected token '{:?}', found '{:?}' on line {}.", expected, self.current_token, self.line);
        }
    }

    pub fn parse_expression(&mut self) -> ASTNode {
        let mut node = self.parse_term();
        while matches!(self.current_token, Token::Plus | Token::Minus | Token::GreaterThan | Token::LessThan) {
            let token = self.current_token.clone();
            self.consume(token.clone());
            node = ASTNode::BinaryOp(Box::new(node), token, Box::new(self.parse_term()));
        }
        node
    }

    pub fn parse_term(&mut self) -> ASTNode {
        let mut node = self.parse_factor();
        while matches!(self.current_token, Token::Star | Token::Slash | Token::StarStar | Token::Modulo) {
            let token = self.current_token.clone();
            self.consume(token.clone());
            node = ASTNode::BinaryOp(Box::new(node), token, Box::new(self.parse_factor()));
        }
        node
    }

    pub fn parse_factor(&mut self) -> ASTNode {
        match self.current_token.clone() {
            Token::Float(value) => {
                let value_clone = value.clone();
                self.consume(Token::Float(value));
                ASTNode::Float(value_clone)
            }
            Token::Identifier(name) => {
                self.consume(Token::Identifier(name.clone()));
                if self.current_token == Token::LParen {
                    self.consume(Token::LParen);
                    let mut args = Vec::new();
                    while self.current_token != Token::RParen {
                        let arg = self.parse_expression();
                        args.push(arg);
                        if self.current_token == Token::Comma {
                            self.consume(Token::Comma);
                        }
                    }
                    self.consume(Token::RParen);
                    ASTNode::Call(name, args)
                } else {
                    ASTNode::Identifier(name)
                }
            }
            Token::StringLiteral(value) => {
                self.consume(Token::StringLiteral(value.clone()));
                ASTNode::StringLiteral(value)
            }
            Token::DewPoint => self.parse_dew_point(),
            Token::FToC => self.parse_ftoc(),
            Token::CToF => self.parse_ctof(),
            Token::CToK => self.parse_ctok(),
            Token::KToC => self.parse_ktoc(),
            Token::FToK => self.parse_ftok(),
            Token::KToF => self.parse_ktof(),
            Token::PauliX => self.parse_paulix(),
            Token::PauliY => self.parse_pauliy(),
            Token::PauliZ => self.parse_pauliz(),
            Token::Hadamard => self.parse_hadamard(),
            Token::CNot => self.parse_cnot(),
            Token::Qubit => self.parse_qubit(),
            Token::MeasureQubit => self.parse_measure_qubit(),
            Token::ResetQubit => self.parse_reset_qubit(),
            Token::Toffoli => self.parse_toffoli(),
            Token::SWAP => self.parse_swap(),
            Token::Phase => self.parse_phase(),
            Token::TGate => self.parse_tgate(),
            Token::SGate => self.parse_sgate(),
            Token::Fredkin => self.parse_fredkin(),
            Token::Pi => {
                self.consume(Token::Pi);
                ASTNode::Pi
            }
            Token::Kelvin => {
                self.consume(Token::Kelvin);
                ASTNode::Kelvin
            }
            Token::RD => {
                self.consume(Token::RD);
                ASTNode::RD
            }
            Token::CP => {
                self.consume(Token::CP);
                ASTNode::CP
            }
            Token::P0 => {
                self.consume(Token::P0);
                ASTNode::P0
            }
            Token::LV => {
                self.consume(Token::LV);
                ASTNode::LV
            }
            Token::CW => {
                self.consume(Token::CW);
                ASTNode::CW
            }
            Token::RhoAir => {
                self.consume(Token::RhoAir);
                ASTNode::RhoAir
            }
            Token::RhoWater => {
                self.consume(Token::RhoWater);
                ASTNode::RhoWater
            }
            Token::G => {
                self.consume(Token::G);
                ASTNode::G
            }
            Token::LParen => {
                self.consume(Token::LParen);
                let expr = self.parse_expression();
                self.consume(Token::RParen);
                expr
            }
            Token::LBrace => {
                self.consume(Token::LBrace);
                let block = self.parse_block();
                ASTNode::Block(block)
            }
            _ => panic!("Unexpected token '{:?}' on line {}.", self.current_token, self.line),
        }
    }

    pub fn parse_function_definition(&mut self) -> ASTNode {
        self.consume(Token::Function);
        let name = if let Token::Identifier(name) = self.current_token.clone() {
            self.consume(Token::Identifier(name.clone()));
            name
        } else {
            panic!("Expected function name on line {}.", self.line);
        };
        self.consume(Token::LParen);
        let mut params = Vec::new();
        while self.current_token != Token::RParen {
            if let Token::Identifier(param) = self.current_token.clone() {
                self.consume(Token::Identifier(param.clone()));
                params.push(param);
                if self.current_token == Token::Comma {
                    self.consume(Token::Comma);
                }
            } else {
                panic!("Expected parameter name on line {}.", self.line);
            }
        }
        self.consume(Token::RParen);
        self.consume(Token::LBrace);
        let body = self.parse_block();
        ASTNode::Function(name, params, Box::new(ASTNode::Block(body)))
    }

    fn parse_dew_point(&mut self) -> ASTNode {
        self.consume(Token::DewPoint);
        self.consume(Token::LParen);
        let temp = self.parse_expression();
        self.consume(Token::Comma);
        let humidity = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::DewPoint(Box::new(temp), Box::new(humidity))
    }

    fn parse_ftoc(&mut self) -> ASTNode {
        self.consume(Token::FToC);
        self.consume(Token::LParen);
        let fahrenheit = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::FToC(Box::new(fahrenheit))
    }

    fn parse_ctof(&mut self) -> ASTNode {
        self.consume(Token::CToF);
        self.consume(Token::LParen);
        let celsius = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::CToF(Box::new(celsius))
    }

    fn parse_ctok(&mut self) -> ASTNode {
        self.consume(Token::CToK);
        self.consume(Token::LParen);
        let celsius = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::CToK(Box::new(celsius))
    }

    fn parse_ktoc(&mut self) -> ASTNode {
        self.consume(Token::KToC);
        self.consume(Token::LParen);
        let kelvin = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::KToC(Box::new(kelvin))
    }

    fn parse_ftok(&mut self) -> ASTNode {
        self.consume(Token::FToK);
        self.consume(Token::LParen);
        let fahrenheit = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::FToK(Box::new(fahrenheit))
    }

    fn parse_ktof(&mut self) -> ASTNode {
        self.consume(Token::KToF);
        self.consume(Token::LParen);
        let kelvin = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::KToF(Box::new(kelvin))
    }

    fn parse_paulix(&mut self) -> ASTNode {
        self.consume(Token::PauliX);
        self.consume(Token::LParen);
        let qubit = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::PauliX(Box::new(qubit))
    }

    fn parse_pauliy(&mut self) -> ASTNode {
        self.consume(Token::PauliY);
        self.consume(Token::LParen);
        let qubit = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::PauliY(Box::new(qubit))
    }

    fn parse_pauliz(&mut self) -> ASTNode {
        self.consume(Token::PauliZ);
        self.consume(Token::LParen);
        let qubit = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::PauliZ(Box::new(qubit))
    }

    fn parse_hadamard(&mut self) -> ASTNode {
        self.consume(Token::Hadamard);
        self.consume(Token::LParen);
        let qubit = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::Hadamard(Box::new(qubit))
    }

    fn parse_cnot(&mut self) -> ASTNode {
        self.consume(Token::CNot);
        self.consume(Token::LParen);
        let control = self.parse_expression();
        self.consume(Token::Comma);
        let target = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::CNot(Box::new(control), Box::new(target))
    }

    fn parse_qubit(&mut self) -> ASTNode {
        self.consume(Token::Qubit);
        self.consume(Token::LParen);
        let state = self.parse_expression();
        self.consume(Token::Comma);
        let num_qubits = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::Qubit(Box::new(state), Box::new(num_qubits))
    }

    fn parse_measure_qubit(&mut self) -> ASTNode {
        self.consume(Token::MeasureQubit);
        self.consume(Token::LParen);
        let qubit = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::MeasureQubit(Box::new(qubit))
    }

    fn parse_reset_qubit(&mut self) -> ASTNode {
        self.consume(Token::ResetQubit);
        self.consume(Token::LParen);
        let qubit = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::ResetQubit(Box::new(qubit))
    }

    fn parse_toffoli(&mut self) -> ASTNode {
        self.consume(Token::Toffoli);
        self.consume(Token::LParen);
        let control1 = self.parse_expression();
        self.consume(Token::Comma);
        let control2 = self.parse_expression();
        self.consume(Token::Comma);
        let target = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::Toffoli(Box::new(control1), Box::new(control2), Box::new(target))
    }

    fn parse_swap(&mut self) -> ASTNode {
        self.consume(Token::SWAP);
        self.consume(Token::LParen);
        let qubit1 = self.parse_expression();
        self.consume(Token::Comma);
        let qubit2 = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::SWAP(Box::new(qubit1), Box::new(qubit2))
    }

    fn parse_phase(&mut self) -> ASTNode {
        self.consume(Token::Phase);
        self.consume(Token::LParen);
        let qubit = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::Phase(Box::new(qubit))
    }

    fn parse_tgate(&mut self) -> ASTNode {
        self.consume(Token::TGate);
        self.consume(Token::LParen);
        let qubit = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::TGate(Box::new(qubit))
    }

    fn parse_sgate(&mut self) -> ASTNode {
        self.consume(Token::SGate);
        self.consume(Token::LParen);
        let qubit = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::SGate(Box::new(qubit))
    }

    fn parse_fredkin(&mut self) -> ASTNode {
        self.consume(Token::Fredkin);
        self.consume(Token::LParen);
        let control = self.parse_expression();
        self.consume(Token::Comma);
        let target1 = self.parse_expression();
        self.consume(Token::Comma);
        let target2 = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::Fredkin(Box::new(control), Box::new(target1), Box::new(target2))
    }

    fn parse_call(&mut self) -> ASTNode {
        // EXAMPLE: `call(heat_index(temperature, humidity))`
        self.consume(Token::Call);
        self.consume(Token::LParen);
        let name = if let Token::Identifier(name) = self.current_token.clone() {
            self.consume(Token::Identifier(name.clone()));
            name
        } else {
            panic!("Expected function name on line {}.", self.line);
        };
        self.consume(Token::LParen);
        let mut args = Vec::new();
        while self.current_token != Token::RParen {
            let arg = self.parse_expression();
            args.push(arg);
            if self.current_token == Token::Comma {
                self.consume(Token::Comma);
                if self.current_token == Token::RParen {
                    panic!("Trailing comma found before closing parenthesis on line {}.", self.line);
                }
            } else if self.current_token != Token::RParen {
                panic!("Expected token 'RParen' or 'Comma', found '{:?}' on line {}.", self.current_token, self.line);
            }
        }
        self.consume(Token::RParen);
        self.consume(Token::RParen);
        ASTNode::Call(name, args)
    }

    pub fn parse_statement(&mut self) -> ASTNode {
        match self.current_token.clone() {
            Token::Identifier(_) => self.parse_assignment(),
            Token::Print => self.parse_print(),
            Token::If => self.parse_if(),
            Token::Function => self.parse_function_definition(),
            Token::Import => self.parse_import(),
            Token::Call => self.parse_call(),
            Token::LBrace => {
                self.consume(Token::LBrace);
                let block = self.parse_block();
                ASTNode::Block(block)
            }
            _ => panic!("Unexpected token '{:?}' on line {}.", self.current_token, self.line),
        }
    }

    pub fn parse_assignment(&mut self) -> ASTNode {
        let name = match self.current_token.clone() {
            Token::Identifier(name) => name,
            _ => panic!("Expected identifier on line {}.", self.line),
        };
        self.consume(Token::Identifier(name.clone()));
        self.consume(Token::Assign);
        let expr = self.parse_expression();
        ASTNode::Assignment(name, Box::new(expr))
    }

    pub fn parse_print(&mut self) -> ASTNode {
        self.consume(Token::Print);
        self.consume(Token::LParen);
        let expr = self.parse_expression();
        self.consume(Token::RParen);
        ASTNode::Print(Box::new(expr))
    }
    pub fn parse_import(&mut self) -> ASTNode {
        self.consume(Token::Import);
        let module_name = if let Token::StringLiteral(name) = self.current_token.clone() {
            self.consume(Token::StringLiteral(name.clone()));
            name + "." + crate::configs::FILE_EXTENSION
        } else {
            panic!("Expected module name on line {}.", self.line);
        };
        ASTNode::Import(module_name)
    }

    pub fn parse_if(&mut self) -> ASTNode {
        self.consume(Token::If);
        self.consume(Token::LParen);
        let condition = self.parse_expression();
        self.consume(Token::RParen);
        self.consume(Token::LBrace);
        let then_branch = self.parse_block();
        let else_branch = if self.current_token == Token::Else {
            self.consume(Token::Else);
            self.consume(Token::LBrace);
            let else_branch = self.parse_block();
            Some(Box::new(ASTNode::Block(else_branch)))
        } else {
            None
        };
        ASTNode::If(Box::new(condition), Box::new(ASTNode::Block(then_branch)), else_branch)
    }

    pub fn parse_block(&mut self) -> Vec<ASTNode> {
        let mut nodes = Vec::new();
        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            nodes.push(self.parse_statement());
        }
        self.consume(Token::RBrace);
        nodes
    }

    pub fn parse(&mut self) -> Vec<ASTNode> {
        let mut nodes = Vec::new();
        while self.current_token != Token::EOF {
            nodes.push(self.parse_statement());
        }
        nodes
    }
}