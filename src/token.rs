use num_rational::BigRational;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Float(BigRational),
    Identifier(String),
    Function,
    Import,
    Call,
    Plus,
    Minus,
    Star,
    StarStar,
    Slash,
    Modulo,
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
    CToK,
    KToC,
    FToK,
    KToF,
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
    PauliX,
    PauliY,
    PauliZ,
    Hadamard,
    CNot,
    Toffoli,
    SWAP,
    ResetQubit,
    Phase,
    TGate,
    SGate,
    Fredkin,
    Qubit,
    MeasureQubit,
    EOF,
}