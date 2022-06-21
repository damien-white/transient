//! Lexer rules that define valid or accepted syntax.

use lazy_static::lazy_static;
use regex::Regex;

use super::token::Kind;
use crate::tk;

lazy_static! {
    static ref COMMENT_REGEX: Regex =
        Regex::new(r#"^//[^\n]*\n"#).expect("Comment regex must be valid.");
    static ref STRING_REGEX: Regex =
        Regex::new(r#"^"((\\"|\\\\)|[^\\"])*""#).expect("String regex must be valid.");
    static ref DOUBLE_REGEX: Regex =
        Regex::new(r#"^((\d+\.\d+)|(\.\d+))([eE][-+]?\d+)?"#).expect("Double regex must be valid.");
    static ref INTEGER_REGEX: Regex =
        Regex::new(r#"^(0|\d+)"#).expect("Integer regex must be valid.");
    static ref IDENTIFIER_REGEX: Regex =
        Regex::new(r#"^([a-zA-Z]|_)([a-zA-Z]|_|\d)*"#).expect("Identifier regex must be valid.");
}

/// Defines a single rule for the lexer.
pub(crate) struct Rule {
    pub kind: Kind,
    pub matches: fn(&str) -> Option<usize>,
}

/// Match against single-character token kinds, returning `None` on failure.
fn match_single_char(input: &str, c: char) -> Option<usize> {
    input
        .chars()
        .next()
        .and_then(|ch| if ch == c { Some(1) } else { None })
}

/// Match against multi-character token kinds, returning `None` on failure.
fn match_multi_char(input: &str, first: char, second: char) -> Option<usize> {
    if input.len() >= 2 {
        match_single_char(input, first)
            .and_then(|_| match_single_char(&input[1..], second).map(|_| 2))
    } else {
        None
    }
}

/// Match against keyword token kinds, returning `None` on failure.
fn match_keyword(input: &str, keyword: &str) -> Option<usize> {
    input.starts_with(keyword).then(|| keyword.len())
}

/// Match against regular expression, returning `None` on failure.
fn match_regex(input: &str, re: &Regex) -> Option<usize> {
    re.find(input).map(|m| m.end())
}

/// Provides the rules to the lexical scanner.
pub(crate) fn definitions() -> Vec<Rule> {
    vec![
        // Single characters
        Rule {
            kind: tk![!],
            matches: |input| match_single_char(input, '!'),
        },
        Rule {
            kind: tk![=],
            matches: |input| match_single_char(input, '='),
        },
        Rule {
            kind: tk![/],
            matches: |input| match_single_char(input, '/'),
        },
        Rule {
            kind: tk![_],
            matches: |input| match_single_char(input, '_'),
        },
        Rule {
            kind: tk![<],
            matches: |input| match_single_char(input, '<'),
        },
        Rule {
            kind: tk![>],
            matches: |input| match_single_char(input, '>'),
        },
        // Multiple characters
        Rule {
            kind: tk![==],
            matches: |input| match_multi_char(input, '=', '='),
        },
        Rule {
            kind: tk![!=],
            matches: |input| match_multi_char(input, '!', '='),
        },
        Rule {
            kind: tk![&&],
            matches: |input| match_multi_char(input, '&', '&'),
        },
        Rule {
            kind: tk![||],
            matches: |input| match_multi_char(input, '|', '|'),
        },
        Rule {
            kind: tk![<=],
            matches: |input| match_multi_char(input, '<', '='),
        },
        Rule {
            kind: tk![>=],
            matches: |input| match_multi_char(input, '>', '='),
        },
        // Keywords
        Rule {
            kind: tk![let],
            matches: |input| match_keyword(input, "let"),
        },
        Rule {
            kind: tk![fn],
            matches: |input| match_keyword(input, "fn"),
        },
        Rule {
            kind: tk![struct],
            matches: |input| match_keyword(input, "struct"),
        },
        Rule {
            kind: tk![if],
            matches: |input| match_keyword(input, "if"),
        },
        Rule {
            kind: tk![else],
            matches: |input| match_keyword(input, "else"),
        },
        // Patterns (regular expressions)
        Rule {
            kind: tk![string],
            matches: |input| match_regex(input, &STRING_REGEX),
        },
        Rule {
            kind: tk![comment],
            matches: |input| match_regex(input, &COMMENT_REGEX),
        },
        Rule {
            kind: tk![integer],
            matches: |input| match_regex(input, &INTEGER_REGEX),
        },
        Rule {
            kind: tk![double],
            matches: |input| match_regex(input, &DOUBLE_REGEX),
        },
        Rule {
            kind: tk![identifier],
            matches: |input| match_regex(input, &IDENTIFIER_REGEX),
        },
    ]
}

/// Matches a single, unambiguous character in the token stream.
///
/// Tokens that may only be a part of a larger token kind return `None`.
pub(crate) const fn unambiguous_single_char(c: char) -> Option<Kind> {
    Some(match c {
        '+' => tk![+],
        '-' => tk![-],
        '*' => tk![*],
        '^' => tk![^],
        '.' => tk![.],
        ',' => tk![,],
        ':' => tk![:],
        ';' => tk![;],
        '[' => tk!['['],
        ']' => tk![']'],
        '(' => tk!['('],
        ')' => tk![')'],
        '{' => tk!['{'],
        '}' => tk!['}'],
        _ => return None,
    })
}
