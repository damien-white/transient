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
