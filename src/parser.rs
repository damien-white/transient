//! This module contains transient's core parsing logic.
use std::iter::Peekable;

use crate::kind;
use crate::lexer::{Lexer, Token};

pub mod ast;

#[allow(dead_code)] // TODO: Remove this once methods are implemented.
/// Left-to-right, leftmost derivation parser implementation - LL(1) parser.
pub struct Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    input: &'input str,
    tokens: Peekable<I>,
}

/// Iterator for producing tokens with whitespace and comments stripped out.
///
/// `TokenIter` wraps the `Lexer` and filters out any whitespace or comment
/// token kinds. This utility type makes it so that we don't need to worry about
/// either. When we need our next token, we simply call `next` on `TokenIter`.
pub struct TokenIter<'input> {
    lexer: Lexer<'input>,
}

impl<'input> TokenIter<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            lexer: Lexer::new(input),
        }
    }
}

impl<'input> Iterator for TokenIter<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next_token = self.lexer.next()?;
            if !matches!(next_token.kind(), kind![ws] | kind![comment]) {
                return Some(next_token);
            }
        }
    }
}

impl<'input> Parser<'input, TokenIter<'input>> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            tokens: TokenIter::new(input).peekable(),
        }
    }
}
