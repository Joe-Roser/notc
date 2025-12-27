use std::collections::HashMap;
use std::rc::Rc;

use crate::parsing::SpannedAstTree;
use crate::traits;
use crate::types::resolved_types::{
    IdentifierId, ResolvedChunk, ResolvedExpression, ResolvedIdentifier, ResolvedParam,
    ResolvedStatement,
};
use crate::types::spanned_types::{
    SpannedChunk, SpannedExpression, SpannedIdentifier, SpannedStatement,
};

#[derive(Debug)]
pub struct ResolvedAstTree {
    pub body: Vec<ResolvedChunk>,
}
impl traits::AstNodeTrait for ResolvedAstTree {}

#[derive(Debug)]
pub struct NameResolver {
    input: Rc<str>,
    map: HashMap<Rc<str>, IdentifierId>,
}

impl NameResolver {
    pub fn from_rc_str(input: Rc<str>) -> Self {
        return NameResolver {
            input,
            map: HashMap::new(),
        };
    }

    pub fn pre_intern(mut self, pre_intern: &[&str]) -> Self {
        for s in pre_intern {
            let id = IdentifierId(self.map.len());
            self.map.insert((*s).into(), id);
        }

        return self;
    }

    pub fn resolve(&mut self, ast: SpannedAstTree) -> ResolvedAstTree {
        let mut new_body = Vec::new();
        for chunk in ast.body {
            new_body.push(self.resolve_chunk(chunk));
        }

        return ResolvedAstTree { body: new_body };
    }

    fn intern(&mut self, identifier: SpannedIdentifier) -> ResolvedIdentifier {
        let name = &self.input[identifier.span.clone()];
        if let Some(id) = self.map.get(name) {
            return ResolvedIdentifier {
                span: identifier.span,
                id: *id,
            };
        }

        let id = IdentifierId(self.map.len());
        self.map.insert(name.into(), id);

        return ResolvedIdentifier {
            span: identifier.span,
            id,
        };
    }
}

impl NameResolver {
    fn resolve_chunk(&mut self, chunk: SpannedChunk) -> ResolvedChunk {
        match chunk {
            SpannedChunk::Constant => todo!(),
            SpannedChunk::StaticVar => todo!(),
            SpannedChunk::Function {
                name,
                params,
                rtype,
                body,
                span,
            } => {
                let name = self.intern(name);
                let mut new_params = Vec::new();
                for param in params {
                    let name = self.intern(param.name);
                    let ptype = self.intern(param.ptype);
                    new_params.push(ResolvedParam { name, ptype });
                }
                let rtype = self.intern(rtype);
                let body = self.resolve_statement(body);

                return ResolvedChunk::Function {
                    name,
                    params: new_params,
                    rtype,
                    body,
                    span,
                };
            }
            SpannedChunk::EOF => panic!(),
        }
    }

    fn resolve_statement(&mut self, st: SpannedStatement) -> ResolvedStatement {
        match st {
            SpannedStatement::Decleration {
                name,
                expression,
                rtype,
                span,
            } => ResolvedStatement::Decleration {
                name: self.intern(name),
                expression: Box::new(self.resolve_expression(*expression)),
                rtype: self.intern(rtype),
                span,
            },
            SpannedStatement::Reassignment {
                name,
                expression,
                span,
            } => ResolvedStatement::Reassignment {
                name: self.intern(name),
                expression: Box::new(self.resolve_expression(*expression)),
                span,
            },
            SpannedStatement::If {
                condition,
                statement,
                ielse,
                span,
            } => ResolvedStatement::If {
                condition: Box::new(self.resolve_expression(*condition)),
                statement: Box::new(self.resolve_statement(*statement)),
                ielse: ielse.map(|s| Box::new(self.resolve_statement(*s))),
                span,
            },
            SpannedStatement::Scope { body, span } => {
                let mut new_body = Vec::new();
                for st in body {
                    new_body.push(self.resolve_statement(st));
                }

                ResolvedStatement::Scope {
                    body: new_body,
                    span,
                }
            }
            SpannedStatement::VoidCall { name, params, span } => {
                let mut new_params = Vec::new();
                for p in params {
                    new_params.push(self.resolve_expression(p));
                }

                ResolvedStatement::VoidCall {
                    name: self.intern(*name),
                    params: new_params,
                    span,
                }
            }
        }
    }

    fn resolve_expression(&mut self, ex: SpannedExpression) -> ResolvedExpression {
        match ex {
            SpannedExpression::Identifier(id) => ResolvedExpression::Identifier(self.intern(id)),
            SpannedExpression::Literal { span } => {
                todo!("{:?}", span);
                // ResolvedExpression::Literal { span }
            }
            SpannedExpression::UnaryOperator {
                operation,
                expression,
            } => ResolvedExpression::UnaryOperator {
                operation,
                expression: Box::new(self.resolve_expression(*expression)),
            },
            SpannedExpression::BinaryOperator {
                left,
                span,
                presedence,
                right,
            } => ResolvedExpression::BinaryOperator {
                left: Box::new(self.resolve_expression(*left)),
                span,
                presedence,
                right: Box::new(self.resolve_expression(*right)),
            },
            SpannedExpression::Call { name, params, span } => todo!(),
            SpannedExpression::Tmp => panic!(),
        }
    }
}
