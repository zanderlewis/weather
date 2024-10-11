use crate::token::Token;
use num_bigint::BigInt;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ASTNode {
    Block(Vec<ASTNode>),
    Number(BigInt),
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