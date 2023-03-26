use crate::token_type::TokenType;

pub struct Token {
    pub kind: TokenType, // token的类型
    pub lexeme: String,  // token的文本
}
impl Token {
    pub fn new(kind: TokenType, lexeme: String) -> Token {
        Token { kind, lexeme }
    }
    pub fn get_type_str(&self) -> String {
        match self.kind {
            TokenType::LEFTPAREN => "LeftParen".to_string(),
            TokenType::RIGHTPAREN => "RightParen".to_string(),
            TokenType::PLUS => "PLUS".to_string(),
            TokenType::MINUS => "MINUS".to_string(),
            TokenType::STAR => "STAR".to_string(),
            TokenType::SLASH => "SLASH".to_string(),
            TokenType::CARET => "CARET".to_string(),
            TokenType::INTEGER => "INTEGER".to_string(),
            TokenType::FLOAT => "FLOAT".to_string(),
            TokenType::EOF => "EOF".to_string(),
        }
    }
    pub fn get_type_cn_str(&self) -> String {
        match self.kind {
            TokenType::LEFTPAREN => "左括号".to_string(),
            TokenType::RIGHTPAREN => "右括号".to_string(),
            TokenType::PLUS => "加号".to_string(),
            TokenType::MINUS => "减号".to_string(),
            TokenType::STAR => "乘号".to_string(),
            TokenType::SLASH => "除号".to_string(),
            TokenType::CARET => "乘方".to_string(),
            TokenType::INTEGER => "整数".to_string(),
            TokenType::FLOAT => "浮点数".to_string(),
            TokenType::EOF => "结束".to_string(),
        }
    }
    pub fn print_token(&self) {
        let mut type_cn_str = self.get_type_cn_str();
        if type_cn_str.len() < 4 {
            type_cn_str.push_str("\t");
        }
        println!(
            "|{:10}|{}\t|{:6}|",
            self.get_type_str(),
            type_cn_str,
            self.lexeme
        );
    }
}
