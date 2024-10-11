use crate::token::Token;

#[derive(Debug, Clone)]
pub enum ASTNode {
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