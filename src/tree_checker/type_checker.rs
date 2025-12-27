use std::{collections::HashMap, ops::Range, rc::Rc};

use crate::{
    traits::TreeChecker,
    tree_checker::name_resolver::ResolvedAstTree,
    types::resolved_types::{IdentifierId, ResolvedChunk, ResolvedExpression, ResolvedStatement},
};

pub struct TypeChecker {
    scope: Scope,
}
impl TypeChecker {
    pub fn new() -> Self {
        return TypeChecker {
            scope: Scope::new(),
        };
    }
    pub fn debug_check_result(r: &Result<(), TypeError>, input: Rc<str>) {
        let s;
        match r {
            Ok(()) => {
                println!("Ok");
                return;
            }
            Err(e) => s = e,
        }
        match s {
            TypeError::DeclarationMatch(range) => println!(
                "Err: The declaration doesn't match the expression: {}",
                &input[range.clone()]
            ),
            TypeError::AssignmentMatch(range) => println!(
                "Err: The original declaration doesn't match the expression: {}",
                &input[range.clone()]
            ),
            TypeError::NotDeclared(range) => {
                println!("Err: Variable wasn't declared: {}", &input[range.clone()])
            }
            TypeError::NotVoid(range) => {
                println!(
                    "Err: Function return value is not used: {}",
                    &input[range.clone()]
                )
            }
            TypeError::TypeMismatch(range) => {
                println!(
                    "Err: Types do not match, if statements require booleans: {}",
                    &input[range.clone()]
                )
            }
            TypeError::ParamTypes(range) => {
                println!(
                    "Err: Param types are dont match function declaration: {}",
                    &input[range.clone()]
                )
            }
        }
    }

    fn insert(&mut self, k: IdentifierId, v: TypeId) {
        self.scope.variables.insert(k, v);
    }
    fn get(&self, k: &IdentifierId) -> Option<&TypeId> {
        return self.scope.get(k);
    }
}

// Scopes
//
#[derive(Debug)]
struct Scope {
    variables: HashMap<IdentifierId, TypeId>,
    parent: Option<Box<Scope>>,
}

impl Scope {
    fn new() -> Scope {
        return Scope {
            variables: HashMap::new(),
            parent: None,
        };
    }

    fn push(&mut self) {
        let parent = std::mem::replace(self, Scope::new());
        self.parent = Some(Box::new(parent));
    }
    fn pop(&mut self) -> Option<()> {
        if let Some(s) = self.parent.take() {
            *self = *s;
            return Some(());
        }
        return None;
    }
    fn get(&self, k: &IdentifierId) -> Option<&TypeId> {
        match (self.variables.get(k), &self.parent) {
            (None, None) => return None,
            (None, Some(p)) => return p.get(k),
            (v, _) => return v,
        }
    }
}

// TypeError
//
#[derive(Debug, PartialEq, Eq)]
pub enum TypeError {
    DeclarationMatch(Range<usize>),
    AssignmentMatch(Range<usize>),
    NotDeclared(Range<usize>),
    NotVoid(Range<usize>),
    TypeMismatch(Range<usize>),
    ParamTypes(Range<usize>),
}

// TypeId
//
#[derive(Debug, PartialEq, Eq)]
pub enum TypeId {
    Variable(IdentifierId),
    Fn {
        params: Vec<IdentifierId>,
        ret: Box<IdentifierId>,
    },
}
impl PartialEq<TypeId> for &TypeId {
    fn eq(&self, other: &TypeId) -> bool {
        self == other
    }

    fn ne(&self, other: &TypeId) -> bool {
        !self.eq(other)
    }
}

// TypeChecker
//
impl TreeChecker<ResolvedAstTree> for TypeChecker {
    type CheckError = TypeError;

    fn check(&mut self, ast: &ResolvedAstTree) -> Result<(), Self::CheckError> {
        for chunk in &ast.body {
            match chunk {
                ResolvedChunk::Constant => todo!(),
                ResolvedChunk::StaticVar => todo!(),
                ResolvedChunk::Function {
                    name,
                    params,
                    rtype,
                    ..
                } => {
                    let params = params.iter().map(|p| p.ptype.id).collect();
                    self.insert(
                        name.id,
                        TypeId::Fn {
                            params,
                            ret: Box::new(rtype.id),
                        },
                    )
                }
                ResolvedChunk::EOF => panic!(),
            }
        }
        for chunk in &ast.body {
            self.check_chunk(chunk)?;
        }
        assert!(self.scope.parent.is_none());
        Ok(())
    }
}

