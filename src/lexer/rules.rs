//! Lexer rules that define valid or accepted syntax.

use lazy_static::lazy_static;
use regex::Regex;

use crate::kind;
use crate::lexer::Kind;

lazy_static! {
    static ref COMMENT_REGEX: Regex =
        Regex::new(r#"^//[^\n]*\n"#).expect("Comment regex must be valid.");
    static ref STRING_REGEX: Regex =
        Regex::new(r#"^"((\\"|\\\\)|[^\\"])*""#).expect("String regex must be valid.");
    static ref FLOAT_REGEX: Regex =
        Regex::new(r#"^((\d+\.\d+)|(\.\d+))([eE][-+]?\d+)?"#).expect("Float regex must be valid.");
    static ref INT_REGEX: Regex = Regex::new(r#"^(0|\d+)"#).expect("Float regex must be valid.");
    static ref IDENT_REGEX: Regex =
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
            kind: kind![!],
            matches: |input| match_single_char(input, '!'),
        },
        Rule {
            kind: kind![=],
            matches: |input| match_single_char(input, '='),
        },
        Rule {
            kind: kind![/],
            matches: |input| match_single_char(input, '/'),
        },
        Rule {
            kind: kind![_],
            matches: |input| match_single_char(input, '_'),
        },
        Rule {
            kind: kind![<],
            matches: |input| match_single_char(input, '<'),
        },
        Rule {
            kind: kind![>],
            matches: |input| match_single_char(input, '>'),
        },
        // Multiple characters
        Rule {
            kind: kind![==],
            matches: |input| match_multi_char(input, '=', '='),
        },
        Rule {
            kind: kind![!=],
            matches: |input| match_multi_char(input, '!', '='),
        },
        Rule {
            kind: kind![&&],
            matches: |input| match_multi_char(input, '&', '&'),
        },
        Rule {
            kind: kind![||],
            matches: |input| match_multi_char(input, '|', '|'),
        },
        Rule {
            kind: kind![<=],
            matches: |input| match_multi_char(input, '<', '='),
        },
        Rule {
            kind: kind![>=],
            matches: |input| match_multi_char(input, '>', '='),
        },
        // Keywords
        Rule {
            kind: kind![let],
            matches: |input| match_keyword(input, "let"),
        },
        Rule {
            kind: kind![fn],
            matches: |input| match_keyword(input, "fn"),
        },
        Rule {
            kind: kind![struct],
            matches: |input| match_keyword(input, "struct"),
        },
        Rule {
            kind: kind![if],
            matches: |input| match_keyword(input, "if"),
        },
        Rule {
            kind: kind![else],
            matches: |input| match_keyword(input, "else"),
        },
        // Patterns (regular expressions)
        Rule {
            kind: kind![string],
            matches: move |input| match_regex(input, &STRING_REGEX),
        },
        Rule {
            kind: kind![comment],
            matches: move |input| match_regex(input, &COMMENT_REGEX),
        },
        Rule {
            kind: kind![int],
            matches: |input| match_regex(input, &INT_REGEX),
        },
        Rule {
            kind: kind![float],
            matches: |input| match_regex(input, &FLOAT_REGEX),
        },
        Rule {
            kind: kind![ident],
            matches: |input| match_regex(input, &IDENT_REGEX),
        },
    ]
}

/// Matches a single, unambiguous character in the token stream.
///
/// Tokens that may only be a part of a larger token kind return `None`.
pub(crate) const fn unambiguous_single_char(c: char) -> Option<Kind> {
    Some(match c {
        '+' => kind![+],
        '-' => kind![-],
        '*' => kind![*],
        '^' => kind![^],
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
