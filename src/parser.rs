use crate::lexer::Lexer;
use crate::token::Token;
use crate::ast::ASTNode;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        Self { lexer, current_token }
    }
    
    pub fn parse_expression(&mut self) -> ASTNode {
        let mut node = self.parse_term();
        while self.current_token == Token::Plus || self.current_token == Token::Minus || self.current_token == Token::GreaterThan || self.current_token == Token::LessThan {
            let token = self.current_token.clone();
            self.current_token = self.lexer.next_token();
            node = match token {
                Token::Plus | Token::Minus => ASTNode::BinaryOp(Box::new(node), token, Box::new(self.parse_term())),
                Token::GreaterThan => ASTNode::GreaterThan(Box::new(node), Box::new(self.parse_term())),
                Token::LessThan => ASTNode::LessThan(Box::new(node), Box::new(self.parse_term())),
                _ => panic!("Unexpected token: {:?}", token),
            };
        }
        node
    }

    pub fn parse_term(&mut self) -> ASTNode {
        let mut node = self.parse_factor();
        while self.current_token == Token::Star || self.current_token == Token::Slash {
            let token = self.current_token.clone();
            self.current_token = self.lexer.next_token();
            node = ASTNode::BinaryOp(Box::new(node), token, Box::new(self.parse_factor()));
        }
        node
    }

    pub fn parse_factor(&mut self) -> ASTNode {
        match self.current_token.clone() {
            Token::Number(value) => {
                self.current_token = self.lexer.next_token();
                ASTNode::Number(value)
            }
            Token::Identifier(name) => {
                self.current_token = self.lexer.next_token();
                ASTNode::Identifier(name)
            }
            Token::StringLiteral(value) => {
                self.current_token = self.lexer.next_token();
                ASTNode::StringLiteral(value)
            }
            Token::DewPoint => {
                self.current_token = self.lexer.next_token(); // Consume 'dewpoint'
                if self.current_token == Token::LParen {
                    self.current_token = self.lexer.next_token(); // Consume '('
                    let temp = self.parse_expression();
                    if self.current_token == Token::Comma {
                        self.current_token = self.lexer.next_token(); // Consume ','
                        let humidity = self.parse_expression();
                        if self.current_token == Token::RParen {
                            self.current_token = self.lexer.next_token(); // Consume ')'
                            ASTNode::DewPoint(Box::new(temp), Box::new(humidity))
                        } else {
                            panic!("Expected ')'");
                        }
                    } else {
                        panic!("Expected ','");
                    }
                } else {
                    panic!("Expected '(' after 'dewpoint'");
                }
            }
            Token::FToC => {
                self.current_token = self.lexer.next_token(); // Consume 'ftoc'
                if self.current_token == Token::LParen {
                    self.current_token = self.lexer.next_token(); // Consume '('
                    let fahrenheit = self.parse_expression();
                    if self.current_token == Token::RParen {
                        self.current_token = self.lexer.next_token(); // Consume ')'
                        ASTNode::FToC(Box::new(fahrenheit))
                    } else {
                        panic!("Expected ')'");
                    }
                } else {
                    panic!("Expected '(' after 'ftoc'");
                }
            }
            Token::CToF => {
                self.current_token = self.lexer.next_token(); // Consume 'ctof'
                if self.current_token == Token::LParen {
                    self.current_token = self.lexer.next_token(); // Consume '('
                    let celsius = self.parse_expression();
                    if self.current_token == Token::RParen {
                        self.current_token = self.lexer.next_token(); // Consume ')'
                        ASTNode::CToF(Box::new(celsius))
                    } else {
                        panic!("Expected ')'");
                    }
                } else {
                    panic!("Expected '(' after 'ctof'");
                }
            }
            Token::LParen => {
                self.current_token = self.lexer.next_token();
                let expr = self.parse_expression();
                if self.current_token == Token::RParen {
                    self.current_token = self.lexer.next_token();
                    expr
                } else {
                    panic!("Expected ')'");
                }
            }
            Token::LBrace => {
                self.current_token = self.lexer.next_token();
                let expr = self.parse_expression();
                if self.current_token == Token::RBrace {
                    self.current_token = self.lexer.next_token();
                    expr
                } else {
                    panic!("Expected '}}'");
                }
            }
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    pub fn parse_statement(&mut self) -> ASTNode {
        match self.current_token.clone() {
            Token::Identifier(_) => self.parse_assignment(),
            Token::Print => self.parse_print(),
            Token::If => self.parse_if(),
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    pub fn parse_assignment(&mut self) -> ASTNode {
        let name = match self.current_token.clone() {
            Token::Identifier(name) => name,
            _ => panic!("Expected identifier"),
        };
        self.current_token = self.lexer.next_token(); // Consume identifier
        if self.current_token == Token::Assign {
            self.current_token = self.lexer.next_token(); // Consume '='
            let expr = self.parse_expression();
            ASTNode::Assignment(name, Box::new(expr))
        } else {
            panic!("Expected '='");
        }
    }

    pub fn parse_print(&mut self) -> ASTNode {
        self.current_token = self.lexer.next_token(); // Consume 'print'
        if self.current_token == Token::LParen {
            self.current_token = self.lexer.next_token(); // Consume '('
            let expr = self.parse_expression();
            if self.current_token == Token::RParen {
                self.current_token = self.lexer.next_token(); // Consume ')'
                ASTNode::Print(Box::new(expr))
            } else {
                panic!("Expected ')'");
            }
        } else {
            panic!("Expected '(' after 'print'");
        }
    }

    pub fn parse_if(&mut self) -> ASTNode {
        self.current_token = self.lexer.next_token(); // Consume 'if'
        if self.current_token == Token::LParen {
            self.current_token = self.lexer.next_token(); // Consume '('
            let condition = self.parse_expression();
            if self.current_token == Token::RParen {
                self.current_token = self.lexer.next_token(); // Consume ')'
                if self.current_token == Token::LBrace {
                    self.current_token = self.lexer.next_token(); // Consume '{'
                    let then_branch = self.parse_block();
                    if self.current_token == Token::RBrace {
                        self.current_token = self.lexer.next_token(); // Consume '}'
                        let else_branch = if self.current_token == Token::Else {
                            self.current_token = self.lexer.next_token(); // Consume 'else'
                            if self.current_token == Token::LBrace {
                                self.current_token = self.lexer.next_token(); // Consume '{'
                                let else_branch = self.parse_block();
                                if self.current_token == Token::RBrace {
                                    self.current_token = self.lexer.next_token(); // Consume '}'
                                    Some(Box::new(else_branch))
                                } else {
                                    panic!("Expected '}}'");
                                }
                            } else {
                                panic!("Expected '{{' after 'else'");
                            }
                        } else {
                            None
                        };
                        ASTNode::If(Box::new(condition), Box::new(then_branch), else_branch)
                    } else {
                        panic!("Expected '}}'");
                    }
                } else {
                    panic!("Expected '{{' after 'if'");
                }
            } else {
                panic!("Expected ')'");
            }
        } else {
            panic!("Expected '(' after 'if'");
        }
    }

    pub fn parse_block(&mut self) -> ASTNode {
        let mut nodes = Vec::new();
        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            nodes.push(self.parse_statement());
        }
        // Assuming a block is a sequence of statements, we can return the first statement for simplicity
        // In a real implementation, you might want to return a Block node containing all statements
        nodes.into_iter().next().expect("Expected at least one statement in block")
    }

    pub fn parse(&mut self) -> Vec<ASTNode> {
        let mut nodes = Vec::new();
        while self.current_token != Token::EOF {
            nodes.push(self.parse_statement());
        }
        nodes
    }
}