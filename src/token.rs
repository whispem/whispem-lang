#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let, Print, If, Else, While, For, In, And, Or, Not, Fn, Return, Break, Continue,
    Length, Push, Pop, Reverse, Slice, Range, Input, ReadFile, WriteFile, Keys, Values, HasKey,
    True, False, Identifier(String), Number(f64), Str(String),
    Plus, Minus, Star, Slash, Percent,
    Equals, EqualEqual, Bang, BangEqual, Less, LessEqual, Greater, GreaterEqual,
    LParen, RParen, LeftBrace, RightBrace, LeftBracket, RightBracket, Comma, Colon,
    Newline, Eof,
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Token::Let=>"'let'".to_string(), Token::Print=>"'print'".to_string(),
            Token::If=>"'if'".to_string(), Token::Else=>"'else'".to_string(),
            Token::While=>"'while'".to_string(), Token::For=>"'for'".to_string(),
            Token::In=>"'in'".to_string(), Token::And=>"'and'".to_string(),
            Token::Or=>"'or'".to_string(), Token::Not=>"'not'".to_string(),
            Token::Fn=>"'fn'".to_string(), Token::Return=>"'return'".to_string(),
            Token::Break=>"'break'".to_string(), Token::Continue=>"'continue'".to_string(),
            Token::True=>"'true'".to_string(), Token::False=>"'false'".to_string(),
            Token::Identifier(n)=>format!("identifier '{}'",n), Token::Number(n)=>format!("number '{}'",n),
            Token::Str(s)=>format!("string \"{}\"",s), Token::Plus=>"'+'".to_string(),
            Token::Minus=>"'-'".to_string(), Token::Star=>"'*'".to_string(),
            Token::Slash=>"'/'".to_string(), Token::Percent=>"'%'".to_string(),
            Token::Equals=>"'='".to_string(), Token::EqualEqual=>"'=='".to_string(),
            Token::Bang=>"'!'".to_string(), Token::BangEqual=>"'!='".to_string(),
            Token::Less=>"'<'".to_string(), Token::LessEqual=>"'<='".to_string(),
            Token::Greater=>"'>'".to_string(), Token::GreaterEqual=>"'>='".to_string(),
            Token::LParen=>"'('".to_string(), Token::RParen=>"')'".to_string(),
            Token::LeftBrace=>"'{'".to_string(), Token::RightBrace=>"'}'".to_string(),
            Token::LeftBracket=>"'['".to_string(), Token::RightBracket=>"']'".to_string(),
            Token::Comma=>"','".to_string(), Token::Colon=>"':'".to_string(),
            Token::Newline=>"newline".to_string(), Token::Eof=>"end of file".to_string(),
            Token::Length=>"'length'".to_string(), Token::Push=>"'push'".to_string(),
            Token::Pop=>"'pop'".to_string(), Token::Reverse=>"'reverse'".to_string(),
            Token::Slice=>"'slice'".to_string(), Token::Range=>"'range'".to_string(),
            Token::Input=>"'input'".to_string(), Token::ReadFile=>"'read_file'".to_string(),
            Token::WriteFile=>"'write_file'".to_string(), Token::Keys=>"'keys'".to_string(),
            Token::Values=>"'values'".to_string(), Token::HasKey=>"'has_key'".to_string(),
        };
        write!(f, "{}", s)
    }
}
#[derive(Debug, Clone)]
pub struct Spanned { pub token: Token, pub line: usize, pub column: usize }