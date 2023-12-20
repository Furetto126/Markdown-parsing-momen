#[cfg(test)]
mod tests {
    use crate::scanner::*;
    use crate::token::*;
    
    #[test]
    fn correctly_identify_bold_text() {
        let expected = vec!(TokenType::Bold, TokenType::Literal, TokenType::BoldClose);
        let input = "**eeee!!  -- +++ \n\n**".to_string();

        let mut result = Scanner::parse(input).tokens.

        assert_eq!();
    }
}