//! This module contains transient's core parsing logic.
use std::iter::Peekable;

use crate::lexer::{Kind, Lexer, Token};
use crate::tk;

pub mod ast;
mod expression;
mod operator;

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
            if !matches!(next_token.kind(), tk![ws] | tk![comment]) {
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

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    /// Gets the source text of the token by
    pub fn text(&self, token: Token) -> &'input str {
        token.text(self.input)
    }

    /// Attempts to look ahead to determine what the next token `Kind` is.
    pub(crate) fn peek(&mut self) -> Kind {
        self.tokens.peek().map(|t| t.kind()).unwrap_or(tk![EOF])
    }

    /// Checks whether the next token is a particular `Kind` of token.
    pub(crate) fn compare(&mut self, kind: Kind) -> bool {
        self.peek().eq(&kind)
    }

    /// Gets the next token from the lexer.
    pub(crate) fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    /// Skips a single token while verifying it is the expected token kind.
    pub(crate) fn skip(&mut self, expected: Kind) {
        let token = self
            .next()
            .expect(&*format!("Found `EOF`, but expected: `{}`", expected));

        assert_eq!(
            token.kind(),
            expected,
            "Found `{}`, but expected: `{}`",
            token.kind(),
            expected
        );
    }
}
