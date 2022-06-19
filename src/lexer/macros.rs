/// Macro for referencing a token's `Kind`
#[macro_export]
macro_rules! kind {
    // Single characters
    [+] => {
        $crate::lexer::Kind::Plus
    };
    [-] => {
        $crate::lexer::Kind::Minus
    };
    [*] => {
        $crate::lexer::Kind::Times
    };
    [/] => {
        $crate::lexer::Kind::Solidus
    };
    [^] => {
        $crate::lexer::Kind::Pow
    };
    [=] => {
        $crate::lexer::Kind::Eq
    };
    [.] => {
        $crate::lexer::Kind::Dot
    };
    [,] => {
        $crate::lexer::Kind::Comma
    };
    [_] => {
        $crate::lexer::Kind::Underscore
    };
    [!] => {
        $crate::lexer::Kind::Bang
    };
    [&] => {
        $crate::lexer::Kind::Ampersand
    };
    [|] => {
        $crate::lexer::Kind::Pipe
    };
    [:] => {
        $crate::lexer::Kind::Colon
    };
    [;] => {
        $crate::lexer::Kind::SemiColon
    };
    // Brackets
    [<] => {
        $crate::lexer::Kind::LAngle
    };
    [>] => {
        $crate::lexer::Kind::RAngle
    };
    ['['] => {
        $crate::lexer::Kind::LSquare
    };
    [']'] => {
        $crate::lexer::Kind::RSquare
    };
    ['{'] => {
        $crate::lexer::Kind::LBrace
    };
    ['}'] => {
        $crate::lexer::Kind::RBrace
    };
    ['('] => {
        $crate::lexer::Kind::LParen
    };
    [')'] => {
        $crate::lexer::Kind::RParen
    };
    // Multi-char
    [string] => {
        $crate::lexer::Kind::String
    };
    [comment] => {
        $crate::lexer::Kind::Comment
    };
    [int] => {
        $crate::lexer::Kind::Integer
    };
    [float] => {
        $crate::lexer::Kind::Float
    };
    [ident] => {
        $crate::lexer::Kind::Ident
    };
    [let] => {
        $crate::lexer::Kind::KeywordLet
    };
    [fn] => {
        $crate::lexer::Kind::KeywordFn
    };
    [struct] => {
        $crate::lexer::Kind::KeywordStruct
    };
    [if] => {
        $crate::lexer::Kind::KeywordIf
    };
    [else] => {
        $crate::lexer::Kind::KeywordElse
    };
    // Operators
    [&&] => {
        $crate::lexer::Kind::And
    };
    [||] => {
        $crate::lexer::Kind::Or
    };
    [==] => {
        $crate::lexer::Kind::Eqq
    };
    [!=] => {
        $crate::lexer::Kind::Neq
    };
    [>=] => {
        $crate::lexer::Kind::Geq
    };
    [<=] => {
        $crate::lexer::Kind::Leq
    };
    // Miscellaneous
    [error] => {
        $crate::lexer::Kind::Error
    };
    [ws] => {
        $crate::lexer::Kind::Whitespace
    };
    [EOF] => {
        $crate::lexer::Kind::Eof
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn token_kind_displays() {
        assert_eq!(kind![+].to_string(), "+");
        assert_eq!(kind![<=].to_string(), "<=");
        assert_eq!(kind![let].to_string(), "let");
        assert_eq!(kind![error].to_string(), "<?>");
        assert_eq!(kind![comment].to_string(), "// Comment");
    }

    #[test]
    fn spans_can_be_indexed() {
        assert_eq!(kind![+].to_string(), "+");
        assert_eq!(kind![<=].to_string(), "<=");
        assert_eq!(kind![let].to_string(), "let");
        assert_eq!(kind![error].to_string(), "<?>");
        assert_eq!(kind![comment].to_string(), "// Comment");
    }
}
