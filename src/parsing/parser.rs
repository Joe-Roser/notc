pub use super::SpannedAstTree;
use std::{fmt::Debug, ops::Range};

use crate::{
    traits,
    types::{
        Token,
        spanned_types::{
            SpannedChunk, SpannedExpression, SpannedIdentifier, SpannedParam, SpannedStatement,
        },
    },
};

#[derive(Debug)]
pub enum ParseError {
    UnknownToken(Range<usize>),
    BadSyntax(Token, &'static str),
}

macro_rules! parse_error {
    ($tok:expr, $msg:literal) => {
        Err(match $tok {
            Token::Unknown(s) => ParseError::UnknownToken(s),
            b => ParseError::BadSyntax(b, $msg),
        })
    };
}

fn is_unary_ooperator(t: &str) -> bool {
    return match t {
        "!" => true,
        "-" => true,
        _ => false,
    };
}

impl SpannedAstTree {
    pub fn parse_all(
        &mut self,
        lexer: &mut (impl traits::LexerTrait<Token> + traits::DebugLexerTrait<Token> + Debug),
    ) -> Result<(), ParseError> {
        loop {
            match self.parse_chunk(lexer)? {
                SpannedChunk::EOF => break,
                chunk => {
                    self.body.push(chunk);
                }
            }
        }
        return Ok(());
    }

    //
    // Parse Chunks
    //
    pub fn parse_chunk(
        &mut self,
        lexer: &mut (impl traits::LexerTrait<Token> + traits::DebugLexerTrait<Token>),
    ) -> Result<SpannedChunk, ParseError> {
        match lexer.next_token() {
            // TODO:
            Token::Const(_s) => todo!(),
            // TODO:
            Token::Static(_s) => todo!(),
            // File Function Declarations
            Token::Fn(s) => {
                let fn_identifier: SpannedIdentifier;
                match lexer.next_token() {
                    Token::Identifier(i) => fn_identifier = i,
                    b => {
                        return parse_error!(b, "Function Needs Identifier");
                    }
                }

                match lexer.next_token() {
                    Token::LParen(_) => {}
                    b => {
                        return parse_error!(
                            b,
                            "Function Definition requires parentheses, expected LParen"
                        );
                    }
                }

                // Parse in the parameters, an optional number of name: Type, separated by commas
                let mut params: Vec<SpannedParam> = Vec::new();
                let e;
                match lexer.next_token() {
                    Token::RParen(a) => e = a.end,
                    Token::Identifier(mut name) => loop {
                        match lexer.next_token() {
                            Token::Colon(_) => {}
                            b => {
                                return parse_error!(
                                    b,
                                    "Function parameters require types, eg(a: Type)"
                                );
                            }
                        }

                        let ptype;
                        match lexer.next_token() {
                            Token::Identifier(i) => ptype = i,
                            b => {
                                return parse_error!(
                                    b,
                                    "Function parameters require types, eg(a: Type)"
                                );
                            }
                        }

                        params.push(SpannedParam { name, ptype });

                        match lexer.next_token() {
                            Token::RParen(s) => {
                                e = s.end;
                                break;
                            }
                            Token::Comma(_) => {}
                            b => {
                                return parse_error!(
                                    b,
                                    "Function parameters are seperated by commas and ended by a closing parenthesis"
                                );
                            }
                        }

                        match lexer.next_token() {
                            Token::Identifier(i) => name = i,
                            b => {
                                return parse_error!(
                                    b,
                                    "function parameters are made of characters or _"
                                );
                            }
                        }
                    },
                    b => {
                        return parse_error!(
                            b,
                            "If no parameters, close parentheses, else, list parameters"
                        );
                    }
                }
                let rtype;
                match lexer.next_token() {
                    Token::Identifier(id) => rtype = id,
                    b => {
                        return parse_error!(
                            b,
                            "Please put a return type at the end of function declarations"
                        );
                    }
                }

                match lexer.next_token() {
                    Token::LArrow(_) => {}
                    b => {
                        return parse_error!(
                            b,
                            "Please put '-> between return type and function body"
                        );
                    }
                }

                // Parse in the function body
                let body = self.parse_statement(lexer)?;

                return Ok(SpannedChunk::Function {
                    name: fn_identifier,
                    params,
                    rtype,
                    body,
                    span: s.start..e,
                });
            }
            Token::EOF(_) => return Ok(SpannedChunk::EOF),
            // Miss
            b => {
                return parse_error!(
                    b,
                    "Chunks are either constants, static declerations, or functions"
                );
            }
        }
    }

