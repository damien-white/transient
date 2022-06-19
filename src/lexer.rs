// Macros
pub mod macros;

mod token;
pub use token::{Kind, Slice, Span, Token};

#[derive(Clone, Debug, Default)]
pub struct Lexer;

impl Lexer {
    pub fn new() -> Self {
        Self {}
    }
}
