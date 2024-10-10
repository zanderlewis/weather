use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};

// Token types for the lexer
#[derive(Debug, PartialEq, Clone)]
enum Token {
    KeywordModel,
    KeywordLocation,
    KeywordParameter,
    KeywordForecast,
    KeywordCompute,
    KeywordIf,
    KeywordElseIf,
    KeywordEndIf,
    KeywordPrint,
    KeywordOutput,
    KeywordFunction,
    KeywordEndFunction,
    KeywordCall,
    Identifier(String),
    Number(f64),
    StringLiteral(String),
    GreaterThan,
    LessThan,
    Equals,
    Plus,
    Minus,
    Multiply,
    Divide,
    Comma,
    OpenParen,
    CloseParen,
    Eof,
}

// Lexer to convert the script into tokens
struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    fn new(input: String) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn next_token(&mut self) -> Token {
        while self.position < self.input.len() {
            let current_char = self.input[self.position];

            match current_char {
                ' ' | '\t' | '\n' => self.position += 1, // Skip whitespace
                '=' => {
                    self.position += 1;
                    return Token::Equals;
                }
                '>' => {
                    self.position += 1;
                    return Token::GreaterThan;
                }
                '<' => {
                    self.position += 1;
                    return Token::LessThan;
                }
                '+' => {
                    self.position += 1;
                    return Token::Plus;
                }
                '-' => {
                    self.position += 1;
                    return Token::Minus;
                }
                '*' => {
                    self.position += 1;
                    return Token::Multiply;
                }
                '/' => {
                    self.position += 1;
                    return Token::Divide;
                }
                ',' => {
                    self.position += 1;
                    return Token::Comma;
                }
                '(' => {
                    self.position += 1;
                    return Token::OpenParen;
                }
                ')' => {
                    self.position += 1;
                    return Token::CloseParen;
                }
                '"' => return self.parse_string_literal(),
                _ if current_char.is_digit(10) || current_char == '.' => return self.parse_number(),
                _ if current_char.is_alphabetic() => return self.parse_identifier(),
                _ => self.position += 1,
            }
        }
        Token::Eof
    }

    fn parse_identifier(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position].is_alphanumeric() {
            self.position += 1;
        }
        let identifier: String = self.input[start..self.position].iter().collect();
        match identifier.as_str() {
            "MODEL" => Token::KeywordModel,
            "LOCATION" => Token::KeywordLocation,
            "PARAMETER" => Token::KeywordParameter,
            "FORECAST" => Token::KeywordForecast,
            "COMPUTE" => Token::KeywordCompute,
            "IF" => Token::KeywordIf,
            "ELSEIF" => Token::KeywordElseIf,
            "ENDIF" => Token::KeywordEndIf,
            "PRINT" => Token::KeywordPrint,
            "OUTPUT" => Token::KeywordOutput,
            "FUNCTION" => Token::KeywordFunction,
            "ENDFUNCTION" => Token::KeywordEndFunction,
            "CALL" => Token::KeywordCall,
            _ => Token::Identifier(identifier),
        }
    }

    fn parse_number(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() && (self.input[self.position].is_digit(10) || self.input[self.position] == '.') {
            self.position += 1;
        }
        let number: String = self.input[start..self.position].iter().collect();
        Token::Number(number.parse().unwrap())
    }

    fn parse_string_literal(&mut self) -> Token {
        self.position += 1; // Skip the opening quote
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position] != '"' {
            self.position += 1;
        }
        let string_literal: String = self.input[start..self.position].iter().collect();
        self.position += 1; // Skip the closing quote
        Token::StringLiteral(string_literal)
    }
}

// AST Node for the Parser
#[derive(Debug, Clone)]
enum ASTNode {
    Model { name: String, location: String, parameters: HashMap<String, f64>, forecast: Vec<ASTNode> },
    Parameter { name: String, value: f64 },
    ForecastLoop { duration: i32, step: i32, computations: Vec<ASTNode> },
    Computation { variable: String, expression: String },
    Conditional { condition: String, true_block: Vec<ASTNode>, false_block: Option<Vec<ASTNode>> },
    PrintStatement { message: String },
    Output { variables: Vec<String>, file: String },
    Function { name: String, body: Vec<ASTNode> },
    Call { name: String },
}

