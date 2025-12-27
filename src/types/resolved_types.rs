use std::ops::Range;
// Resolved Nodes
//
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentifierId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedIdentifier {
    pub span: Range<usize>,
    pub id: IdentifierId,
}
#[derive(Debug, Clone)]
pub struct ResolvedOperator {
    pub span: Range<usize>,
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
        span: Range<usize>,
    },
    EOF,
}
#[derive(Debug)]
pub enum ResolvedStatement {
    Decleration {
        name: ResolvedIdentifier,
        expression: Box<ResolvedExpression>,
        rtype: ResolvedIdentifier,
        span: Range<usize>,
    },
    Reassignment {
        name: ResolvedIdentifier,
        expression: Box<ResolvedExpression>,
        span: Range<usize>,
    },
    If {
        condition: Box<ResolvedExpression>,
        statement: Box<ResolvedStatement>,
        ielse: Option<Box<ResolvedStatement>>,
        span: Range<usize>,
    },
    Scope {
        body: Vec<ResolvedStatement>,
        span: Range<usize>,
    },
    VoidCall {
        name: ResolvedIdentifier,
        params: Vec<ResolvedExpression>,
        span: Range<usize>,
    },
}
#[derive(Debug, PartialEq, Eq)]
pub enum ResolvedExpression {
    Identifier(ResolvedIdentifier),
    Literal {
        span: Range<usize>,
    },
    UnaryOperator {
        operation: Range<usize>,
        expression: Box<ResolvedExpression>,
    },
    BinaryOperator {
        left: Box<ResolvedExpression>,
        span: Range<usize>,
        presedence: usize,
        right: Box<ResolvedExpression>,
    },
    Call,
    Tmp,
}
