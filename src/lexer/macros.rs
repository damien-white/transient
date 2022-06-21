/// Macro for referencing a token's `Kind`
#[macro_export]
macro_rules! tk {
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
        $crate::lexer::Kind::Divide
    };
    [^] => {
        $crate::lexer::Kind::Power
    };
    [=] => {
        $crate::lexer::Kind::Equals
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
    // Multi-character
    [string] => {
        $crate::lexer::Kind::String
    };
    [comment] => {
        $crate::lexer::Kind::Comment
    };
    [integer] => {
        $crate::lexer::Kind::Integer
    };
    [double] => {
        $crate::lexer::Kind::Double
    };
    [identifier] => {
        $crate::lexer::Kind::Identifier
    };
    // Keywords
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
    // Whitespace
    [ws] => {
        $crate::lexer::Kind::Whitespace
    };
    // End of file
    [EOF] => {
        $crate::lexer::Kind::Eof
    };
    // Error
    [error] => {
        $crate::lexer::Kind::Error
    };
}
