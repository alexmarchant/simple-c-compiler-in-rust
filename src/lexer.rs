#[derive(Debug, Clone)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Semicolon,
    KeywordInt,
    KeywordReturn,
    Identifier(String),
    IntegerLiteral(i64),
    BitwiseComplement,
    LogicalNegation,
    MinusSign,
    PlusSign,
    MultiplicationSign,
    DivisionSign,
    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

pub fn parse_tokens(contents: String) -> Vec<Token> {
    let token_strings: Vec<String> = parse_token_strings(contents);
    let mut tokens = Vec::new();
    for token_string in &token_strings {
        match token_string.as_ref() {
            "{" => tokens.push(Token::OpenBrace),
            "}" => tokens.push(Token::CloseBrace),
            "(" => tokens.push(Token::OpenParen),
            ")" => tokens.push(Token::CloseParen),
            "-" => tokens.push(Token::MinusSign),
            "+" => tokens.push(Token::PlusSign),
            "*" => tokens.push(Token::MultiplicationSign),
            "/" => tokens.push(Token::DivisionSign),
            "~" => tokens.push(Token::BitwiseComplement),
            "!" => tokens.push(Token::LogicalNegation),
            ";" => tokens.push(Token::Semicolon),
            "int" => tokens.push(Token::KeywordInt),
            "return" => tokens.push(Token::KeywordReturn),
            "&&" => tokens.push(Token::And),
            "||" => tokens.push(Token::Or),
            "==" => tokens.push(Token::Equal),
            "!=" => tokens.push(Token::NotEqual),
            "<" => tokens.push(Token::LessThan),
            "<=" => tokens.push(Token::LessThanOrEqual),
            ">" => tokens.push(Token::GreaterThan),
            ">=" => tokens.push(Token::GreaterThanOrEqual),
            _ => {
                match token_string.parse::<i64>() {
                    Ok(i) => tokens.push(Token::IntegerLiteral(i)),
                    Err(_) => tokens.push(Token::Identifier(token_string.clone())),
                }
            },
        }
    }
    return tokens;
}

fn parse_token_strings(contents: String) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    for char in contents.chars() { 
        match char {
            ' ' | '\n' | '\t' => {
                if current_token.len() > 0 {
                    tokens.push(current_token);
                    current_token = String::new();
                }
            },
            '{' | '}' | '(' | ')' | ';' | '-' | '!' | '~' => {
                if current_token.len() > 0 {
                    tokens.push(current_token);
                    current_token = String::new();
                }
                let mut token = String::new();
                token.push(char);
                tokens.push(token);
            },
            _ => current_token.push(char),
        }
    }
    return tokens;
}

