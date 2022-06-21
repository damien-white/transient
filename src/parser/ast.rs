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
