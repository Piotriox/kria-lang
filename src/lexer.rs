#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Set,
    If,
    Else,
    ElseIf,
    While,
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
    Equal,
    EqualEqual,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    And,
    Or,
    Not,
    
    // Delimiters
    LParen,
    RParen,
    LBrace,
    RBrace,
    Newline,
    
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
    
    fn skip_spaces(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() && ch != '\n' {
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
        self.skip_spaces();
        
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
                        if self.peek_char() == Some('/') {
                            self.advance();
                            self.advance();
                            while let Some(ch) = self.current_char() {
                                if ch == '\n' {
                                    break;
                                }
                                self.advance();
                            }
                            self.next_token()
                        } else {
                            self.advance();
                            Token::Slash
                        }
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
                        if self.peek_char() == Some('=') {
                            self.advance();
                            self.advance();
                            Token::EqualEqual
                        } else {
                            self.advance();
                            Token::Equal
                        }
                    }
                    '!' => {
                        self.advance();
                        if self.current_char() == Some('=') {
                            self.advance();
                            Token::NotEqual
                        } else {
                            Token::Not
                        }
                    }
                    '>' => {
                        self.advance();
                        if self.current_char() == Some('=') {
                            self.advance();
                            Token::GreaterEqual
                        } else {
                            Token::Greater
                        }
                    }
                    '<' => {
                        self.advance();
                        if self.current_char() == Some('=') {
                            self.advance();
                            Token::LessEqual
                        } else {
                            Token::Less
                        }
                    }
                    '{' => {
                        self.advance();
                        Token::LBrace
                    }
                    '}' => {
                        self.advance();
                        Token::RBrace
                    }
                    '\n' => {
                        self.advance();
                        Token::Newline
                    }
                    '"' => Token::String(self.read_string()),
                    _ if ch.is_ascii_digit() => Token::Number(self.read_number()),
                    _ if ch.is_alphabetic() || ch == '_' => {
                        let ident = self.read_identifier();
                        match ident.as_str() {
                            "set" => Token::Set,
                            "if" => Token::If,
                            "else" => Token::Else,
                            "elseif" => Token::ElseIf,
                            "while" => Token::While,
                            "true" => Token::True,
                            "false" => Token::False,
                            "null" => Token::Null,
                            "print" => Token::Print,
                            "and" => Token::And,
                            "or" => Token::Or,
                            "not" => Token::Not,
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