    //
    // Parse Statements
    //
    pub fn parse_statement(
        &mut self,
        lexer: &mut (impl traits::LexerTrait<Token> + traits::DebugLexerTrait<Token>),
    ) -> Result<SpannedStatement, ParseError> {
        match lexer.next_token() {
            // Declaration
            Token::Let(span) => {
                let name;
                match lexer.next_token() {
                    Token::Identifier(i) => name = i,
                    b => {
                        return parse_error!(b, "Let statement followed by identifier");
                    }
                }

                match lexer.next_token() {
                    Token::Colon(_) => {}
                    b => return parse_error!(b, "Please provide type for definition 'v: type'"),
                }

                let rtype;
                match lexer.next_token() {
                    Token::Identifier(i) => rtype = i,
                    b => {
                        return parse_error!(b, "Please annotate with type");
                    }
                }

                match lexer.next_token() {
                    Token::Operator(o) if self.resolve_span(o.span.clone()) == "=" => {}
                    b => {
                        return parse_error!(b, "Please indicate an initial value using '='");
                    }
                }

                let expression = self.parse_expression(lexer)?;
                let expression = Box::new(expression);

                match lexer.next_token() {
                    Token::EOL(_) => {}
                    b => {
                        return parse_error!(b, "Please end statement with ';'");
                    }
                }

                return Ok(SpannedStatement::Decleration {
                    name,
                    expression,
                    rtype,
                    span,
                });
            }
            // If Statement
            Token::If(s) => {
                match lexer.next_token() {
                    Token::LParen(_) => {}
                    b => {
                        return parse_error!(b, "Please put condition in parentheses");
                    }
                }

                let condition = self.parse_expression(lexer)?;
                let condition = Box::new(condition);

                let e;
                match lexer.next_token() {
                    Token::RParen(s) => e = s.end,
                    b => {
                        return parse_error!(b, "Please close parentheses after condition");
                    }
                }
                let statement = self.parse_statement(lexer)?;
                let statement = Box::new(statement);

                // Else Statements
                match lexer.peek_next() {
                    (Token::Else(_), i) => {
                        lexer.go_to(i);

                        let ielse = self.parse_statement(lexer)?;
                        let ielse = Some(Box::new(ielse));

                        return Ok(SpannedStatement::If {
                            condition,
                            statement,
                            ielse,
                            span: s.start..e,
                        });
                    }
                    _ => {
                        return Ok(SpannedStatement::If {
                            condition,
                            statement,
                            ielse: None,
                            span: s.start..e,
                        });
                    }
                }
            }
            // Scope
            Token::LBrace(s) => {
                let mut body = Vec::new();
                let e;
                loop {
                    if let (Token::RBrace(s), i) = lexer.peek_next() {
                        e = s.end;
                        lexer.go_to(i);
                        break;
                    }
                    body.push(self.parse_statement(lexer)?);
                }

                return Ok(SpannedStatement::Scope {
                    body,
                    span: s.start..e,
                });
            }
            // Identifier Branch
            Token::Identifier(name) => {
                // Collect the statement
                let st;
                match lexer.next_token() {
                    // Function Call
                    Token::LParen(_) => {
                        let mut params = Vec::new();
                        let e;
                        match lexer.peek_next() {
                            (Token::RParen(s), i) => {
                                e = s.end;
                                lexer.go_to(i);
                            }
                            _ => loop {
                                params.push(self.parse_expression(lexer)?);
                                match lexer.next_token() {
                                    Token::Comma(_) => {}
                                    Token::RParen(s) => {
                                        e = s.end;
                                        break;
                                    }
                                    b => {
                                        return parse_error!(
                                            b,
                                            "Either terminate parameter calls, or seperate them with a comma"
                                        );
                                    }
                                }
                            },
                        }

                        st = Ok(SpannedStatement::VoidCall {
                            span: name.span.start..e,
                            name: Box::new(name),
                            params,
                        });
                    }
                    // Reassignments
                    Token::Operator(op)
                        if ["=", "+=", "-=", "*=", "/="]
                            .contains(&self.resolve_span(op.span.clone())) =>
                    {
                        let expression = Box::new(self.parse_expression(lexer)?);
                        st = Ok(SpannedStatement::Reassignment {
                            name,
                            expression,
                            span: op.span,
                        });
                    }
                    b => {
                        return parse_error!(
                            b,
                            "Invalid token after identifier, either make function call or reassign variable"
                        );
                    }
                }
                // Return if it ends with an EOL
                match lexer.next_token() {
                    Token::EOL(_) => return st,
                    b => {
                        return parse_error!(b, "Please end statements in ';'");
                    }
                }
                // End of identifier branch
            }
            Token::Return(s1) => {
                let expr;
                match self.parse_expression(lexer) {
                    Ok(e) => expr = Some(Box::new(e)),
                    Err(ParseError::BadSyntax(t, _)) if matches!(t, Token::EOL(_)) => {
                        return Ok(SpannedStatement::Return {
                            expr: None,
                            span: s1.start..t.get_span().end,
                        });
                    }
                    Err(b) => return Err(b),
                }
                match lexer.next_token() {
                    Token::EOL(s2) => {
                        return Ok(SpannedStatement::Return {
                            expr,
                            span: s1.start..s2.end,
                        });
                    }
                    b => return parse_error!(b, "Please end statements in ';'"),
                }
            }
            // Miss
            b => {
                return parse_error!(b, "Invalid statement start");
            }
        }
    }

