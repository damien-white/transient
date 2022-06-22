use crate::lexer::Token;
use crate::tk;

use super::ast;
use super::Parser;

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    pub fn parse_expression(&mut self) -> ast::Expr {
        let mut lhs = match self.peek() {
            literal @ tk![integer] | literal @ tk![double] | literal @ tk![string] => {
                let text = {
                    // The calls on `self` need to be split as `next` takes a `&mut self`.
                    let token = self.next().unwrap();
                    self.text(token)
                };

                let inner = match literal {
                    tk![integer] => ast::Literal::Integer(
                        text.parse()
                            .expect(&*format!("Invalid integer literal: `{text}`")),
                    ),
                    tk![double] => ast::Literal::Double(
                        text.parse()
                            .expect(&*format!("Invalid double literal: `{text}`")),
                    ),
                    tk![string] => ast::Literal::String(text[1..(text.len() - 1)].to_string()),
                    _ => unreachable!("This case should never be reached."),
                };

                ast::Expr::Literal(inner)
            }

            tk![identifier] => {
                let name = {
                    let token = self.next().unwrap();
                    self.text(token).to_string()
                };

                // Classify the identifier; identifier or function call
                if !self.compare(tk!['(']) {
                    // Identifier
                    ast::Expr::Identifier(name)
                } else {
                    // function call
                    let mut args = vec![];
                    self.skip(tk!['(']);
                    // function arguments
                    while !self.compare(tk![')']) {
                        let arg = self.parse_expression();
                        args.push(arg);
                        if self.compare(tk![,]) {
                            self.skip(tk![,]);
                        }
                    }

                    self.skip(tk![')']);
                    ast::Expr::FunctionCall { name, args }
                }
            }

            tk!['('] => {
                // Grouped expressions are parsed recursively
                self.skip(tk!['(']);
                let expr = self.parse_expression();
                self.skip(tk![')']);
                expr
            }

            op @ tk![+] | op @ tk![-] | op @ tk![!] => {
                self.skip(op);
                let expr = self.parse_expression();
                ast::Expr::PrefixOperator {
                    op,
                    expr: Box::new(expr),
                }
            }
            kind => panic!("Unknown start of expression: `{kind}`"),
        };

        loop {
            let op = match self.peek() {
                op @ tk![+]
                | op @ tk![-]
                | op @ tk![*]
                | op @ tk![/]
                | op @ tk![^]
                | op @ tk![==]
                | op @ tk![!=]
                | op @ tk![<]
                | op @ tk![<=]
                | op @ tk![>]
                | op @ tk![>=]
                | op @ tk![!] => op,
                tk![')'] | tk!['}'] | tk![,] | tk![;] => break,
                tk![EOF] => break,
                unknown => panic!("Unrecognized binary operator: `{unknown}`"),
            };

            self.skip(op);
            let rhs = self.parse_expression();
            lhs = ast::Expr::InfixOperator {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }
        }

        lhs
    }
}
