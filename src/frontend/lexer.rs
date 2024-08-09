#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Complex
    Identifier,
    BinaryOperator,
    EOF,

    // Symbols
    Equals,
    OpenParen,
    ClosedParen,
    Colon,
    SemiColon,
    DoubleQuote,
    SingleQuote,

    // Comparison operators
    Equality,
    Inequality,

    // Unary operators
    Increment,
    Decrement,
    Not,

    // Variable assignment
    Let,
    Const,

    // Variable literal types
    Number,
    Character,
    String,
    Boolean,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    fn create(token_type: TokenType, value: String) -> Self {
        Token { token_type, value }
    }
}

pub const KEYWORDS: [&'static str; 5] = ["let", "const", "bool", "str", "char"];

fn create_reserved(keyword: &str) -> Result<Token, String> {
    match keyword {
        "let" => Ok(Token::create(TokenType::Let, String::from(keyword))),
        "const" => Ok(Token::create(TokenType::Const, String::from(keyword))),
        "bool" => Ok(Token::create(TokenType::Boolean, String::from(keyword))),
        "str" => Ok(Token::create(TokenType::String, String::from(keyword))),
        "char" => Ok(Token::create(TokenType::Character, String::from(keyword))),
        // "int" => Ok(Token::create(TokenType::Integer, String::from(keyword))),
        // "float" => Ok(Token::create(TokenType::Float, String::from(keyword))),
        _ => return Err(format!("Unknown keyword: {}", keyword).to_string()),
    }
}

fn is_integer(val: &char) -> bool {
    match val {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => true,
        _ => false,
    }
}

fn is_skipabble(val: &char) -> bool {
    match val {
        ' ' | '\n' | '\t' | '\r' => true,
        _ => false,
    }
}

pub fn tokenise(source_code: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::<Token>::new();

    let mut src_chars: Vec<char> = source_code.chars().collect();

    while src_chars.len() > 0 {
        match src_chars[0] {
            '(' => tokens.push(Token::create(
                TokenType::OpenParen,
                src_chars.remove(0).to_string(),
            )),
            ')' => tokens.push(Token::create(
                TokenType::ClosedParen,
                src_chars.remove(0).to_string(),
            )),
            ';' => tokens.push(Token::create(
                TokenType::SemiColon,
                src_chars.remove(0).to_string(),
            )),
            ':' => tokens.push(Token::create(
                TokenType::Colon,
                src_chars.remove(0).to_string(),
            )),
            '"' => tokens.push(Token::create(
                TokenType::DoubleQuote,
                src_chars.remove(0).to_string(),
            )),
            '\'' => tokens.push(Token::create(
                TokenType::SingleQuote,
                src_chars.remove(0).to_string(),
            )),
            '!' => {
                if src_chars[1] == '=' && src_chars[2] == '=' {
                    for _ in 1..=3 {
                        src_chars.remove(0);
                    }
                    tokens.push(Token::create(TokenType::Inequality, "!==".to_string()))
                }
                tokens.push(Token::create(
                    TokenType::Not,
                    src_chars.remove(0).to_string(),
                ))
            }
            '=' => {
                if src_chars[1] == '=' && src_chars[2] == '=' {
                    for _ in 1..=3 {
                        src_chars.remove(0);
                    }
                    tokens.push(Token::create(TokenType::Equality, "===".to_string()))
                }
                tokens.push(Token::create(
                    TokenType::Equals,
                    src_chars.remove(0).to_string(),
                ))
            }
            '+' | '-' => match src_chars[1] {
                '+' => {
                    for _ in 1..=2 {
                        src_chars.remove(0);
                    }
                    tokens.push(Token::create(TokenType::Increment, "++".to_string()))
                }
                '-' => {
                    for _ in 1..=2 {
                        src_chars.remove(0);
                    }
                    tokens.push(Token::create(TokenType::Decrement, "--".to_string()))
                }
                _ => tokens.push(Token::create(
                    TokenType::BinaryOperator,
                    src_chars.remove(0).to_string(),
                )),
            },
            '*' | '/' | '%' => tokens.push(Token::create(
                TokenType::BinaryOperator,
                src_chars.remove(0).to_string(),
            )),
            _ => {
                // multi-character tokens. E.g. literals

                if is_integer(&src_chars[0]) {
                    let mut buff = String::from("");

                    let mut allow_dot = true;
                    while src_chars.len() > 0 && (is_integer(&src_chars[0]) || src_chars[0] == '.')
                    {
                        if src_chars[0] == '.' && allow_dot == false {
                            return Err("Unexpected \".\" found in number".to_string());
                        } else if src_chars[0] == '.' {
                            allow_dot = false;
                        }
                        buff.push(src_chars.remove(0));
                    }

                    tokens.push(Token::create(TokenType::Number, buff));
                    continue;
                }

                if src_chars[0].is_ascii_alphabetic() {
                    let mut buff = String::from("");
                    while src_chars.len() > 0
                        && (src_chars[0].is_ascii_alphabetic() || is_integer(&src_chars[0]))
                    {
                        buff.push(src_chars.remove(0))
                    }

                    if KEYWORDS.contains(&buff.as_str()) {
                        match create_reserved(buff.as_str()) {
                            Ok(token) => tokens.push(token),
                            Err(m) => return Err(m),
                        }
                        continue;
                    }

                    tokens.push(Token::create(TokenType::Identifier, buff));
                    continue;
                }

                if is_skipabble(&src_chars[0]) {
                    src_chars.remove(0);
                    continue;
                }

                return Err(format!(
                    "Unknown character detected in tokeniser in source: {}",
                    src_chars[0]
                ));
            }
        };
    }

    tokens.push(Token::create(TokenType::EOF, "EndOfFile".to_string()));

    Ok(tokens)
}
