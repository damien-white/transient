use crate::lexer::Token;
use crate::parser::operator::Operator;
use crate::tk;

use super::ast;
use super::Parser;

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    #[inline]
    pub fn expression(&mut self) -> ast::Expr {
        self.parse_expression(0)
    }

    pub fn parse_expression(&mut self, binding_power: u8) -> ast::Expr {
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
                        let arg = self.parse_expression(0);
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
                let expr = self.parse_expression(0);
                self.skip(tk![')']);
                expr
            }

            op @ tk![+] | op @ tk![-] | op @ tk![!] => {
                self.skip(op);
                let ((), right_bp) = op.prefix_binding_power();
                let expr = self.parse_expression(right_bp);
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
                | op @ tk![&&]
                | op @ tk![||]
                | op @ tk![<]
                | op @ tk![<=]
                | op @ tk![>]
                | op @ tk![>=]
                | op @ tk![!] => op,
                tk![')'] | tk!['}'] | tk![,] | tk![;] => break,
                tk![EOF] => break,
                unknown => panic!("Unrecognized binary operator: `{unknown}`"),
            };

            if let Some((left_bp, ())) = op.postfix_binding_power() {
                if left_bp < binding_power {
                    // previous operator has higher binding power than new one
                    break;
                }

                self.skip(op);
                lhs = ast::Expr::PostfixOperator {
                    op,
                    expr: Box::new(lhs),
                };
                // Parsed an operator; so continue the loop.
                continue;
            }

            if let Some((left_bp, right_bp)) = op.infix_binding_power() {
                if left_bp < binding_power {
                    // previous operator has higher binding power than new one
                    break;
                }

                self.skip(op);
                let rhs = self.parse_expression(right_bp);
                lhs = ast::Expr::InfixOperator {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
                // Parsed an operator; so continue the loop.
                continue;
            }
            // Not an operator; stop parsing.
            break;
        }

        lhs
    }
}
