use super::ParseError;

use crate::types::{Token, spanned_types::SpannedChunk};

pub use std::{ops::Range, rc::Rc};

#[derive(Debug)]
pub struct SpannedAstTree {
    pub(super) input: Rc<str>,
    pub(crate) body: Vec<SpannedChunk>,
}

impl SpannedAstTree {
    pub fn from_rc_str(input: Rc<str>) -> SpannedAstTree {
        return SpannedAstTree {
            input,
            body: Vec::new(),
        };
    }

    pub fn debug_ast_result(&self, r: &Result<(), ParseError>) -> () {
        match r {
            Ok(()) => _ = {},
            Err(e) => match e {
                ParseError::UnknownToken(s) => println!("{}", &self.input[s.clone()]),
                ParseError::BadSyntax(b, msg) => {
                    print!("{}, found ", msg);
                    match b {
                        Token::NumericLiteral(range) => {
                            println!("\"{}\": {:?}", &self.input[range.clone()], range)
                        }
                        Token::Identifier(id) => {
                            println!("\"{}\", {:?}", &self.input[id.span.clone()], id.span)
                        }
                        Token::Operator(op) => {
                            println!("\"{}\", {:?}", &self.input[op.span.clone()], op.span)
                        }
                        t => {
                            println!("{:?}", t);
                        }
                    }
                }
            },
        }
    }

    pub(super) fn resolve_span<'a>(&'a self, span: Range<usize>) -> &'a str {
        return &self.input[span];
    }
}
