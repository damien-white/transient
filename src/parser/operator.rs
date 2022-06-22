use crate::lexer::Kind;
use crate::tk;

/// Trait for handling operator precedence and parsing expressions.
///
/// Source:
/// https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
pub trait Operator {
    /// Prefix operators bind their operand to the right
    fn prefix_binding_power(&self) -> ((), u8);

    fn infix_binding_power(&self) -> Option<(u8, u8)>;

    fn postfix_binding_power(&self) -> Option<(u8, ())>;
}

impl Operator for Kind {
    fn prefix_binding_power(&self) -> ((), u8) {
        match self {
            tk![+] | tk![-] | tk![!] => ((), 51),
            // Prefix operators are the only operators already seen when this is
            // called, so we know the token must be one of the above.
            _ => unreachable!("Token kind is not a prefix operator. {:?}", self),
        }
    }

    fn infix_binding_power(&self) -> Option<(u8, u8)> {
        let result = match self {
            tk![||] => (1, 2),
            tk![&&] => (3, 4),
            tk![==] | tk![!=] => (5, 6),
            tk![<] | tk![>] | tk![<=] | tk![>=] => (7, 8),
            tk![+] | tk![-] => (9, 10),
            tk![*] | tk![/] => (11, 12),
            tk![^] => (22, 21),
            _ => return None,
        };
        Some(result)
    }

    fn postfix_binding_power(&self) -> Option<(u8, ())> {
        let result = match self {
            tk![!] => (101, ()),
            _ => return None,
        };
        Some(result)
    }
}
