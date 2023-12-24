#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub chars: Vec<char>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    Header1,
    Header1Close,
    Header2,
    Header2Close,
    Header3,
    Header3Close,
    Italic,
    ItalicClose,
    Bold,
    BoldClose,
    BoldItalic,
    BoldItalicClose,
    Literal,
    Null,
}

impl Default for TokenType {
    fn default() -> Self {
        TokenType::Null
    }
}

impl Default for Token {
    fn default() -> Self {
        Token {
            token_type: TokenType::default(),
            chars: vec![],
        }
    }
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Token {
        Token {
            token_type,
            chars: value.chars().collect(),
        }
    }

    pub fn get_chars_as_string(&self) -> String {
        self.chars.clone().into_iter().collect()
    }
}
