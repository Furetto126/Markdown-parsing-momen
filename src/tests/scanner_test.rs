use crate::scanner::*;
use crate::token::*;

#[test]
fn correctly_identify_bold_text() {
    let input = "**eeee!!  -- +++ \n\n**".to_string();
    let expected_tokens = vec![TokenType::Bold, TokenType::Literal, TokenType::BoldClose];
    let parsed_input_tokens = Scanner::parse(input);

    assert_eq!(expected_tokens, parsed_input_tokens.get_token_types());
    assert_eq!(
        "eeee!!  -- +++ \n\n",
        parsed_input_tokens.tokens[1].get_chars_as_string()
    )
}
