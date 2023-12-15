#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub chars: Vec<char>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Header1,
    Header2,
    Header3,
    Literal,
    Null
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::Null
    }
}

impl Default for Token {
    fn default() -> Self {
        Token { token_type: TokenType::default(), chars: vec![] }
    }
}

impl Token{
    pub fn from(token_string: String) -> Token {
        match token_string.as_str() {
            "# " => Token { token_type: TokenType::Header1, chars: token_string.chars().collect() },
            "## " => Token { token_type: TokenType::Header2, chars: token_string.chars().collect() },
            "###" => Token { token_type: TokenType::Header3, chars: token_string.chars().collect() },
            _ => Token { token_type: TokenType::Literal, chars:  token_string.chars().collect() },
        }
    }
}


// # ciao
// "juihuiu"

// Hearder Literal(ciao)
// OpenString Literal(jiuhuiu) CloseString

