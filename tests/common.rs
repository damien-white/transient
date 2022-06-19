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
    let lexer = Lexer::new();
    let input = "+-(.):";
    let tokens = lexer.tokenize(input);
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
    let lexer = Lexer::new();
    let input = "{$hello$+";
    let tokens = lexer.tokenize(input);
    assert_tokens!(tokens, [kind!['{'], kind![error], kind![+], kind![EOF],]);
}
