use transient::kind;
use transient::lexer::*;

/// Walks `$tokens` and compares them to the given token kinds.
macro_rules! assert_tokens {
    ($tokens:ident, [$($kind:expr,)*]) => {
        {
            let mut tokens_iter = $tokens.iter();
            $(
                let token = tokens_iter.next().expect("should be more tokens in the input stream");
                assert_eq!(token.kind(), $kind);
            )*
        }
    };
}

#[test]
fn single_character_tokens() {
    let input = "+-(.):";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    assert_tokens!(
        tokens,
        [
            kind![+],
            kind![-],
            kind!['('],
            kind![.],
            kind![')'],
            kind![:],
            kind![EOF],
        ]
    );
}

#[test]
fn unknown_or_unexpected_input() {
    let input = "{$hello$+";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    assert_tokens!(tokens, [kind!['{'], kind![error], kind![+], kind![EOF],]);
}

#[test]
fn token_spans() {
    {
        let input = "+-(.):";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let dot = tokens[3];
        assert_eq!(dot.kind(), kind![.]);
        assert_eq!(dot.span(), Span::new(3, 4));
    }
    {
        let input = "{$hello$+";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let error = tokens[1];
        assert_eq!(error.kind(), kind![error]);
        assert_eq!(error.span(), Span::new(1, 8));
    }
}

#[test]
fn single_char_tokens_with_whitespace() {
    let input = "    +  - (.):  ";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    let leading_whitespace = &tokens[0];
    assert_eq!(leading_whitespace.kind(), kind![ws]);
    assert_eq!(leading_whitespace.len(), 4);

    let between_plus_and_minus = &tokens[2];
    assert_eq!(between_plus_and_minus.kind(), kind![ws]);
    assert_eq!(between_plus_and_minus.len(), 2);

    let between_minus_and_left_paren = &tokens[4];
    assert_eq!(between_minus_and_left_paren.kind(), kind![ws]);
    assert_eq!(between_minus_and_left_paren.len(), 1);

    let trailing_whitespace = &tokens[9];
    assert_eq!(trailing_whitespace.kind(), kind![ws]);
    assert_eq!(trailing_whitespace.len(), 2);

    let tokens = tokens
        .into_iter()
        .filter(|t| t.kind() != kind![ws])
        .collect::<Vec<_>>();

    assert_tokens!(
        tokens,
        [
            kind![+],
            kind![-],
            kind!['('],
            kind![.],
            kind![')'],
            kind![:],
            kind![EOF],
        ]
    )
}
