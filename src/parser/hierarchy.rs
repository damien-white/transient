use crate::lexer::Token;
use crate::parser::{ast, Parser};
use crate::tk;

impl<'input, I> Parser<'input, I>
where
    I: Iterator<Item = Token>,
{
    pub fn statement(&mut self) -> ast::Stmt {
        match self.peek() {
            tk![let] => {
                self.consume(tk![let]);
                let identifier = self.next().expect("Expected identifier after `let`");
                assert_eq!(
                    identifier.kind(),
                    tk![identifier],
                    "Expected identifier after `let`, but found: `{}`",
                    identifier.kind()
                );

                let name = self.text(identifier).to_string();
                self.consume(tk![=]);
                let value = self.expression();
                self.consume(tk![;]);
                ast::Stmt::Let {
                    var: name,
                    value: Box::new(value),
                }
            }

            tk![identifier] => {
                let identifier = self.next().unwrap();
                let name = self.text(identifier).to_string();
                self.consume(tk![=]);
                let value = self.expression();
                self.consume(tk![;]);
                ast::Stmt::Assignment {
                    var: name,
                    value: Box::new(value),
                }
            }

            tk![if] => {
                self.consume(tk![if]);
                self.consume(tk!['(']);
                let cond = self.expression();
                self.consume(tk![')']);
                assert!(
                    self.at(tk!['{']),
                    "Expected a block after an `if` statement."
                );

                let block = match self.statement() {
                    ast::Stmt::Block { stmts } => stmts,
                    _ => unreachable!(),
                };

                let else_stmt = if self.at(tk![else]) {
                    self.consume(tk![else]);
                    assert!(
                        self.at(tk![if]) || self.at(tk!['{']),
                        "Expected a block or `if` statement after an `else` statement"
                    );
                    Some(Box::new(self.statement()))
                } else {
                    None
                };

                ast::Stmt::If {
                    cond: Box::new(cond),
                    body: block,
                    else_stmt,
                }
            }

            tk!['{'] => {
                self.consume(tk!['{']);
                let mut stmts = vec![];
                while !self.at(tk!['}']) {
                    stmts.push(self.statement());
                }
                self.consume(tk!['}']);
                ast::Stmt::Block { stmts }
            }

            kind => panic!("Unknown start of statement: `{kind}`"),
        }
    }
}
