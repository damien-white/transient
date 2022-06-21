use transient::lexer::*;
use transient::parser::{ast, Parser};
use transient::tk;

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

fn expression_parser(input: &str) -> ast::Expr {
    let mut parser = Parser::new(input);
    parser.parse_expression()
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
            [tk![+], tk![-], tk!['('], tk![.], tk![')'], tk![:], tk![EOF],]
        );
    }

    #[test]
    fn unknown_or_unexpected_input() {
        let input = "{$$$$$$$+";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_tokens!(tokens, [tk!['{'], tk![error], tk![+], tk![EOF],]);
    }

    #[test]
    fn token_spans() {
        {
            let input = "+-(.):";
            let mut lexer = Lexer::new(input);
            let tokens = lexer.tokenize();
            let dot = tokens[3];
            assert_eq!(dot.kind(), tk![.]);
            assert_eq!(dot.span(), Span::new(3, 4));
        }
        {
            let input = "{$$$$$$$+";
            let mut lexer = Lexer::new(input);
            let tokens = lexer.tokenize();
            let error = tokens[1];
            assert_eq!(error.kind(), tk![error]);
            assert_eq!(error.span(), Span::new(1, 8));
        }
    }

    #[test]
    fn single_char_tokens_with_whitespace() {
        let input = "    +  - (.):  ";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let leading_whitespace = &tokens[0];
        assert_eq!(leading_whitespace.kind(), tk![ws]);
        assert_eq!(leading_whitespace.len(), 4);

        let between_plus_and_minus = &tokens[2];
        assert_eq!(between_plus_and_minus.kind(), tk![ws]);
        assert_eq!(between_plus_and_minus.len(), 2);

        let between_minus_and_left_paren = &tokens[4];
        assert_eq!(between_minus_and_left_paren.kind(), tk![ws]);
        assert_eq!(between_minus_and_left_paren.len(), 1);

        let trailing_whitespace = &tokens[9];
        assert_eq!(trailing_whitespace.kind(), tk![ws]);
        assert_eq!(trailing_whitespace.len(), 2);

        let tokens = tokens
            .into_iter()
            .filter(|t| t.kind() != tk![ws])
            .collect::<Vec<_>>();

        assert_tokens!(
            tokens,
            [tk![+], tk![-], tk!['('], tk![.], tk![')'], tk![:], tk![EOF],]
        );
    }

    #[test]
    fn ambiguous_multi_character_tokens() {
        let input = "&&=<=_!=||";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_tokens!(
            tokens,
            [tk![&&], tk![=], tk![<=], tk![_], tk![!=], tk![||], tk![EOF],]
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
                tk![let],
                tk![ws],
                tk![fn],
                tk![ws],
                tk![struct],
                tk![ws],
                tk![if],
                tk![ws],
                tk![else],
                tk![EOF],
            ]
        );

        let tokens = tokens
            .iter()
            .filter(|&t| t.kind() != tk![ws])
            .collect::<Vec<_>>();
        assert_tokens!(
            tokens,
            [tk![let], tk![fn], tk![struct], tk![if], tk![else], tk![EOF],]
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
            .filter(|t| t.kind() != tk![ws])
            .collect::<Vec<_>>();

        #[rustfmt::skip]
        assert_tokens!(
            tokens,
            [
                // comment line
                tk![comment],
                // function signature
                tk![fn], tk![identifier], tk!['('],
                    tk![identifier], tk![:], tk![identifier], tk![,],
                    tk![identifier], tk![:], tk![identifier],
                tk![')'], tk!['{'],
                    // function body
                    // `time` assignment
                    tk![let], tk![identifier], tk![=], tk![string], tk![+], tk![integer],
                        tk![/], tk![double], tk![^], tk![integer], tk![;],
                    // `iter` assignment
                    tk![let], tk![identifier], tk![=], tk![identifier], tk![.], tk![identifier],
                        tk!['('], tk![')'], tk![;],
                    // if let Some ... expr
                    tk![if], tk![let], tk![identifier], tk!['('], tk![identifier], tk![')'],
                        tk![=], tk![identifier], tk![.], tk![identifier], tk!['('], tk![')'],
                    tk!['{'],
                        // `time` reassignment
                        tk![identifier], tk![=], tk![identifier], tk![+], tk![identifier], tk![;],
                    // else if
                    tk!['}'], tk![else], tk![if], tk![!], tk![identifier], tk!['{'],
                        // `time` re-assignment
                        tk![identifier], tk![=], tk![identifier], tk![+], tk![string], tk![;],
                    tk!['}'], // end if
                tk!['}'], // end fn
            tk![EOF], // EOF
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
            .filter(|t| t.kind() != tk![ws])
            .collect();

        #[rustfmt::skip]
        assert_tokens!(
            tokens,
            [
                // struct definition with generic type
                tk![struct], tk![identifier], tk![<], tk![identifier], tk![>], tk!['{'],
                // struct field with type `Bar<T>`
                    tk![identifier], tk![:], tk![identifier], tk![<], tk![identifier], tk![>], tk![,],
                tk!['}'], // end struct
                tk![EOF],
            ]
        );
        let bar_property = tokens[6];
        assert_eq!(bar_property.span(), Span::new(21, 24));
        assert_eq!(bar_property.text(input), "bar");

        let foo_struct_def = tokens[1];
        assert_eq!(foo_struct_def.span(), Span::new(8, 11));
        assert_eq!(foo_struct_def.text(input), "Foo");
    }

    #[test]
    fn parse_expressions() {
        let expr = expression_parser("42");
        assert_eq!(expr, ast::Expr::Literal(ast::Literal::Integer(42)));
        let expr = expression_parser("  2.7768  ");
        assert_eq!(expr, ast::Expr::Literal(ast::Literal::Double(2.7768)));
        let expr = expression_parser("\"this_is_a_string\"");
        assert_eq!(
            expr,
            ast::Expr::Literal(ast::Literal::String("this_is_a_string".to_string()))
        );
        let expr = expression_parser(r#""this is 0123456789 also a string""#);
        assert_eq!(
            expr,
            ast::Expr::Literal(ast::Literal::String(
                "this is 0123456789 also a string".to_string()
            ))
        );
        let expr = expression_parser("BuildCommand");
        assert_eq!(expr, ast::Expr::Identifier("BuildCommand".to_string()));
        let expr = expression_parser("send  (  x, 2) ");
        assert_eq!(
            expr,
            ast::Expr::FunctionCall {
                name: "send".to_string(),
                args: vec![
                    ast::Expr::Identifier("x".to_string()),
                    ast::Expr::Literal(ast::Literal::Integer(2))
                ]
            }
        );
        let expr = expression_parser("!should_work");
        assert_eq!(
            expr,
            ast::Expr::PrefixOperator {
                op: tk![!],
                expr: Box::new(ast::Expr::Identifier("should_work".to_string()))
            }
        );
        // TODO: Add infix binop parsing; case passes (correctly) but cannot yet handle trailing `.toString()`
        let expr = expression_parser("-20.toString()");
        assert_eq!(
            expr,
            ast::Expr::PrefixOperator {
                op: tk![-],
                expr: Box::new(ast::Expr::Literal(ast::Literal::Integer(20)))
            }
        );
    }
}
