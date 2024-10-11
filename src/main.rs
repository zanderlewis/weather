use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    Star,
    Slash,
    GreaterThan,
    LessThan,
    Assign,
    Comma,
    Print,
    LBrace,
    RBrace,
    LParen,
    RParen,
    If,
    Else,
    StringLiteral(String),
    DewPoint,
    FToC,
    CToF,
    EOF,
}

struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let ch = self.input[self.position];
        self.position += 1;

        match ch {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '>' => Token::GreaterThan,
            '<' => Token::LessThan,
            '=' => Token::Assign,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '"' => self.read_string_literal(),
            '0'..='9' => self.read_number(ch),
            'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(ch),
            ',' => Token::Comma,
            '#' => {
                while self.position < self.input.len() && self.input[self.position] != '\n' {
                    self.position += 1;
                }
                self.next_token()
            }
            _ => panic!("Unexpected character: {}", ch),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    fn read_number(&mut self, first_char: char) -> Token {
        let mut number = first_char.to_string();
        while self.position < self.input.len() && self.input[self.position].is_digit(10) {
            number.push(self.input[self.position]);
            self.position += 1;
        }
        Token::Number(number.parse().unwrap())
    }

    fn read_identifier(&mut self, first_char: char) -> Token {
        let mut identifier = first_char.to_string();
        while self.position < self.input.len() && (self.input[self.position].is_alphanumeric() || self.input[self.position] == '_') {
            identifier.push(self.input[self.position]);
            self.position += 1;
        }
        match identifier.as_str() {
            "print" => Token::Print,
            "if" => Token::If,
            "else" => Token::Else,
            "dewpoint" => Token::DewPoint,
            "ftoc" => Token::FToC,
            "ctof" => Token::CToF,
            _ => Token::Identifier(identifier),
        }
    }

    fn read_string_literal(&mut self) -> Token {
        let mut string = String::new();
        while self.position < self.input.len() && self.input[self.position] != '"' {
            string.push(self.input[self.position]);
            self.position += 1;
        }
        self.position += 1; // Consume closing quote
        Token::StringLiteral(string)
    }
}

#[derive(Debug, Clone)]
enum ASTNode {
    Number(f64),
    Identifier(String),
    StringLiteral(String),
    BinaryOp(Box<ASTNode>, Token, Box<ASTNode>),
    Assignment(String, Box<ASTNode>),
    Print(Box<ASTNode>),
    If(Box<ASTNode>, Box<ASTNode>, Option<Box<ASTNode>>), // condition, then, else
    DewPoint(Box<ASTNode>, Box<ASTNode>), // temperature, humidity
    FToC(Box<ASTNode>), // fahrenheit
    CToF(Box<ASTNode>), // celsius
    GreaterThan(Box<ASTNode>, Box<ASTNode>),
    LessThan(Box<ASTNode>, Box<ASTNode>),
}

struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        Self { lexer, current_token }
    }
    
    fn parse_expression(&mut self) -> ASTNode {
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

    fn parse_term(&mut self) -> ASTNode {
        let mut node = self.parse_factor();
        while self.current_token == Token::Star || self.current_token == Token::Slash {
            let token = self.current_token.clone();
            self.current_token = self.lexer.next_token();
            node = ASTNode::BinaryOp(Box::new(node), token, Box::new(self.parse_factor()));
        }
        node
    }

    fn parse_factor(&mut self) -> ASTNode {
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

    fn parse_statement(&mut self) -> ASTNode {
        match self.current_token.clone() {
            Token::Identifier(_) => self.parse_assignment(),
            Token::Print => self.parse_print(),
            Token::If => self.parse_if(),
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn parse_assignment(&mut self) -> ASTNode {
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

    fn parse_print(&mut self) -> ASTNode {
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

    fn parse_if(&mut self) -> ASTNode {
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

    fn parse_block(&mut self) -> ASTNode {
        let mut nodes = Vec::new();
        while self.current_token != Token::RBrace && self.current_token != Token::EOF {
            nodes.push(self.parse_statement());
        }
        // Assuming a block is a sequence of statements, we can return the first statement for simplicity
        // In a real implementation, you might want to return a Block node containing all statements
        nodes.into_iter().next().expect("Expected at least one statement in block")
    }

    fn parse(&mut self) -> Vec<ASTNode> {
        let mut nodes = Vec::new();
        while self.current_token != Token::EOF {
            nodes.push(self.parse_statement());
        }
        nodes
    }
}

struct Interpreter {
    variables: HashMap<String, f64>,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    fn execute(&mut self, node: ASTNode) {
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

    fn evaluate(&mut self, node: ASTNode) -> f64 {
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

    fn interpret(&mut self, nodes: Vec<ASTNode>) {
        for node in nodes {
            self.execute(node);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <script>", args[0]);
        return;
    }

    let script = fs::read_to_string(&args[1]).expect("Failed to read script");
    let lexer = Lexer::new(script);
    let mut parser = Parser::new(lexer);
    let nodes = parser.parse();
    let mut interpreter = Interpreter::new();
    interpreter.interpret(nodes);
}