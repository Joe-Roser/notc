use super::Span;
use std::ops::Range;
// Resolved Nodes
//
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentifierId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedIdentifier {
    pub span: Span,
    pub id: IdentifierId,
}
#[derive(Debug, Clone)]
pub struct ResolvedOperator {
    pub span: Span,
    pub presedence: usize,
}
#[derive(Debug)]
pub struct ResolvedParam {
    pub name: ResolvedIdentifier,
    pub ptype: ResolvedIdentifier,
}

#[derive(Debug)]
pub enum ResolvedChunk {
    Constant,
    StaticVar,
    Function {
        name: ResolvedIdentifier,
        params: Vec<ResolvedParam>,
        rtype: ResolvedIdentifier,
        body: ResolvedStatement,
        span: Span,
    },
    EOF,
}
#[derive(Debug)]
pub enum ResolvedStatement {
    Decleration {
        name: ResolvedIdentifier,
        expression: Box<ResolvedExpression>,
        rtype: ResolvedIdentifier,
        span: Span,
    },
    Reassignment {
        name: ResolvedIdentifier,
        expression: Box<ResolvedExpression>,
        span: Span,
    },
    If {
        condition: Box<ResolvedExpression>,
        statement: Box<ResolvedStatement>,
        ielse: Option<Box<ResolvedStatement>>,
        span: Span,
    },
    Scope {
        body: Vec<ResolvedStatement>,
        span: Span,
    },
    VoidCall {
        name: ResolvedIdentifier,
        params: Vec<ResolvedExpression>,
        span: Span,
    },
    Return {
        expression: Option<Box<ResolvedExpression>>,
        span: Span,
    },
}
#[derive(Debug, PartialEq, Eq)]
pub enum ResolvedExpression {
    Identifier(ResolvedIdentifier),
    Literal {
        span: Span,
    },
    UnaryOperator {
        operation: Span,
        expression: Box<ResolvedExpression>,
    },
    BinaryOperator {
        left: Box<ResolvedExpression>,
        span: Span,
        presedence: usize,
        right: Box<ResolvedExpression>,
    },
    Call,
    Tmp,
}
