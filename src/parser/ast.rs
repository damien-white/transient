use std::fmt;

use crate::lexer::Kind;

/// Abstract syntax tree based on expressions as a central language concept.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// Literal value, such as `Integer`, `Double` or `String`
    Literal(Literal),
    /// Identifier, storing its name.
    Identifier(String),
    /// Function call with its name and the arguments that were passed to it.
    FunctionCall { name: String, args: Vec<Expr> },
    /// Unary prefix operators
    PrefixOperator { op: Kind, expr: Box<Expr> },
    /// Binary operators
    InfixOperator {
        op: Kind,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    /// Unary postfix operators
    PostfixOperator { op: Kind, expr: Box<Expr> },
}

/// Literals are representations of the language's primitive types.
#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    /// `Integer` literal value
    Integer(usize),
    /// `Double` literal value
    Double(f64),
    /// `String` literal value
    String(String),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Literal(value) => {
                write!(f, "{value}")
            }
            Expr::Identifier(name) => {
                write!(f, "{name}")
            }
            Expr::FunctionCall { name, args } => {
                write!(f, "{name}(")?;
                for arg in args {
                    write!(f, "{arg},")?;
                }
                write!(f, ")")
            }
            Expr::PrefixOperator { op, expr } => {
                write!(f, "({op} {expr})")
            }
            Expr::InfixOperator { op, lhs, rhs } => {
                write!(f, "({lhs} {op} {rhs})")
            }
            Expr::PostfixOperator { op, expr } => {
                write!(f, "({expr} {op})")
            }
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Integer(integer) => {
                write!(f, "{}", integer)
            }
            Literal::Double(double) => {
                write!(f, "{}", double)
            }
            Literal::String(string) => {
                write!(f, r#""{}""#, string)
            }
        }
    }
}
