use crate::{
    traits::{DebugLexerTrait, LexerTrait},
    types::spanned_types::{SpannedIdentifier, Token},
};

use crate::{SYMBOL_MATCHES, TEXT_MATCHES};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Lexer {
    text: Rc<str>,
    index: usize,
}

impl Lexer {
    pub fn from_string(text: String) -> Lexer {
        return Lexer {
            text: Rc::from(text.into_boxed_str()),
            index: 0,
        };
    }
    pub fn from_rc_str(text: Rc<str>) -> Lexer {
        return Lexer { text, index: 0 };
    }

    fn peek_char(&self) -> Option<char> {
        return self.text[self.index..].chars().next();
    }
}
#[allow(refining_impl_trait)]
impl LexerTrait<Token> for Lexer {
    fn next_token(&mut self) -> Token {
        while self.index < self.text.len() {
            if self.peek_char().unwrap().is_whitespace() {
                self.index += 1;
            } else {
                break;
            }
        }

        if self.index == self.text.len() {
            return Token::EOF(self.index..self.index);
        }

        let start = self.index;
        let c = self.peek_char().unwrap();

        if c.is_alphabetic() || c == '_' {
            self.index += 1;
            // Grab whole identifer
            while self.index < self.text.len() {
                let c = self.peek_char().unwrap();
                if !c.is_alphabetic() && c != '_' {
                    break;
                }
                self.index += 1;
            }
            // Iterate over all possible non-identifier tokens
            for (str, t) in TEXT_MATCHES {
                if *str == &self.text[start..self.index] {
                    return t.clone().set_span(start..self.index);
                }
            }
            return Token::Identifier(SpannedIdentifier {
                span: start..self.index,
            });
        }

        if c.is_digit(10) {
            self.index += 1;
            while self.index < self.text.len() {
                let c = self.peek_char().unwrap();
                if !c.is_digit(10) && c != '.' {
                    break;
                }
                self.index += 1;
            }
            return Token::NumericLiteral(start..self.index);
        }

        for (symbol, token) in SYMBOL_MATCHES {
            if self.text[self.index..].starts_with(symbol) {
                self.index += symbol.len();
                return token.clone().set_span(start..self.index);
            }
        }

        // TODO: String and character literals
        return Token::Unknown(start..self.index);
    }

    fn peek_next(&self) -> (Token, usize) {
        let mut clone = self.clone();
        let token = clone.next_token();
        let i = clone.index;
        return (token, i);
    }

    fn go_to(&mut self, i: usize) -> () {
        self.index = i;
    }
}

impl DebugLexerTrait<Token> for Lexer {
    fn next_token_dbg(&mut self) -> Token {
        let a = self.next_token();
        dbg!(&a);
        dbg!(&self.text[a.get_span()]);

        return a;
    }

    fn recap(&self) -> &str {
        return &self.text[0..self.index];
    }

    fn get_index(&self) -> usize {
        return self.index;
    }
}
