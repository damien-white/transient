//! Lexer rules that define valid or accepted syntax.

use crate::kind;
use crate::lexer::Kind;

pub(crate) struct Rule {
    pub kind: Kind,
    pub matches: fn(&str) -> Option<usize>,
}

fn match_single_char(input: &str, c: char) -> Option<usize> {
    input
        .chars()
        .next()
        .and_then(|ch| if ch == c { Some(1) } else { None })
}

fn match_two_chars(input: &str, first: char, second: char) -> Option<usize> {
    if input.len() >= 2 {
        match_single_char(input, first).and_then(|val1| {
            println!("val1: {val1}");
            match_single_char(&input[1..], second).map(|val2| {
                println!("val2: {val2}");
                2
            })
        })
    } else {
        None
    }
}

fn match_keyword(input: &str, keyword: &str) -> Option<usize> {
    input.starts_with(keyword).then(|| keyword.len())
}

pub(crate) fn define_rules() -> Vec<Rule> {
    vec![
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
        Rule {
            kind: kind![==],
            matches: |input| match_two_chars(input, '=', '='),
        },
        Rule {
            kind: kind![!=],
            matches: |input| match_two_chars(input, '!', '='),
        },
        Rule {
            kind: kind![&&],
            matches: |input| match_two_chars(input, '&', '&'),
        },
        Rule {
            kind: kind![||],
            matches: |input| match_two_chars(input, '|', '|'),
        },
        Rule {
            kind: kind![<=],
            matches: |input| match_two_chars(input, '<', '='),
        },
        Rule {
            kind: kind![>=],
            matches: |input| match_two_chars(input, '>', '='),
        },
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
        '/' => kind![/],
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
