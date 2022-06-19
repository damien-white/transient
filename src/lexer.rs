// Macros
pub mod macros;
mod rules;

mod token;

use rules::single_character_token;
pub use token::{Kind, Span, Token};

use crate::kind;

#[derive(Clone, Debug, Default)]
pub struct Lexer;

impl Lexer {
    pub fn new() -> Self {
        Self {}
    }

    /// Consumes the next token after checking its validity.
    fn valid_token(&self, input: &str) -> Option<Token> {
        let next = input.chars().next().unwrap();
        let (len, kind) = if let Some(kind) = single_character_token(next) {
            (1, kind)
        } else {
            return None;
        };

        Some(Token::new(
            kind,
            // TODO: Fix this Span
            Span::new(0..len),
        ))
    }

    /// Attempts to consume the next token, emitting an error on failure.
    pub fn next_token(&self, input: &str) -> Token {
        self.valid_token(input)
            .unwrap_or_else(|| self.invalid_token(input))
    }

    /// Creates an error `Token` when the `next_token` method fails.
    fn invalid_token(&self, input: &str) -> Token {
        let len = input
            .char_indices()
            .find(|(pos, _)| self.valid_token(&input[*pos..]).is_some())
            .map(|(pos, _)| pos)
            .unwrap_or_else(|| input.len());

        debug_assert!(len <= input.len());
        Token::new(kind![error], Span::new(0..len))
    }

    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        let mut output = vec![];
        let mut suffix = input;

        while !suffix.is_empty() {
            let token = self.next_token(suffix);
            output.push(token);
            suffix = &suffix[token.len()..];
        }

        output.push(Token::new(kind![EOF], Span::new(input.len()..input.len())));
        output
    }
}
