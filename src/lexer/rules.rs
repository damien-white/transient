//! Lexer rules that define valid or accepted syntax.

use crate::kind;
use crate::lexer::Kind;

/// Matches a single, unambiguous character in the token stream.
///
/// Tokens that may only be a part of a larger token kind return `None`.
pub(crate) const fn single_character_token(c: char) -> Option<Kind> {
    Some(match c {
        '+' => kind![+],
        '-' => kind![-],
        '*' => kind![*],
        '/' => kind![/],
        '.' => kind![.],
        ',' => kind![,],
        ':' => kind![:],
        ';' => kind![;],
        '[' => kind!['['],
        ']' => kind![']'],
        '(' => kind!['('],
        ')' => kind![')'],
        '{' => kind!['{'],
        '}' => kind!['}'],
        _ => return None,
    })
}
