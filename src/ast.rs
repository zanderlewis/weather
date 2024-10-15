use crate::token::Token;
use num_rational::BigRational;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ASTNode {
    Block(Vec<ASTNode>),
    Float(BigRational),
    Identifier(String),
    StringLiteral(String),
    BinaryOp(Box<ASTNode>, Token, Box<ASTNode>),
    Assignment(String, Box<ASTNode>),
    Call(String, Vec<ASTNode>),
    Function(String, Vec<String>, Box<ASTNode>),
    Import(String),
    Print(Box<ASTNode>),
    If(Box<ASTNode>, Box<ASTNode>, Option<Box<ASTNode>>), // condition, then, else
    DewPoint(Box<ASTNode>, Box<ASTNode>), // temperature, humidity
    FToC(Box<ASTNode>), // fahrenheit -> celsius
    CToF(Box<ASTNode>), // celsius -> fahrenheit
    CToK(Box<ASTNode>), // celsius -> kelvin
    KToC(Box<ASTNode>), // kelvin -> celsius
    FToK(Box<ASTNode>), // fahrenheit -> kelvin
    KToF(Box<ASTNode>), // kelvin -> fahrenheit
    Pi,
    Kelvin,
    RD,
    CP,
    P0,
    LV,
    CW,
    RhoAir,
    RhoWater,
    G,
    GreaterThan(Box<ASTNode>, Box<ASTNode>),
    LessThan(Box<ASTNode>, Box<ASTNode>),
}