    //
    // Parse Expressions
    //
    pub fn parse_expression(
        &mut self,
        lexer: &mut (impl traits::LexerTrait<Token> + traits::DebugLexerTrait<Token>),
    ) -> Result<SpannedExpression, ParseError> {
        // First value in expression
        let left;
        match lexer.next_token() {
            Token::Operator(op) => match self.resolve_span(op.span.clone()) {
                // Complicated Logic
                // Want to make the ! operator have less presedence than any binary operator. May
                // do for all unary operators, just have to think about it
                t if is_unary_ooperator(t) => {
                    // Get a mutable expression
                    let mut expression = self.parse_expression(lexer)?;
                    // Get a mutable reference to it
                    let mut node = &mut expression;

                    // Get the left most child as a mutable reference
                    while let SpannedExpression::BinaryOperator { left, .. } = node {
                        node = left.as_mut();
                    }

                    // Swip swap a temp in as the value
                    let old = std::mem::replace(node, SpannedExpression::Tmp);

                    // Replace the temp with the new, wrapped value!!!
                    *node = SpannedExpression::UnaryOperator {
                        operation: op.span,
                        expression: Box::new(old),
                    };

                    // Was really lifetime tricky to implement
                    return Ok(expression);
                }
                _ => {
                    return parse_error!(
                        Token::Operator(op),
                        "Invalid operator to start expression"
                    );
                }
            },
            // TODO: Expression function calls
            Token::Identifier(identifier) => {
                match lexer.peek_next() {
                    (Token::LParen(span), i) => {
                        // Function call
                        let s = span.start;
                        lexer.go_to(i);

                        let mut params = Vec::new();
                        let e;
                        match lexer.peek_next() {
                            (Token::RParen(s), i) => {
                                e = s.end;
                                lexer.go_to(i);
                            }
                            _ => loop {
                                params.push(self.parse_expression(lexer)?);
                                match lexer.next_token() {
                                    Token::Comma(_) => {}
                                    Token::RParen(s) => {
                                        e = s.end;
                                        break;
                                    }
                                    b => {
                                        return parse_error!(
                                            b,
                                            "Either terminate parameter calls, or seperate them with a comma"
                                        );
                                    }
                                }
                            },
                        }

                        left = SpannedExpression::Call {
                            name: Box::new(identifier),
                            params,
                            span: s..e,
                        };
                    }
                    _ => left = SpannedExpression::Identifier(identifier),
                }
            }
            // TODO: Numeric Literals
            Token::NumericLiteral(range) => {
                todo!()
            }
            Token::LParen(s) => {
                let expression = self.parse_expression(lexer)?;
                let expression = Box::new(expression);

                let e;
                match lexer.next_token() {
                    Token::RParen(s) => e = s.end,
                    b => {
                        return parse_error!(b, "No matching closing parentheses");
                    }
                }
                left = SpannedExpression::UnaryOperator {
                    operation: s.start..e,
                    expression,
                };
            }
            // Miss
            b => {
                return parse_error!(b, "Invalid start to expression");
            }
        }

        // We want to collapse operators now. If there is an operator here, then we check if it is
        // valid, then check if it is the only one, else we reorder for presedence
        let left_operation;
        match lexer.peek_next() {
            (Token::Operator(op), i) => {
                lexer.go_to(i);
                left_operation = op;
            }
            _ => {
                return Ok(left);
            }
        }
        let left = Box::new(left);

        match self.parse_expression(lexer)? {
            SpannedExpression::BinaryOperator {
                left: mid,
                span,
                presedence,
                right,
            } => {
                if left_operation.presedence < presedence {
                    return Ok(SpannedExpression::BinaryOperator {
                        left,
                        span,
                        presedence,
                        right: Box::new(SpannedExpression::BinaryOperator {
                            left: mid,
                            span: left_operation.span,
                            presedence: left_operation.presedence,
                            right,
                        }),
                    });
                } else {
                    return Ok(SpannedExpression::BinaryOperator {
                        left: Box::new(SpannedExpression::BinaryOperator {
                            left,
                            span,
                            presedence,
                            right: mid,
                        }),
                        span: left_operation.span,
                        presedence,
                        right,
                    });
                }
            }
            right => {
                return Ok(SpannedExpression::BinaryOperator {
                    left,
                    span: left_operation.span,
                    presedence: left_operation.presedence,
                    right: Box::new(right),
                });
            }
        }
    }
}
