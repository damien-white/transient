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

#[cfg(test)]
mod tests {
    use super::*;

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
        let input = "{$$$$$$$+";
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
            let input = "{$$$$$$$+";
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
        );
    }

    #[test]
    fn ambiguous_multi_character_tokens() {
        let input = "&&=<=_!=||";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_tokens!(
            tokens,
            [
                kind![&&],
                kind![=],
                kind![<=],
                kind![_],
                kind![!=],
                kind![||],
                kind![EOF],
            ]
        );
    }

    #[test]
    fn keyword_tokens() {
        let input = "let fn struct if else";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_tokens!(
            tokens,
            [
                kind![let],
                kind![ws],
                kind![fn],
                kind![ws],
                kind![struct],
                kind![ws],
                kind![if],
                kind![ws],
                kind![else],
                kind![EOF],
            ]
        );

        let tokens = tokens
            .iter()
            .filter(|&t| t.kind() != kind![ws])
            .collect::<Vec<_>>();
        assert_tokens!(
            tokens,
            [
                kind![let],
                kind![fn],
                kind![struct],
                kind![if],
                kind![else],
                kind![EOF],
            ]
        )
    }

    #[test]
    fn function_definition() {
        let input = r#"
// Testing a function
fn build_project(repo_name: String, strict: bool) {
    let time = "Date time: \" test" + 3 / 2.4e-2^5;
    let iter = time.chars();
    if let Some(c) = iter.next() {
        time = time + c;
    } else if !strict {
        time = time + ",";
    }
}
"#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer
            .tokenize()
            .into_iter()
            .filter(|t| t.kind() != kind![ws])
            .collect::<Vec<_>>();

        #[rustfmt::skip]
        assert_tokens!(
            tokens,
            [
                // comment line
                kind![comment],
                // function signature
                kind![fn], kind![ident], kind!['('],
                    kind![ident], kind![:], kind![ident], kind![,],
                    kind![ident], kind![:], kind![ident],
                kind![')'], kind!['{'],
                    // function body
                    // `time` assignment
                    kind![let], kind![ident], kind![=], kind![string], kind![+], kind![int],
                        kind![/], kind![float], kind![^], kind![int], kind![;],
                    // `iter` assignment
                    kind![let], kind![ident], kind![=], kind![ident], kind![.], kind![ident],
                        kind!['('], kind![')'], kind![;],
                    // if let Some ... expr
                    kind![if], kind![let], kind![ident], kind!['('], kind![ident], kind![')'],
                        kind![=], kind![ident], kind![.], kind![ident], kind!['('], kind![')'],
                    kind!['{'],
                        // `time` reassignment
                        kind![ident], kind![=], kind![ident], kind![+], kind![ident], kind![;],
                    // else if
                    kind!['}'], kind![else], kind![if], kind![!], kind![ident], kind!['{'],
                        // `time` re-assignment
                        kind![ident], kind![=], kind![ident], kind![+], kind![string], kind![;],
                    kind!['}'], // end if
                kind!['}'], // end fn
            kind![EOF], // EOF
            ]
        );
    }

    #[test]
    fn struct_definition() {
        let input = r#"
struct Foo<T> {
    bar: Bar<T>,
}
"#;
        let mut lexer = Lexer::new(input);
        let tokens: Vec<_> = lexer
            .tokenize()
            .into_iter()
            .filter(|t| t.kind() != kind![ws])
            .collect();
        assert_tokens!(
            tokens,
            [
                // struct definition with generic type
                kind![struct],
                kind![ident],
                kind![<],
                kind![ident],
                kind![>],
                kind!['{'],
                // struct field with type `Bar<T>`
                kind![ident],
                kind![:],
                kind![ident],
                kind![<],
                kind![ident],
                kind![>],
                kind![,],
                kind!['}'], // end struct
                kind![EOF],
            ]
        );
        let bar_property = tokens[6];
        assert_eq!(bar_property.span(), Span::new(21, 24));
        assert_eq!(bar_property.text(input), "bar");

        let foo_struct_def = tokens[1];
        assert_eq!(foo_struct_def.span(), Span::new(8, 11));
        assert_eq!(foo_struct_def.text(input), "Foo");
    }
}
