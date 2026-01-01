use std::{collections::HashMap, ops::Range, rc::Rc};

use crate::{
    traits::TreeChecker,
    tree_checker::name_resolver::ResolvedAstTree,
    types::{
        Span,
        resolved_types::{IdentifierId, ResolvedChunk, ResolvedExpression, ResolvedStatement},
    },
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
            TypeError::BadReturnType(range) => {
                println!(
                    "Err: Return type doesn't match declared return type: {}",
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
    DeclarationMatch(Span),
    AssignmentMatch(Span),
    NotDeclared(Span),
    NotVoid(Span),
    TypeMismatch(Span),
    ParamTypes(Span),
    BadReturnType(Span),
}

// TypeId
//
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TypeId {
    Void,
    Bool,
    Usize,
    Fn {
        params: Rc<[TypeId]>,
        ret: Rc<TypeId>,
    },
}
impl PartialEq<TypeId> for &TypeId {
    fn eq(&self, other: &TypeId) -> bool {
        self == &other
    }

    fn ne(&self, other: &TypeId) -> bool {
        !self.eq(other)
    }
}
impl From<IdentifierId> for TypeId {
    fn from(value: IdentifierId) -> Self {
        match value.0 {
            0 => TypeId::Void,
            1 => TypeId::Bool,
            2 => TypeId::Usize,
            _ => panic!(),
        }
    }
}

// TypeChecker
//
impl TreeChecker<ResolvedAstTree> for TypeChecker {
    type CheckError = TypeError;

    fn check(&mut self, ast: &ResolvedAstTree) -> Result<(), Self::CheckError> {
        // Add all the types in the file
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
                    let params = params.iter().map(|p| TypeId::from(p.ptype.id)).collect();
                    self.insert(
                        name.id,
                        TypeId::Fn {
                            params,
                            ret: Rc::new(TypeId::from(rtype.id)),
                        },
                    );
                }
                ResolvedChunk::EOF => panic!(),
            }
        }
        //Check all chunks
        for chunk in &ast.body {
            self.check_chunk(chunk)?;
        }
        // Make sure theres been no funny buisness
        assert!(self.scope.parent.is_none());
        Ok(())
    }
}

// Checking methods for the building blocks of the code
//
impl TypeChecker {
    fn check_chunk(&mut self, chunk: &ResolvedChunk) -> Result<(), TypeError> {
        match chunk {
            ResolvedChunk::Constant => todo!(),
            ResolvedChunk::StaticVar => todo!(),
            ResolvedChunk::Function {
                body,
                params,
                rtype,
                span,
                ..
            } => {
                self.scope.push();

                params
                    .iter()
                    .for_each(|p| self.insert(p.name.id, TypeId::from(p.ptype.id)));

                let rtype = TypeId::from(rtype.id);
                if let Some(v) = self.check_statement(body)? {
                    if v != rtype {
                        return Err(TypeError::BadReturnType(span.clone()));
                    }
                }

                self.scope.pop().unwrap();
                Ok(())
            }
            ResolvedChunk::EOF => todo!(),
        }
    }
    fn check_statement(
        &mut self,
        statement: &ResolvedStatement,
    ) -> Result<Option<&TypeId>, TypeError> {
        match statement {
            ResolvedStatement::Decleration {
                name,
                expression,
                rtype,
                span,
            } => {
                if &TypeId::from(rtype.id) != self.check_expression(expression)? {
                    return Err(TypeError::DeclarationMatch(span.clone()));
                }
                self.insert(name.id, TypeId::from(rtype.id));
                Ok(None)
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
                Ok(None)
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

                Ok(None)
            }
            ResolvedStatement::Scope { body, .. } => {
                self.scope.push();
                for statement in body {
                    self.check_statement(statement)?;
                }
                self.scope.pop().unwrap();

                Ok(None)
            }
            ResolvedStatement::VoidCall { name, params, span } => {
                //TODO: Check this
                let tid;
                match self.get(&name.id) {
                    Some(type_id) => tid = type_id,
                    None => return Err(TypeError::NotDeclared(span.clone())),
                }

                let def_ret;
                let def_params;
                match tid {
                    TypeId::Fn { params, ret } => {
                        def_ret = ret;
                        def_params = params;
                    }
                    _ => return Err(TypeError::TypeMismatch(span.clone())),
                }

                if **def_ret != TypeId::Void {
                    return Err(TypeError::NotVoid(span.clone()));
                }

                let mut call_params = Vec::new();
                for p in params {
                    call_params.push(self.check_expression(p)?);
                }
                if *call_params != **def_params {
                    return Err(TypeError::ParamTypes(span.clone()));
                }
                Ok(None)
            }
            ResolvedStatement::Return { expression, .. } => {
                if let Some(expression) = expression {
                    Ok(Some(self.check_expression(expression)?))
                } else {
                    Ok(Some(&TypeId::Void))
                }
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
                // TODO: Add proper type checking here
                let left = self.check_expression(left)?;
                let right = self.check_expression(right)?;
                return Ok(left);
            }
            ResolvedExpression::Call => todo!(),
            ResolvedExpression::Tmp => panic!(),
        }
    }
}
