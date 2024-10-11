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

    fn consume(&mut self, expected: Token) {
        if self.current_token == expected {
            self.current_token = self.lexer.next_token();
        } else {
            panic!("Expected token: {:?}, found: {:?}", expected, self.current_token);
        }
    }

    pub fn parse_expression(&mut self) -> ASTNode {
        let mut node = self.parse_term();
        while matches!(self.current_token, Token::Plus | Token::Minus | Token::GreaterThan | Token::LessThan) {
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
        while matches!(self.current_token, Token::Star | Token::Slash) {
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
            Token::DewPoint => self.parse_dew_point(),
            Token::FToC => self.parse_ftoc(),
            Token::CToF => self.parse_ctof(),
            Token::CToK => self.parse_ctok(),
            Token::KToC => self.parse_ktoc(),
            Token::FToK => self.parse_ftok(),
            Token::KToF => self.parse_ktof(),
            Token::LParen => {
                self.consume(Token::LParen);
                let expr = self.parse_expression();
                self.consume(Token::RParen);
                expr
            }
            Token::LBrace => {
                self.consume(Token::LBrace);
                let expr = self.parse_expression();
                self.consume(Token::RBrace);
                expr
            }
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
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

    pub fn parse_if(&mut self) -> ASTNode {
        self.consume(Token::If);
        self.consume(Token::LParen);
        let condition = self.parse_expression();
        self.consume(Token::RParen);
        self.consume(Token::LBrace);
        let then_branch = self.parse_block();
        self.consume(Token::RBrace);
        let else_branch = if self.current_token == Token::Else {
            self.consume(Token::Else);
            self.consume(Token::LBrace);
            let else_branch = self.parse_block();
            self.consume(Token::RBrace);
            Some(Box::new(else_branch))
        } else {
            None
        };
        ASTNode::If(Box::new(condition), Box::new(ASTNode::Block(then_branch)), else_branch.map(|b| Box::new(ASTNode::Block(*b))))
    }

    pub fn parse_block(&mut self) -> Vec<ASTNode> {
        let mut nodes = Vec::new();
        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            nodes.push(self.parse_statement());
        }
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