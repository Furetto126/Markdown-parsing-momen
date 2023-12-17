use crate::token::{Token, TokenType};

use lazy_static::lazy_static;
use std::{
    collections::{btree_map::Values, HashMap},
    io::LineWriter,
};

lazy_static! {
    static ref SPECIAL_STRING_CLOSER: HashMap<TokenType, String> = {
        let mut hash_map = HashMap::new();
        hash_map.insert(TokenType::Header1, "\n".to_string());
        hash_map.insert(TokenType::Header2, "\n".to_string());
        hash_map.insert(TokenType::Header3, "\n".to_string());
        hash_map.insert(TokenType::Italic, "*".to_string());
        hash_map.insert(TokenType::Bold, "**".to_string());
        hash_map.insert(TokenType::BoldItalic, "***".to_string());
        hash_map.insert(TokenType::Literal, "".to_string());
        hash_map.insert(TokenType::Null, "".to_string());
        hash_map
    };
    static ref SPECIAL_CLOSER: HashMap<TokenType, TokenType> = {
        let mut hash_map = HashMap::new();
        hash_map.insert(TokenType::Header1, TokenType::Header1Close);
        hash_map.insert(TokenType::Header2, TokenType::Header2Close);
        hash_map.insert(TokenType::Header3, TokenType::Header3Close);
        hash_map.insert(TokenType::Italic, TokenType::ItalicClose);
        hash_map.insert(TokenType::Bold, TokenType::BoldClose);
        hash_map.insert(TokenType::BoldItalic, TokenType::BoldItalicClose);
        hash_map.insert(TokenType::Literal, TokenType::Null);
        hash_map.insert(TokenType::Null, TokenType::Null);
        hash_map
    };
}

#[derive(Debug, Clone)]
pub struct Scanner {
    chars: Vec<char>,
    offset: usize,
    pub tokens: Vec<Token>,
}

impl Scanner {
    fn new(input: String) -> Self {
        Scanner {
            chars: input.chars().collect(),
            offset: 0,
            tokens: vec![],
        }
    }
}

impl Scanner {
    // Peeks one character ahead
    fn peek(&self, offset: Option<usize>) -> Option<char> {
        if !self.is_end() {
            Some(self.chars[self.offset + offset.unwrap_or(0)])
        } else {
            None
        }
    }

    // Peeks one characher ahead and consumes the value
    fn consume(&mut self) -> Option<char> {
        if !self.is_end() {
            let consumed_char = self.chars[self.offset];
            self.offset += 1;
            Some(consumed_char)
        } else {
            None
        }
    }

    /// Skip one character
    fn advance(&mut self) {
        self.offset += 1;
    }

    fn revert(&mut self) {
        self.offset -= 1;
    }

    fn is_last(&self) -> bool {
        self.offset >= self.chars.len() - 1
    }

    fn is_end(&self) -> bool {
        self.offset > self.chars.len() - 1
    }

    fn consume_literal(&mut self) {
        if let Some(previous_token) = self.clone().tokens.last().clone() {
            let mut literal_string = "".to_string();

            if SPECIAL_CLOSER
                .values()
                .any(|value| value.eq(&previous_token.token_type))
            {
                self.consume_literal_no_close();
                return;
            }

            let closing_token_string = SPECIAL_STRING_CLOSER
                .get(&previous_token.token_type)
                .unwrap()
                .clone();
            // If it's a normal token
            loop {
                if let Some(current) = self.consume() {
                    literal_string.push(current);
                    if literal_string.contains(&closing_token_string) {
                        literal_string = literal_string.replace(&closing_token_string, "");
                        break;
                    }
                } else {
                    // If there are no more characters left
                    break;
                }
            }

            let literal_token = Token::new(TokenType::Literal, literal_string);
            let expected_closing_token = SPECIAL_CLOSER
                .get(&previous_token.token_type)
                .unwrap()
                .clone();
            self.tokens.push(literal_token);
            self.tokens.push(Token::new(
                expected_closing_token,
                closing_token_string.clone(),
            )); // abcde * g *
        } else {
            self.consume_literal_no_close();
        }
    }

    /// Consumes the literal without expectinge a close token
    fn consume_literal_no_close(&mut self) {
        println!("consuming external literal");
        let mut literal_string = "".to_string();

        loop {
            if let Some(current) = self.consume().clone() {
                match current {
                    '#' => {
                        // if a token is found then push the literal and then consume the header
                        let literal_token = Token::new(TokenType::Literal, literal_string.clone());
                        self.tokens.push(literal_token);

                        self.revert();
                        self.consume_header();
                        return;
                    }
                    '*' => {
                        let literal_token = Token::new(TokenType::Literal, literal_string.clone());
                        self.tokens.push(literal_token);

                        self.revert();
                        self.consume_italic_or_bold();
                        return;
                    }
                    _ => literal_string.push(current),
                }
            } else {
                break;
            }
        }
        let literal_token = Token::new(TokenType::Literal, literal_string.clone());
        self.tokens.push(literal_token);

        println!("Literal consumed: {}", literal_string);
    }

    // # hello <- valid
    // #hello <- invalid
    fn consume_header(&mut self) {
        let mut token_string = "".to_string();
        for _ in 0..3 {
            if self.peek(None).unwrap() == '#' {
                token_string.push(self.consume().unwrap());
                continue;
            }
            break;
        }
        if self.peek(None).unwrap() == ' ' {
            token_string.push(self.consume().unwrap());
        }

        self.tokens.push(match token_string.as_str() {
            "# " => Token {
                token_type: TokenType::Header1,
                chars: token_string.chars().collect(),
            },
            "## " => Token {
                token_type: TokenType::Header2,
                chars: token_string.chars().collect(),
            },
            "### " => Token {
                token_type: TokenType::Header3,
                chars: token_string.chars().collect(),
            },
            _ => Token {
                token_type: TokenType::Literal,
                chars: token_string.chars().collect(),
            },
        });
    }

    fn consume_italic_or_bold(&mut self) {
        let mut token_string = "".to_string();
        for _ in 0..3 {
            if self.peek(None).unwrap() == '*' {
                token_string.push(self.consume().unwrap());
                continue;
            }
            break;
        }

        self.tokens.push(match token_string.as_str() {
            "*" => Token {
                token_type: TokenType::Italic,
                chars: token_string.chars().collect(),
            },
            "**" => Token {
                token_type: TokenType::Bold,
                chars: token_string.chars().collect(),
            },
            "***" => Token {
                token_type: TokenType::BoldItalic,
                chars: token_string.chars().collect(),
            },
            _ => Token {
                token_type: TokenType::Literal,
                chars: token_string.chars().collect(),
            },
        });
    }
}

/// Parse a string
/// Returns a `Scanner` (use .tokens to grab the tokens from the scanner)
pub fn parse(input: String) -> Scanner {
    let mut scanner = Scanner::new(input);

    loop {
        let current_char = match scanner.peek(None) {
            Some(c) => c,
            None => break,
        };

        match current_char {
            '#' => scanner.consume_header(),
            '*' => scanner.consume_italic_or_bold(),
            _ => scanner.consume_literal(),
        };
    }
    scanner //  not TO  REMOVE :3 uwu daddy drop database
}
