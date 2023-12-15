use crate::token::{Token, TokenType};

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref SPECIAL_CLOSER: HashMap<TokenType, String> = {
        let mut hash_map = HashMap::new();
        hash_map.insert(TokenType::Header1, "\n".to_string());
        hash_map.insert(TokenType::Header2, "\n".to_string());
        hash_map.insert(TokenType::Header3, "\n".to_string());
        hash_map.insert(TokenType::Literal, "".to_string());
        hash_map.insert(TokenType::Null, "".to_string());
        hash_map
    };
}

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

    fn is_end(&self) -> bool {
        self.offset >= self.chars.len() - 1
    }

    fn consume_literal(&mut self) {
        let mut literal_string = "".to_string();
        let previous_token = self.tokens.last().unwrap();
        let closing_token_string = *SPECIAL_CLOSER.get(&previous_token.token_type).unwrap().clone();
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
        
        self.tokens.push(Token::from(token_string));
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
            _ => todo!()
        };
    }
    scanner //  not TO  REMOVE :3 uwu daddy drop database
}
