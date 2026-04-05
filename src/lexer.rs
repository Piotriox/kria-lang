#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Set,
    True,
    False,
    Null,
    Print,
    
    // Identifiers and Literals
    Identifier(String),
    Number(i64),
    String(String),
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    
    // Delimiters
    LParen,
    RParen,
    Equal,
    Semicolon,
    
    // Special
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }
    
    fn current_char(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }
    
    fn peek_char(&self) -> Option<char> {
        if self.position + 1 < self.input.len() {
            Some(self.input[self.position + 1])
        } else {
            None
        }
    }
    
    fn advance(&mut self) {
        self.position += 1;
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn read_string(&mut self) -> String {
        self.advance(); // Skip opening quote
        let mut result = String::new();
        
        while let Some(ch) = self.current_char() {
            if ch == '"' {
                self.advance();
                break;
            } else if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.current_char() {
                    match escaped {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        _ => {
                            result.push('\\');
                            result.push(escaped);
                        }
                    }
                    self.advance();
                }
            } else {
                result.push(ch);
                self.advance();
            }
        }
        
        result
    }
    
    fn read_number(&mut self) -> i64 {
        let mut result = String::new();
        
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        result.parse::<i64>().unwrap_or(0)
    }
    
    fn read_identifier(&mut self) -> String {
        let mut result = String::new();
        
        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        
        result
    }
    
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        match self.current_char() {
            None => Token::Eof,
            Some(ch) => {
                match ch {
                    '+' => {
                        self.advance();
                        Token::Plus
                    }
                    '-' => {
                        self.advance();
                        Token::Minus
                    }
                    '*' => {
                        self.advance();
                        Token::Star
                    }
                    '/' => {
                        self.advance();
                        Token::Slash
                    }
                    '(' => {
                        self.advance();
                        Token::LParen
                    }
                    ')' => {
                        self.advance();
                        Token::RParen
                    }
                    '=' => {
                        self.advance();
                        Token::Equal
                    }
                    ';' => {
                        self.advance();
                        Token::Semicolon
                    }
                    '"' => Token::String(self.read_string()),
                    _ if ch.is_ascii_digit() => Token::Number(self.read_number()),
                    _ if ch.is_alphabetic() || ch == '_' => {
                        let ident = self.read_identifier();
                        match ident.as_str() {
                            "set" => Token::Set,
                            "true" => Token::True,
                            "false" => Token::False,
                            "null" => Token::Null,
                            "print" => Token::Print,
                            _ => Token::Identifier(ident),
                        }
                    }
                    _ => {
                        self.advance();
                        self.next_token()
                    }
                }
            }
        }
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token();
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        
        tokens
    }
}