use crate::{token::Token, token_type::TokenType};

pub struct Scanner {
    code_str: String,
    token_vec: Vec<Token>,
}
impl Scanner {
    pub fn new(code_str: String) -> Scanner {
        let mut scanner = Scanner {
            code_str,
            token_vec: Vec::new(),
        };
        scanner.process_code_str();
        scanner
    }
    pub fn get_token_vec(&self) -> &Vec<Token> {
        &self.token_vec
    }
    fn is_at_end(&self, current: usize, scan_str: &str) -> bool {
        current >= scan_str.len()
    }

    fn add_token(&mut self, kind: TokenType, lexeme: String) {
        self.token_vec.push(Token::new(kind, lexeme));
    }

    fn scafn_str_without_space(&mut self, str: &str) {
        let mut start = 0;
        let mut current = 0;
        while !self.is_at_end(current, &str) {
            self.process_lexeme(&mut start, &mut current, &str);
        }
    }

    fn process_lexeme(&mut self, ptr_start: &mut usize, ptr_current: &mut usize, scan_str: &str) {
        let mut start = *ptr_start;
        let mut current = *ptr_current;
        let mut c = String::new();
        let mut ch = scan_str.chars().nth(start).unwrap();
        let mut is_float = false;
        let mut is_number = false;
        if ch == '('
            || ch == ')'
            || ch == '+'
            || ch == '-'
            || ch == '*'
            || ch == '/'
            || ch == '^'
            || ch == '='
            || ch == '{'
            || ch == '}'
        {
            c.push(ch);
            current += 1;
            start = current;
        } else if ch.is_digit(10) {
            is_number = true;
            while ch.is_digit(10) {
                c.push(ch);
                current += 1;
                if current >= scan_str.len() {
                    break;
                }
                ch = scan_str.chars().nth(current).unwrap();
            }
            if c.starts_with('0') && c.len() > 1 {
                panic!("integer number cannot start with 0");
            }
            if ch == '.' {
                c.push(ch);
                current += 1;
                if current >= scan_str.len() {
                    panic!("float number error");
                }
                ch = scan_str.chars().nth(current).unwrap();
                while ch.is_digit(10) {
                    c.push(ch);
                    current += 1;
                    if current >= scan_str.len() {
                        break;
                    }
                    ch = scan_str.chars().nth(current).unwrap();
                }
                if c.ends_with('0') {
                    panic!("float number cannot end with 0");
                }
                is_float = true;
                start = current;
            } else {
                start = current;
            }
        } else if ch.is_alphabetic() || ch == '_' {
            while ch.is_alphanumeric() || ch == '_' {
                c.push(ch);
                current += 1;
                if current >= scan_str.len() {
                    break;
                }
                ch = scan_str.chars().nth(current).unwrap();
            }
            start = current;
        } else {
            panic!("invalid character");
        }
        match c.as_str() {
            "(" => self.add_token(TokenType::LEFTPAREN, c),
            ")" => self.add_token(TokenType::RIGHTPAREN, c),
            "+" => self.add_token(TokenType::PLUS, c),
            "-" => self.add_token(TokenType::MINUS, c),
            "*" => self.add_token(TokenType::STAR, c),
            "/" => self.add_token(TokenType::SLASH, c),
            "^" => self.add_token(TokenType::CARET, c),
            "=" => self.add_token(TokenType::EQUAL, c),
            "{" => self.add_token(TokenType::LEFTBRACE, c),
            "}" => self.add_token(TokenType::RIGHTBRACE, c),
            _ => {
                if c.starts_with(|c: char| c.is_alphabetic() || c == '_') {
                    match c.as_str() {
                        "if" => self.add_token(TokenType::IF, c),
                        "else" => self.add_token(TokenType::ELSE, c),
                        "print" => self.add_token(TokenType::PRINT, c),
                        "return" => self.add_token(TokenType::RETURN, c),
                        "sin" => self.add_token(TokenType::SIN, c),
                        "cos" => self.add_token(TokenType::COS, c),
                        "tan" => self.add_token(TokenType::TAN, c),
                        "log" => self.add_token(TokenType::LOG, c),
                        "ln" => self.add_token(TokenType::LN, c),
                        _ => {
                            if is_number {
                                if is_float {
                                    self.add_token(TokenType::FLOAT, c);
                                } else {
                                    self.add_token(TokenType::INTEGER, c);
                                }
                            } else {
                                self.add_token(TokenType::STRING, c);
                            }
                        }
                    }
                } else {
                    if is_float {
                        self.add_token(TokenType::FLOAT, c);
                    } else {
                        self.add_token(TokenType::INTEGER, c);
                    }
                }
            }
        }
        *ptr_start = start;
        *ptr_current = current;
    }

    pub fn process_code_str(&mut self) {
        let code_str_vec: Vec<String> = self
            .code_str
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        for str in code_str_vec.iter() {
            self.scafn_str_without_space(str);
        }
        self.add_token(TokenType::EOF, "".to_string());
    }
}
