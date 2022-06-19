//! This module contains the core logic for the lexical scanner.

// Macros
pub mod macros;
mod rules;

mod token;

use rules::single_character_token;
pub use token::{Kind, Span, Token};

use crate::kind;

#[derive(Clone, Debug, Default)]
pub struct Lexer<'input> {
    input: &'input str,
    position: usize,
    eof: bool,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            ..Self::default()
        }
    }

    /// Iterates over input, collecting tokens into a `Vec`.
    pub fn tokenize(&mut self) -> Vec<Token> {
        self.collect()
    }

    /// Attempts to consume the next token, emitting an error on failure.
    pub fn next_token(&mut self, input: &str) -> Token {
        self.validate(input)
            .unwrap_or_else(|| self.handle_error(input))
    }

    /// Validates the next token from the input source.
    fn validate(&mut self, input: &str) -> Option<Token> {
        let next = input.chars().next().unwrap();
        let (len, kind) = if let Some(kind) = single_character_token(next) {
            (1, kind)
        } else {
            return None;
        };

        // Set the span indices
        let start = self.position;
        self.position += len;

        Some(Token::new(kind, Span::new(start, start + len)))
    }

    /// Creates an error `Token` when the `next_token` method fails.
    fn handle_error(&mut self, input: &str) -> Token {
        let start = self.position;

        let len = input
            .char_indices()
            .find(|(pos, _)| self.validate(&input[*pos..]).is_some())
            .map(|(pos, _)| pos)
            .unwrap_or_else(|| input.len());
        debug_assert!(len <= input.len());

        let len = len;
        self.position = start + len;
        Token::new(kind![error], Span::new(start, start + len))
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.input.len() {
            if self.eof {
                return None;
            }

            self.eof = true;

            Some(Token::new(
                kind![EOF],
                Span::new(self.position, self.position),
            ))
        } else {
            Some(self.next_token(&self.input[self.position..]))
        }
    }
}
