use std::{fs::File, io::Write, rc::Rc};

use crate::{
    traits,
    tree_checker::ResolvedAstTree,
    types::resolved_types::{IdentifierId, ResolvedChunk, ResolvedExpression, ResolvedStatement},
};

const PRIMITIVE_MAP: &[&str] = &["void", "bool", "unsigned int"];
const PREAMBLE: &str = "typedef int bool;
enum {
    false,
    true
};\n\n";

pub struct CCodeGen {
    target_file: File,
    source_file: Rc<str>,
}

impl CCodeGen {
    pub fn new(input: Rc<str>, file: File) -> Self {
        return CCodeGen {
            source_file: input,
            target_file: file,
        };
    }
}

impl traits::CodeGen<ResolvedAstTree> for CCodeGen {
    fn generate(mut self, ast: &ResolvedAstTree) -> Result<(), std::io::Error> {
        write!(self.target_file, "{}", PREAMBLE)?;
        for chunk in &ast.body {
            self.generate_chunk(chunk)?;
        }

        Ok(())
    }
}
impl CCodeGen {
    fn generate_chunk(&mut self, chunk: &ResolvedChunk) -> Result<(), std::io::Error> {
        match chunk {
            ResolvedChunk::Constant => todo!(),
            ResolvedChunk::StaticVar => todo!(),
            ResolvedChunk::Function {
                name,
                params,
                rtype,
                body,
                ..
            } => {
                write!(
                    self.target_file,
                    "{} {}(",
                    resolve_type(&rtype.id),
                    &self.source_file[name.span.clone()]
                )?;

                for i in 0..params.len() {
                    write!(
                        self.target_file,
                        "{} {}",
                        resolve_type(&params[i].ptype.id),
                        &self.source_file[params[i].name.span.clone()]
                    )?;
                    if i < params.len() - 1 {
                        write!(self.target_file, ", ")?;
                    }
                }

                write!(self.target_file, ")")?;
                self.generate_statement(body)?;
                write!(self.target_file, "\n")?;

                Ok(())
            }
            ResolvedChunk::EOF => panic!(),
        }
    }
    fn generate_statement(&mut self, statemet: &ResolvedStatement) -> Result<(), std::io::Error> {
        match statemet {
            ResolvedStatement::Decleration {
                name,
                expression,
                rtype,
                ..
            } => {
                write!(
                    self.target_file,
                    "{} {} = ",
                    resolve_type(&rtype.id),
                    &self.source_file[name.span.clone()]
                )?;

                self.generate_expression(expression)?;

                write!(self.target_file, ";\n")?;
            }
            ResolvedStatement::Reassignment {
                name,
                expression,
                span,
            } => {
                write!(
                    self.target_file,
                    "{} {} ",
                    &self.source_file[name.span.clone()],
                    &self.source_file[span.clone()]
                )?;

                self.generate_expression(expression)?;

                write!(self.target_file, ";\n")?;
            }
            ResolvedStatement::If {
                condition,
                statement,
                ielse,
                ..
            } => {
                write!(self.target_file, "if (")?;
                self.generate_expression(condition)?;
                write!(self.target_file, ")")?;
                self.generate_statement(statement)?;

                if let Some(st) = ielse {
                    write!(self.target_file, " else ")?;
                    self.generate_statement(st)?;
                }
            }
            ResolvedStatement::Scope { body, .. } => {
                write!(self.target_file, "{{\n")?;
                for st in body {
                    self.generate_statement(st)?;
                }
                write!(self.target_file, "}}\n")?;
            }
            ResolvedStatement::VoidCall { name, params, .. } => {
                write!(
                    self.target_file,
                    "{}(",
                    &self.source_file[name.span.clone()],
                )?;
                for i in 0..params.len() {
                    self.generate_expression(&params[i])?;
                    if i < params.len() - 1 {
                        write!(self.target_file, ", ")?;
                    }
                }
                write!(self.target_file, ");\n")?;
            }
            ResolvedStatement::Return { expression, .. } => {
                write!(self.target_file, "return ")?;
                if let Some(expression) = expression {
                    self.generate_expression(expression)?;
                }
                write!(self.target_file, ";\n")?;
            }
        }

        Ok(())
    }
    fn generate_expression(
        &mut self,
        expression: &ResolvedExpression,
    ) -> Result<(), std::io::Error> {
        match expression {
            ResolvedExpression::Identifier(resolved_identifier) => write!(
                self.target_file,
                "{}",
                &self.source_file[resolved_identifier.span.clone()]
            )?,
            ResolvedExpression::Literal { span } => todo!(),
            ResolvedExpression::UnaryOperator {
                operation,
                expression,
            } => {
                write!(self.target_file, "{}", &self.source_file[operation.clone()])?;
                self.generate_expression(&expression)?;
            }
            ResolvedExpression::BinaryOperator {
                left, span, right, ..
            } => {
                self.generate_expression(&left)?;
                write!(self.target_file, " {} ", &self.source_file[span.clone()])?;
                self.generate_expression(&right)?;
            }
            ResolvedExpression::Call => todo!(),
            ResolvedExpression::Tmp => panic!(),
        }

        Ok(())
    }
}

fn resolve_type(id: &IdentifierId) -> &str {
    match id {
        IdentifierId(i) if i < &3 => return PRIMITIVE_MAP[*i],
        _ => {}
    }
    "other"
}

//