impl TypeChecker {
    fn check_chunk(&mut self, chunk: &ResolvedChunk) -> Result<(), TypeError> {
        match chunk {
            ResolvedChunk::Constant => todo!(),
            ResolvedChunk::StaticVar => todo!(),
            ResolvedChunk::Function { body, params, .. } => {
                self.scope.push();

                params
                    .iter()
                    .for_each(|p| self.insert(p.name.id, TypeId::Variable(p.ptype.id)));
                self.check_statement(body)?;

                self.scope.pop().unwrap();
                Ok(())
            }
            ResolvedChunk::EOF => todo!(),
        }
    }
    fn check_statement(&mut self, statement: &ResolvedStatement) -> Result<(), TypeError> {
        match statement {
            ResolvedStatement::Decleration {
                name,
                expression,
                rtype,
                span,
            } => {
                if &TypeId::Variable(rtype.id) != self.check_expression(expression)? {
                    return Err(TypeError::DeclarationMatch(span.clone()));
                }
                self.insert(name.id, TypeId::Variable(rtype.id));
                Ok(())
            }
            ResolvedStatement::Reassignment {
                name,
                expression,
                span,
            } => {
                let dtype = self.get(&name.id);
                if dtype.is_none() {
                    return Err(TypeError::NotDeclared(span.clone()));
                }
                if dtype.unwrap() != self.check_expression(expression)? {
                    return Err(TypeError::AssignmentMatch(span.clone()));
                }
                Ok(())
            }
            ResolvedStatement::If {
                condition,
                statement,
                ielse,
                span,
            } => {
                if self.get(&IdentifierId(1)).unwrap() != self.check_expression(&condition)? {
                    return Err(TypeError::TypeMismatch(span.clone()));
                }
                self.check_statement(statement)?;
                if let Some(st) = ielse {
                    self.check_statement(st)?;
                }

                Ok(())
            }
            ResolvedStatement::Scope { body, .. } => {
                self.scope.push();
                for statement in body {
                    self.check_statement(statement)?;
                }
                self.scope.pop().unwrap();

                Ok(())
            }
            ResolvedStatement::VoidCall { name, params, span } => {
                let tid;
                match self.get(&name.id) {
                    Some(type_id) => tid = type_id,
                    None => return Err(TypeError::NotDeclared(span.clone())),
                }

                let def_ret;
                let def_params: Vec<_>;
                match tid {
                    TypeId::Fn { params, ret } => {
                        def_ret = ret;
                        def_params = params.iter().map(|p| self.get(p).unwrap()).collect();
                    }
                    _ => return Err(TypeError::TypeMismatch(span.clone())),
                }

                if **def_ret != IdentifierId(0) {
                    return Err(TypeError::NotVoid(span.clone()));
                }

                let mut call_params = Vec::new();
                for p in params {
                    call_params.push(self.check_expression(p)?);
                }
                if def_params != call_params {
                    return Err(TypeError::ParamTypes(span.clone()));
                }
                todo!()
            }
        }
    }
    fn check_expression(&self, expression: &ResolvedExpression) -> Result<&TypeId, TypeError> {
        match expression {
            ResolvedExpression::Identifier(resolved_identifier) => self
                .get(&resolved_identifier.id)
                .ok_or(TypeError::NotDeclared(resolved_identifier.span.clone())),
            ResolvedExpression::Literal { span } => todo!(),
            ResolvedExpression::UnaryOperator { expression, .. } => {
                self.check_expression(expression)
            }
            ResolvedExpression::BinaryOperator {
                left,
                right,
                presedence,
                span,
            } => {
                let left = self.check_expression(left)?;
                let right = self.check_expression(right)?;
                return Ok(left);
            }
            ResolvedExpression::Call => todo!(),
            ResolvedExpression::Tmp => panic!(),
        }
    }
}
