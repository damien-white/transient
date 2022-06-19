// Macros
pub mod macros;
mod rules;

mod token;

use rules::recognize_single_char;
pub use token::{Kind, Span, Token};

#[derive(Clone, Debug, Default)]
pub struct Lexer;

impl Lexer {
    pub fn new() -> Self {
        Self {}
    }

    /// Consumes the next token after checking its validity.
    fn valid_token(&self, input: &str) -> Option<Token> {
        let next = input.chars().next().unwrap();
        let (len, kind) = if let Some(kind) = recognize_single_char(next) {
            (1, kind)
        } else {
            return None;
        };

        Some(Token::new(
            kind,
            // TODO: Fix this Span
            Span { start: 0, end: len },
        ))
    }
}
