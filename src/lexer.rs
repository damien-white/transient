//! This module contains the lexer, which tokenizes the input source.

pub use token::{Kind, Span, Token};

use crate::kind;
use crate::lexer::rules::{definitions, unambiguous_single_char, Rule};

pub mod macros;
mod rules;
mod token;

#[derive(Default)]
pub struct Lexer<'input> {
    input: &'input str,
    position: usize,
    eof: bool,
    rules: Vec<Rule>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            position: 0,
            eof: false,
            rules: definitions(),
        }
    }

    /// Iterates over input, collecting tokens into a `Vec`.
    pub fn tokenize(&mut self) -> Vec<Token> {
        self.collect()
    }

    /// Attempts to consume the next token, emitting an error on failure.
    pub fn next_token(&mut self, input: &str) -> Token {
        match self.validate(input) {
            Some(token) => token,
            None => self.handle_error(input),
        }
    }

    /// Validates the next token from the input source.
    fn validate(&mut self, input: &str) -> Option<Token> {
        let next = input.chars().next()?;

        /* Whitespace tokens */
        let (len, kind) = if next.is_whitespace() {
            (
                input
                    .char_indices()
                    .take_while(|(_, c)| c.is_whitespace())
                    .last()
                    .unwrap()
                    .0
                    + 1,
                kind![ws],
            )
            /* Unambiguous single-character tokens */
        } else if let Some(kind) = unambiguous_single_char(next) {
            (1, kind)
        } else {
            /* Single character (ambiguous), multi-character and keywords */
            self.rules
                .iter()
                .rev()
                .filter_map(|rule| Some(((rule.matches)(input)?, rule.kind)))
                .max_by_key(|&(len, _)| len)?
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
            let remaining = &self.input[self.position..];
            Some(self.next_token(remaining))
        }
    }
}
