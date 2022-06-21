use std::fmt;
use std::ops::{Index, Range};

use crate::tk;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Kind {
    // Single characters
    Plus,
    Minus,
    Times,
    Divide,
    Power,
    Equals,
    Dot,
    Comma,
    Underscore,
    Bang,
    Ampersand,
    Pipe,
    Colon,
    SemiColon,
    // Brackets
    LAngle,
    RAngle,
    LSquare,
    RSquare,
    LBrace,
    RBrace,
    LParen,
    RParen,
    // Multi-character
    String,
    Comment,
    Integer,
    Double,
    Identifier,
    // Keywords
    KeywordLet,
    KeywordFn,
    KeywordStruct,
    KeywordIf,
    KeywordElse,
    // Operators
    And,
    Or,
    Eqq,
    Neq,
    Geq,
    Leq,
    // Whitespace
    Whitespace,
    // End of file
    Eof,
    // Error
    Error,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                // Single characters
                tk![+] => "+",
                tk![-] => "-",
                tk![*] => "*",
                tk![/] => "/",
                tk![^] => "^",
                tk![=] => "=",
                tk![.] => ".",
                tk![,] => ",",
                tk![_] => "_",
                tk![!] => "!",
                tk![&] => "&",
                tk![|] => "|",
                tk![:] => ":",
                tk![;] => ";",
                // Brackets
                tk![<] => "<",
                tk![>] => ">",
                tk!['['] => "]",
                tk![']'] => "]",
                tk!['('] => "(",
                tk![')'] => ")",
                tk!['{'] => "{",
                tk!['}'] => "}",
                // Multi-character
                tk![string] => "String",
                tk![comment] => "// Comment",
                tk![integer] => "Integer",
                tk![double] => "Double",
                tk![identifier] => "Identifier",
                // Keywords
                tk![let] => "let",
                tk![fn] => "fn",
                tk![struct] => "struct",
                tk![if] => "if",
                tk![else] => "else",
                // Operators
                tk![&&] => "&&",
                tk![||] => "||",
                tk![==] => "==",
                tk![!=] => "!=",
                tk![>=] => ">=",
                tk![<=] => "<=",
                // Whitespace
                tk![ws] => "<WS>",
                // End of file
                tk![EOF] => "<EOF>",
                // Error
                tk![error] => "<?>",
            }
        )
    }
}

/// A range type constructed from two indexing pointers.
///
/// This type is essentially a re-implementation of `std::ops::Range<usize>`.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Span {
    /// The lower bound of the range (inclusive).
    start: usize,
    /// The upper bound of the range (exclusive).
    end: usize,
}

impl Span {
    /// Constructs a new `Span` from a start and end offset.
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start <= end);
        Self { start, end }
    }

    /// Returns the start offset of the span.
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the end offset of the span.
    pub fn end(&self) -> usize {
        self.end
    }

    /// Returns the length of the span in bytes.
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns `true` if the span has a length of zero bytes.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Span {
            start: range.start,
            end: range.end,
        }
    }
}

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        Range {
            start: span.start,
            end: span.end,
        }
    }
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        &self[index.start..index.end]
    }
}

impl Index<Span> for [u8] {
    type Output = [u8];

    fn index(&self, index: Span) -> &Self::Output {
        &self[index.start..index.end]
    }
}

/// Individual units produced by the lexer.
///
/// Each `Token` contains a token `Kind` and a `Span`. Given the input string,
/// the token can return the text it represents via the span.
#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Token {
    /// The token kind, or variant, as defined on the `Kind` enum.
    kind: Kind,
    span: Span,
}

impl Token {
    /// Constructs a `Token` from a kind and its corresponding span.
    pub fn new(kind: Kind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Returns the `Kind` of the token.
    pub fn kind(&self) -> Kind {
        self.kind
    }

    /// Returns the `Span` of the token.
    pub fn span(&self) -> Span {
        self.span
    }

    /// Returns the token as text by indexing the input source with its span.
    pub fn text<'input>(&self, input: &'input str) -> &'input str {
        &input[self.span]
    }

    /// Returns the length of the token's span.
    pub fn len(&self) -> usize {
        self.span.len()
    }

    /// Returns true if the token's span comprises an empty range.
    pub fn is_empty(&self) -> bool {
        self.span.is_empty()
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} - {{ {} .. {} }}",
            self.kind, self.span.start, self.span.end
        )
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[cfg(test)]
mod tests {
    use crate::tk;

    use super::*;

    #[test]
    fn token_kind_displays() {
        assert_eq!(tk![+].to_string(), "+");
        assert_eq!(tk![<=].to_string(), "<=");
        assert_eq!(tk![let].to_string(), "let");
        assert_eq!(tk![error].to_string(), "<?>");
        assert_eq!(tk![comment].to_string(), "// Comment");
    }

    #[test]
    fn token_indexing_with_spans() {
        let token = Token {
            kind: Kind::KeywordLet,
            span: Span { start: 0, end: 3 },
        };

        assert_eq!(token.text("let x = 5;"), "let");
        assert_eq!(token.len(), 3);
    }
}