// Parser to convert tokens into an AST
struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn parse(&mut self) -> ASTNode {
        self.parse_model()
    }

    fn parse_model(&mut self) -> ASTNode {
        self.expect_token(Token::KeywordModel);
        let name = self.expect_identifier();
        self.expect_token(Token::KeywordLocation);
        let location = self.expect_string_literal();
        let mut parameters = HashMap::new();
        let mut forecast = Vec::new();

        while !self.is_eof() {
            match self.peek_token() {
                Token::KeywordParameter => {
                    self.next_token();
                    let param_name = self.expect_identifier();
                    let param_value = self.expect_number();
                    parameters.insert(param_name, param_value);
                }
                Token::KeywordForecast => {
                    self.next_token();
                    let duration = self.expect_number() as i32;
                    let step = self.expect_number() as i32;
                    let computations = self.parse_computations();
                    forecast.push(ASTNode::ForecastLoop { duration, step, computations });
                }
                Token::KeywordFunction => {
                    self.next_token();
                    let func_name = self.expect_identifier();
                    let body = self.parse_computations();
                    self.expect_token(Token::KeywordEndFunction);
                    forecast.push(ASTNode::Function { name: func_name, body });
                }
                Token::KeywordCall => {
                    self.next_token();
                    let func_name = self.expect_identifier();
                    forecast.push(ASTNode::Call { name: func_name });
                }
                Token::KeywordOutput => {
                    self.next_token();
                    let variables = self.parse_variables();
                    let file = self.expect_string_literal();
                    forecast.push(ASTNode::Output { variables, file });
                }
                _ => break,
            }
        }

        ASTNode::Model { name, location, parameters, forecast }
    }

    fn parse_computations(&mut self) -> Vec<ASTNode> {
        let mut computations = Vec::new();
        while !self.is_eof() {
            match self.peek_token() {
                Token::KeywordCompute => {
                    self.next_token();
                    let variable = self.expect_identifier();
                    let expression = self.expect_expression();
                    computations.push(ASTNode::Computation { variable, expression });
                }
                Token::KeywordIf => {
                    self.next_token();
                    let condition = self.expect_expression();
                    let true_block = self.parse_computations();
                    let false_block = if self.peek_token() == Token::KeywordElseIf {
                        self.next_token();
                        Some(self.parse_computations())
                    } else {
                        None
                    };
                    self.expect_token(Token::KeywordEndIf);
                    computations.push(ASTNode::Conditional { condition, true_block, false_block });
                }
                Token::KeywordPrint => {
                    self.next_token();
                    let message = self.expect_string_literal();
                    computations.push(ASTNode::PrintStatement { message });
                }
                _ => break,
            }
        }
        computations
    }

    fn parse_variables(&mut self) -> Vec<String> {
        let mut variables = Vec::new();
        while let Token::Identifier(var) = self.peek_token() {
            variables.push(var.clone());
            self.next_token();
            if self.peek_token() != Token::Comma {
                break;
            }
            self.next_token();
        }
        variables
    }

    fn expect_token(&mut self, expected: Token) {
        let token = self.next_token();
        if token != expected {
            panic!("Expected {:?}, found {:?}", expected, token);
        }
    }

    fn expect_identifier(&mut self) -> String {
        if let Token::Identifier(name) = self.next_token() {
            name
        } else {
            panic!("Expected identifier");
        }
    }

    fn expect_number(&mut self) -> f64 {
        if let Token::Number(value) = self.next_token() {
            value
        } else {
            panic!("Expected number");
        }
    }

    fn expect_string_literal(&mut self) -> String {
        if let Token::StringLiteral(value) = self.next_token() {
            value
        } else {
            panic!("Expected string literal");
        }
    }

    fn expect_expression(&mut self) -> String {
        let mut expression = String::new();
        while !self.is_eof() {
            match self.peek_token() {
                Token::Identifier(_) | Token::Number(_) | Token::Plus | Token::Minus | Token::Multiply | Token::Divide => {
                    expression.push_str(&format!("{:?} ", self.next_token()));
                }
                _ => break,
            }
        }
        expression.trim().to_string()
    }

    fn next_token(&mut self) -> Token {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            token
        } else {
            Token::Eof
        }
    }

    fn peek_token(&self) -> Token {
        if self.position < self.tokens.len() {
            self.tokens[self.position].clone()
        } else {
            Token::Eof
        }
    }

    fn is_eof(&self) -> bool {
        self.position >= self.tokens.len()
    }
}

