use core::ops::Index;
use core::{
    fmt,
    ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
};

use crate::kind;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Kind {
    // Single characters
    Plus,
    Minus,
    Times,
    Solidus,
    Pow,
    Eq,
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
    // Multi-char
    String,
    Comment,
    Integer,
    Float,
    Ident,
    KeywordLet,
    KeywordFn,
    KeywordStruct,
    KeywordIf,
    KeywordElse,
    // Boolean Operators
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
                kind![+] => "+",
                kind![-] => "-",
                kind![*] => "*",
                kind![/] => "/",
                kind![^] => "^",
                kind![=] => "=",
                kind![.] => ".",
                kind![,] => ",",
                kind![_] => "_",
                kind![!] => "!",
                kind![&] => "&",
                kind![|] => "|",
                kind![:] => ":",
                kind![;] => ";",
                // Brackets
                kind![<] => "'<'",
                kind![>] => "'>'",
                kind!['['] => "]",
                kind![']'] => "]",
                kind!['('] => "(",
                kind![')'] => ")",
                kind!['{'] => "'{'",
                kind!['}'] => "'}'",
                // Multi-char
                kind![string] => "String",
                kind![comment] => "// Comment",
                kind![int] => "Int",
                kind![float] => "Float",
                kind![ident] => "Identifier",
                kind![let] => "let",
                kind![fn] => "fn",
                kind![struct] => "struct",
                kind![if] => "if",
                kind![else] => "else",
                // Operators
                kind![&&] => "&&",
                kind![||] => "||",
                kind![==] => "==",
                kind![!=] => "!=",
                kind![>=] => ">=",
                kind![<=] => "<=",
                // Miscellaneous
                kind![error] => "<?>",
                kind![ws] => "<WS>",
                kind![EOF] => "<EOF>",
            }
        )
    }
}

/// Slicing operations using ranges. Works similarly to [`Index`].
pub trait Slice<R> {
    fn slice(&self, range: R) -> Self;
}

impl<'a> Slice<Span> for &'a str {
    fn slice(&self, range: Span) -> Self {
        &self[range]
    }
}

impl<'a> Slice<Span> for &'a [u8] {
    fn slice(&self, range: Span) -> Self {
        &self[range]
    }
}

macro_rules! slice_range_impl {
    ( [$st:ident], $rt:ty ) => {
        impl<'a, $st> Slice<$rt> for &'a [$st] {
            fn slice(&self, range: $rt) -> Self {
                &self[range]
            }
        }
    };
    ( $st:ty, $rt:ty ) => {
        impl<'a> Slice<$rt> for &'a $st {
            fn slice(&self, range: $rt) -> Self {
                &self[range]
            }
        }
    };
}

macro_rules! slice_impl {
    ( [$st:ident] ) => {
        slice_range_impl! {[$st], Range<usize>}
        slice_range_impl! {[$st], RangeInclusive<usize>}
        slice_range_impl! {[$st], RangeTo<usize>}
        slice_range_impl! {[$st], RangeToInclusive<usize>}
        slice_range_impl! {[$st], RangeFrom<usize>}
        slice_range_impl! {[$st], RangeFull}
    };
    ($st:ty) => {
        slice_range_impl! {$st, Range<usize>}
        slice_range_impl! {$st, RangeInclusive<usize>}
        slice_range_impl! {$st, RangeTo<usize>}
        slice_range_impl! {$st, RangeToInclusive<usize>}
        slice_range_impl! {$st, RangeFrom<usize>}
        slice_range_impl! {$st, RangeFull}
    };
}

slice_impl! {str}
slice_impl! {[T]}

/// Two pointers representing the start and end of a slice.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Span {
    /// Start (inclusive)
    pub start: usize,
    /// End (exclusive)
    pub end: usize,
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        &self[Range::from(index)]
    }
}

impl Index<Span> for [u8] {
    type Output = [u8];

    fn index(&self, index: Span) -> &Self::Output {
        &self[Range::from(index)]
    }
}

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        span.start..span.end
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

/// Individual units produced by the lexer.
///
/// Each `Token` contains a token `Kind` and a `Span`. Given the input string,
/// the token can return the text it represents via the span.
#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Token {
    pub kind: Kind,
    pub span: Span,
}

impl Token {
    pub fn text<'input>(&self, input: &'input str) -> &'input str {
        &input[self.span]
    }

    pub fn len(&self) -> usize {
        self.span.end - self.span.start
    }

    pub const fn is_empty(&self) -> bool {
        self.span.start == self.span.end
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} - {{ {} .. {} }}>",
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
    use super::*;

    #[test]
    fn tokens_can_be_inspected() {
        let token1 = Token {
            kind: Kind::KeywordLet,
            span: Span { start: 0, end: 3 },
        };

        assert_eq!(token1.to_string(), r#"let"#);
    }
}
