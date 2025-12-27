use std::ops::Range;

use crate::traits;

// Tokens
//
#[derive(Debug, Clone)]
pub enum Token {
    //General
    Colon(Range<usize>),
    Comma(Range<usize>),
    LArrow(Range<usize>),
    // chunk
    Const(Range<usize>),
    Static(Range<usize>),
    Fn(Range<usize>),
    EOF(Range<usize>),
    // Statement
    Let(Range<usize>),
    If(Range<usize>),
    Else(Range<usize>),
    LBrace(Range<usize>),
    RBrace(Range<usize>),
    EOL(Range<usize>),
    // Expression
    NumericLiteral(Range<usize>),
    Identifier(SpannedIdentifier),
    Operator(SpannedOperator),
    LParen(Range<usize>),
    RParen(Range<usize>),

    Unknown(Range<usize>),
}
impl traits::TokenType for Token {}
impl Token {
    pub fn get_span(&self) -> Range<usize> {
        return match self {
            Token::Colon(range) => range.clone(),
            Token::Comma(range) => range.clone(),
            Token::LArrow(range) => range.clone(),
            //
            Token::Const(range) => range.clone(),
            Token::Static(range) => range.clone(),
            Token::Fn(range) => range.clone(),
            Token::EOF(range) => range.clone(),
            //
            Token::Let(range) => range.clone(),
            Token::If(range) => range.clone(),
            Token::Else(range) => range.clone(),
            Token::LBrace(range) => range.clone(),
            Token::RBrace(range) => range.clone(),
            Token::EOL(range) => range.clone(),
            //
            Token::NumericLiteral(range) => range.clone(),
            Token::Identifier(id_token) => id_token.span.clone(),
            Token::Operator(op_token) => op_token.span.clone(),
            Token::LParen(range) => range.clone(),
            Token::RParen(range) => range.clone(),
            Token::Unknown(range) => range.clone(),
        };
    }

    pub fn set_span(mut self, s: Range<usize>) -> Token {
        match &mut self {
            Token::Colon(range) => _ = std::mem::replace(range, s),
            Token::Comma(range) => _ = std::mem::replace(range, s),
            Token::LArrow(range) => _ = std::mem::replace(range, s),
            //
            Token::Const(range) => _ = std::mem::replace(range, s),
            Token::Static(range) => _ = std::mem::replace(range, s),
            Token::Fn(range) => _ = std::mem::replace(range, s),
            Token::EOF(range) => _ = std::mem::replace(range, s),
            //
            Token::Let(range) => _ = std::mem::replace(range, s),
            Token::If(range) => _ = std::mem::replace(range, s),
            Token::Else(range) => _ = std::mem::replace(range, s),
            Token::LBrace(range) => _ = std::mem::replace(range, s),
            Token::RBrace(range) => _ = std::mem::replace(range, s),
            Token::EOL(range) => _ = std::mem::replace(range, s),
            //
            Token::NumericLiteral(range) => _ = std::mem::replace(range, s),
            Token::Identifier(id_token) => _ = std::mem::replace(&mut id_token.span, s),
            Token::Operator(op_token) => _ = std::mem::replace(&mut op_token.span, s),
            Token::LParen(range) => _ = std::mem::replace(range, s),
            Token::RParen(range) => _ = std::mem::replace(range, s),
            Token::Unknown(range) => _ = std::mem::replace(range, s),
        }
        return self;
    }
}

// Spanned Nodes
//
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpannedIdentifier {
    pub span: Range<usize>,
}
#[derive(Debug, Clone)]
pub struct SpannedOperator {
    pub span: Range<usize>,
    pub presedence: usize,
}
#[derive(Debug)]
pub struct SpannedParam {
    pub name: SpannedIdentifier,
    pub ptype: SpannedIdentifier,
}

#[derive(Debug)]
pub enum SpannedChunk {
    Constant,
    StaticVar,
    Function {
        name: SpannedIdentifier,
        params: Vec<SpannedParam>,
        rtype: SpannedIdentifier,
        body: SpannedStatement,
        span: Range<usize>,
    },
    EOF,
}
#[derive(Debug)]
pub enum SpannedStatement {
    Decleration {
        name: SpannedIdentifier,
        expression: Box<SpannedExpression>,
        rtype: SpannedIdentifier,
        span: Range<usize>,
    },
    Reassignment {
        name: SpannedIdentifier,
        expression: Box<SpannedExpression>,
        span: Range<usize>,
    },
    If {
        condition: Box<SpannedExpression>,
        statement: Box<SpannedStatement>,
        ielse: Option<Box<SpannedStatement>>,
        span: Range<usize>,
    },
    Scope {
        body: Vec<SpannedStatement>,
        span: Range<usize>,
    },
    VoidCall {
        name: Box<SpannedIdentifier>,
        params: Vec<SpannedExpression>,
        span: Range<usize>,
    },
}
#[derive(Debug, PartialEq, Eq)]
pub enum SpannedExpression {
    Identifier(SpannedIdentifier),
    Literal {
        span: Range<usize>,
    },
    UnaryOperator {
        operation: Range<usize>,
        expression: Box<SpannedExpression>,
    },
    BinaryOperator {
        left: Box<SpannedExpression>,
        span: Range<usize>,
        presedence: usize,
        right: Box<SpannedExpression>,
    },
    Call {
        name: Box<SpannedIdentifier>,
        params: Vec<SpannedExpression>,
        span: Range<usize>,
    },
    Tmp,
}