// Interpreter for executing the AST
struct Interpreter {
    variables: HashMap<String, f64>,
    functions: HashMap<String, Vec<ASTNode>>,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    fn run(&mut self, ast: &ASTNode) {
        match ast {
            ASTNode::Model { name, location, parameters, forecast } => {
                println!("Running model: {} for location: {}", name, location);
                self.variables = parameters.clone();
                for step in forecast {
                    self.run(step);
                }
            }
            ASTNode::Parameter { name, value } => {
                self.variables.insert(name.clone(), *value);
                println!("Set parameter {} to {}", name, value);
            }
            ASTNode::ForecastLoop { duration, step, computations } => {
                for hour in (0..*duration).step_by(*step as usize) {
                    println!("Forecasting hour: {}", hour);
                    for computation in computations {
                        self.run(computation);
                    }
                }
            }
            ASTNode::Computation { variable, expression } => {
                if let Some(value) = self.evaluate_expression(expression) {
                    self.variables.insert(variable.clone(), value);
                    println!("Computed new value for {}: {}", variable, value);
                }
            }
            ASTNode::Conditional { condition, true_block, false_block } => {
                if self.evaluate_condition(condition) {
                    for statement in true_block {
                        self.run(statement);
                    }
                } else if let Some(false_statements) = false_block {
                    for statement in false_statements {
                        self.run(statement);
                    }
                }
            }
            ASTNode::PrintStatement { message } => {
                println!("{}", message);
            }
            ASTNode::Output { variables, file } => {
                self.write_to_csv(variables, file);
            }
            ASTNode::Function { name, body } => {
                self.functions.insert(name.clone(), body.to_vec());
            }
            ASTNode::Call { name } => {
                if let Some(body) = self.functions.get(name).cloned() {
                    for statement in body {
                        self.run(&statement);
                    }
                }
            }
        }
    }

    fn evaluate_expression(&self, expression: &String) -> Option<f64> {
        // Basic expression evaluation (simplified)
        let tokens: Vec<&str> = expression.split_whitespace().collect();
        if tokens.len() == 3 {
            let left = self.variables.get(tokens[0])?;
            let right: f64 = match tokens[2].parse() {
                Ok(val) => val,
                Err(_) => return None,
            };
            match tokens[1] {
                "+" => Some(left + right),
                "-" => Some(left - right),
                "*" => Some(left * right),
                "/" => Some(left / right),
                _ => None,
            }
        } else {
            None
        }
    }

    fn evaluate_condition(&self, condition: &String) -> bool {
        // Basic condition evaluation (simplified)
        let tokens: Vec<&str> = condition.split_whitespace().collect();
        if tokens.len() == 3 {
            let left = self.variables.get(tokens[0]);
            let right: f64 = match tokens[2].parse() {
                Ok(val) => val,
                Err(_) => return false,
            };
            match tokens[1] {
                ">" => match left {
                    Some(&val) => val > right,
                    None => false,
                },
                "<" => match left {
                    Some(&val) => val < right,
                    None => false,
                },
                "=" => match left {
                    Some(&val) => val == right,
                    None => false,
                },
                _ => false,
            }
        } else {
            false
        }
    }

    fn write_to_csv(&self, variables: &Vec<String>, file: &String) {
        let mut wtr = csv::Writer::from_path(file).expect("Unable to create file");
        for variable in variables {
            if let Some(value) = self.variables.get(variable) {
                wtr.write_record(&[variable, &value.to_string()]).expect("Unable to write record");
            }
        }
        wtr.flush().expect("Unable to flush writer");
        println!("Saved variables {:?} to file {}", variables, file);
    }
}

// Example MeteoScript Program
fn main() {
    let filename = "script.wthr";
    let mut file = fs::File::open(filename).expect("Unable to open file");
    let mut script = String::new();
    file.read_to_string(&mut script).expect("Unable to read file");

    let mut lexer = Lexer::new(script);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token();
        if token == Token::Eof {
            break;
        }
        tokens.push(token);
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.run(&ast);
}