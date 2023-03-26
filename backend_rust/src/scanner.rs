use crate::{token::Token, token_type::TokenType};

pub struct Scanner {
    code_str: String,
    token_vec: Vec<Token>,
}
impl Scanner {
    pub fn new(code_str: String) -> Scanner {
        let mut scanner = Scanner {
            code_str: code_str.replace(" ", ""),
            token_vec: Vec::new(),
        };
        scanner.process_code_str();
        scanner
    }
    pub fn get_token_vec(&self) -> &Vec<Token> {
        &self.token_vec
    }

    fn is_at_end(&self, current: usize) -> bool {
        current >= self.code_str.len()
    }

    fn add_token(&mut self, kind: TokenType, lexeme: String) {
        self.token_vec.push(Token::new(kind, lexeme));
    }

    fn scan_token(&mut self, ptr_start: &mut usize, ptr_current: &mut usize) {
        let mut start = *ptr_start;
        let mut current = *ptr_current;
        let mut c = String::new();
        let mut ch = self.code_str.chars().nth(start).unwrap();
        let mut is_float = false;
        if ch == '(' || ch == ')' || ch == '+' || ch == '-' || ch == '*' || ch == '/' || ch == '^' {
            c.push(ch);
            current += 1;
            start = current;
        } else {
            while ch.is_digit(10) {
                c.push(ch);
                current += 1;
                if current >= self.code_str.len() {
                    break;
                }
                ch = self.code_str.chars().nth(current).unwrap();
            }
            if c.starts_with('0') && c.len() > 1 {
                panic!("integer number cannot start with 0");
            }
            if ch == '.' {
                c.push(ch);
                current += 1;
                if current >= self.code_str.len() {
                    panic!("float number error");
                }
                ch = self.code_str.chars().nth(current).unwrap();
                while ch.is_digit(10) {
                    c.push(ch);
                    current += 1;
                    if current >= self.code_str.len() {
                        break;
                    }
                    ch = self.code_str.chars().nth(current).unwrap();
                }
                if c.ends_with('0') {
                    panic!("float number cannot end with 0");
                }
                is_float = true;
                start = current;
            } else {
                start = current;
            }
        }

        match c.as_str() {
            "(" => self.add_token(TokenType::LEFTPAREN, c),
            ")" => self.add_token(TokenType::RIGHTPAREN, c),
            "+" => self.add_token(TokenType::PLUS, c),
            "-" => self.add_token(TokenType::MINUS, c),
            "*" => self.add_token(TokenType::STAR, c),
            "/" => self.add_token(TokenType::SLASH, c),
            "^" => self.add_token(TokenType::CARET, c),
            _ => {
                if is_float {
                    self.add_token(TokenType::FLOAT, c);
                } else {
                    self.add_token(TokenType::INTEGER, c);
                }
            }
        }
        *ptr_start = start;
        *ptr_current = current;
    }

    pub fn process_code_str(&mut self) {
        let mut start = 0;
        let mut current = 0;
        while !self.is_at_end(current) {
            self.scan_token(&mut start, &mut current);
        }
        self.add_token(TokenType::EOF, "".to_string());
    }
}
