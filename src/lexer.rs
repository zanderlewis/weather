use crate::token::Token;
use num_bigint::BigInt;
use num_rational::BigRational;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
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
            '0'..='9' | '.' => self.read_number(ch),
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

    pub fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    fn read_number(&mut self, first_char: char) -> Token {
        let mut number = first_char.to_string();
        let mut is_float = false;

        while self.position < self.input.len() && (self.input[self.position].is_digit(10) || self.input[self.position] == '.') {
            if self.input[self.position] == '.' {
                is_float = true;
            }
            number.push(self.input[self.position]);
            self.position += 1;
        }

        if is_float {
            Token::Float(BigRational::from_float(number.parse::<f64>().unwrap()).unwrap())
        } else {
            Token::Float(BigRational::from_integer(number.parse::<BigInt>().unwrap()))
        }
    }

    pub fn read_identifier(&mut self, first_char: char) -> Token {
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
            "ctok" => Token::CToK,
            "ktoc" => Token::KToC,
            "ftok" => Token::FToK,
            "ktof" => Token::KToF,
            "_pi_" => Token::Pi,
            "_kelvin_" => Token::Kelvin,
            "_rd_" => Token::RD,
            "_cp_" => Token::CP,
            "_p0_" => Token::P0,
            "_lv_" => Token::LV,
            "_cw_" => Token::CW,
            "_rho_air_" => Token::RhoAir,
            "_rho_water_" => Token::RhoWater,
            "_g_" => Token::G,
            _ => Token::Identifier(identifier),
        }
    }

    pub fn read_string_literal(&mut self) -> Token {
        let mut string = String::new();
        while self.position < self.input.len() && self.input[self.position] != '"' {
            string.push(self.input[self.position]);
            self.position += 1;
        }
        self.position += 1; // Consume closing quote
        Token::StringLiteral(string)
    }
}