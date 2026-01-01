use super::Span;

// Spanned Nodes
//
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpannedIdentifier {
    pub span: Span,
}
#[derive(Debug, Clone)]
pub struct SpannedOperator {
    pub span: Span,
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
        span: Span,
    },
    EOF,
}
#[derive(Debug)]
pub enum SpannedStatement {
    Decleration {
        name: SpannedIdentifier,
        expression: Box<SpannedExpression>,
        rtype: SpannedIdentifier,
        span: Span,
    },
    Reassignment {
        name: SpannedIdentifier,
        expression: Box<SpannedExpression>,
        span: Span,
    },
    If {
        condition: Box<SpannedExpression>,
        statement: Box<SpannedStatement>,
        ielse: Option<Box<SpannedStatement>>,
        span: Span,
    },
    Scope {
        body: Vec<SpannedStatement>,
        span: Span,
    },
    VoidCall {
        name: Box<SpannedIdentifier>,
        params: Vec<SpannedExpression>,
        span: Span,
    },
    Return {
        expr: Option<Box<SpannedExpression>>,
        span: Span,
    },
}
#[derive(Debug, PartialEq, Eq)]
pub enum SpannedExpression {
    Identifier(SpannedIdentifier),
    Literal {
        span: Span,
    },
    UnaryOperator {
        operation: Span,
        expression: Box<SpannedExpression>,
    },
    BinaryOperator {
        left: Box<SpannedExpression>,
        span: Span,
        presedence: usize,
        right: Box<SpannedExpression>,
    },
    Call {
        name: Box<SpannedIdentifier>,
        params: Vec<SpannedExpression>,
        span: Span,
    },
    Tmp,
}
