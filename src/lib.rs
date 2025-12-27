pub mod codegen;
pub mod lexer;
pub mod parsing;
pub mod tree_checker;
pub mod types;

// generics
pub mod traits;

//
use types::spanned_types::{SpannedOperator, Token};

pub const TEXT_MATCHES: &[(&'static str, Token)] = &[
    ("let", Token::Let(0..0)),
    ("if", Token::If(0..0)),
    ("fn", Token::Fn(0..0)),
    ("const", Token::Const(0..0)),
    ("static", Token::Static(0..0)),
    ("else", Token::Else(0..0)),
];
pub const SYMBOL_MATCHES: &[(&'static str, Token)] = &[
    //Symbols
    (",", Token::Comma(0..0)),
    (":", Token::Colon(0..0)),
    (";", Token::EOL(0..0)),
    ("(", Token::LParen(0..0)),
    (")", Token::RParen(0..0)),
    ("{", Token::LBrace(0..0)),
    ("}", Token::RBrace(0..0)),
    ("->", Token::LArrow(0..0)),
    //
    // Operators
    //
    // Boolean
    (
        "==",
        Token::Operator(SpannedOperator {
            span: 0..0,
            presedence: 1,
        }),
    ),
    (
        "!",
        Token::Operator(SpannedOperator {
            span: 0..0,
            presedence: 5,
        }),
    ),
    // Assignments
    (
        "=",
        Token::Operator(SpannedOperator {
            span: 0..0,
            presedence: 0,
        }),
    ),
    (
        "+=",
        Token::Operator(SpannedOperator {
            span: 0..0,
            presedence: 0,
        }),
    ),
    (
        "-=",
        Token::Operator(SpannedOperator {
            span: 0..0,
            presedence: 0,
        }),
    ),
    (
        "*=",
        Token::Operator(SpannedOperator {
            span: 0..0,
            presedence: 0,
        }),
    ),
    (
        "/=",
        Token::Operator(SpannedOperator {
            span: 0..0,
            presedence: 0,
        }),
    ),
    // Binary Operations
    (
        "+",
        Token::Operator(SpannedOperator {
            span: 0..0,
            presedence: 2,
        }),
    ),
    (
        "-",
        Token::Operator(SpannedOperator {
            span: 0..0,
            presedence: 2,
        }),
    ),
    (
        "*",
        Token::Operator(SpannedOperator {
            span: 0..0,
            presedence: 3,
        }),
    ),
    (
        "/",
        Token::Operator(SpannedOperator {
            span: 0..0,
            presedence: 3,
        }),
    ),
];
pub const PRIMATIVE_TYPES: &[&str] = &["void", "bool", "usize"];
