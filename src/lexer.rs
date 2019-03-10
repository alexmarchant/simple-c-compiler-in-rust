use regex::Regex;

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

impl Token {
    fn from_string(string: &str) -> Option<(Token, &str)> {
        if Regex::new(r"^\{").unwrap().is_match(string) {
            return Some((Token::OpenBrace, &string[1..]));
        }
        if Regex::new(r"^\}").unwrap().is_match(string) {
            return Some((Token::CloseBrace, &string[1..]));
        }
        if Regex::new(r"^\(").unwrap().is_match(string) {
            return Some((Token::OpenParen, &string[1..]));
        }
        if Regex::new(r"^\)").unwrap().is_match(string) {
            return Some((Token::CloseParen, &string[1..]));
        }
        if Regex::new(r"^;").unwrap().is_match(string) {
            return Some((Token::Semicolon, &string[1..]));
        }
        if Regex::new(r"^&&").unwrap().is_match(string) {
            return Some((Token::And, &string[2..]));
        }
        if Regex::new(r"^\|\|").unwrap().is_match(string) {
            return Some((Token::Or, &string[2..]));
        }
        if Regex::new(r"^==").unwrap().is_match(string) {
            return Some((Token::Equal, &string[2..]));
        }
        if Regex::new(r"^!=").unwrap().is_match(string) {
            return Some((Token::NotEqual, &string[2..]));
        }
        if Regex::new(r"^<=").unwrap().is_match(string) {
            return Some((Token::LessThanOrEqual, &string[2..]));
        }
        if Regex::new(r"^<").unwrap().is_match(string) {
            return Some((Token::LessThan, &string[1..]));
        }
        if Regex::new(r"^>=").unwrap().is_match(string) {
            return Some((Token::GreaterThanOrEqual, &string[2..]));
        }
        if Regex::new(r"^>").unwrap().is_match(string) {
            return Some((Token::GreaterThan, &string[1..]));
        }
        if Regex::new(r"^~").unwrap().is_match(string) {
            return Some((Token::BitwiseComplement, &string[1..]));
        }
        if Regex::new(r"^!").unwrap().is_match(string) {
            return Some((Token::LogicalNegation, &string[1..]));
        }
        if Regex::new(r"^-").unwrap().is_match(string) {
            return Some((Token::MinusSign, &string[1..]));
        }
        if Regex::new(r"^\+").unwrap().is_match(string) {
            return Some((Token::PlusSign, &string[1..]));
        }
        if Regex::new(r"^\*").unwrap().is_match(string) {
            return Some((Token::MultiplicationSign, &string[1..]));
        }
        if Regex::new(r"^/").unwrap().is_match(string) {
            return Some((Token::DivisionSign, &string[1..]));
        }
        if Regex::new(r"^int").unwrap().is_match(string) {
            return Some((Token::KeywordInt, &string[4..]));
        }
        if Regex::new(r"^return").unwrap().is_match(string) {
            return Some((Token::KeywordReturn, &string[7..]));
        }
        if let Some(found) = Regex::new(r"^\d+").unwrap().find(&string.to_string()) {
            let length = found.end() - found.start();
            match found.as_str().parse::<i64>() {
                Ok(i) => return Some((Token::IntegerLiteral(i), &string[length..])),
                Err(err) => panic!("Error parsing integer: {}", err),
            }
        }
        if let Some(found) = Regex::new(r"^\w+").unwrap().find(&string.to_string()) {
            let length = found.end() - found.start();
            let value = found.as_str().to_string();
            return Some((Token::Identifier(value), &string[length..]));
        }

        return None
    }
}

pub fn parse(contents: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut leftover_contents: &str = &contents.trim();

    while let Some((token, leftover_string)) = Token::from_string(leftover_contents) {
        leftover_contents = leftover_string.trim();
        tokens.push(token);
    }

    if leftover_contents.chars().count() > 0 {
        panic!("Error parsing program");
    }

    return tokens;
}
