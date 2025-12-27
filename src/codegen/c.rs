use std::{fs::File, io::Write, rc::Rc};

use crate::{
    traits,
    tree_checker::ResolvedAstTree,
    types::resolved_types::{IdentifierId, ResolvedChunk, ResolvedExpression, ResolvedStatement},
};

const PRIMITIVE_MAP: &[&str] = &["void", "bool", "size_t"];

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
    fn generate(mut self, ast: &ResolvedAstTree) {
        for chunk in &ast.body {
            self.generate_chunk(chunk);
        }
    }
}
impl CCodeGen {
    fn generate_chunk(&mut self, chunk: &ResolvedChunk) {
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
                )
                .unwrap();

                for i in 0..params.len() {
                    write!(
                        self.target_file,
                        "{} {}",
                        resolve_type(&params[i].ptype.id),
                        &self.source_file[params[i].name.span.clone()]
                    )
                    .unwrap();
                    if i < params.len() - 1 {
                        write!(self.target_file, ", ").unwrap();
                    }
                }

                write!(self.target_file, ") {{\n").unwrap();
                self.generate_statement(body);
            }
            ResolvedChunk::EOF => panic!(),
        }
    }
    fn generate_statement(&self, statemets: &ResolvedStatement) {
        todo!()
    }
    fn generate_expression(&self, expression: &ResolvedExpression) {
        todo!()
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
