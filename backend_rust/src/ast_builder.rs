use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use crate::{ast::AST, token::Token, token_type::TokenType};
pub struct ASTBuilder<'a> {
    token_vec: Vec<Token>,
    root: Option<Rc<RefCell<AST>>>,
    priority_map: [(&'a str, i32); 18],
}

impl ASTBuilder<'_> {
    pub fn new() -> ASTBuilder<'static> {
        ASTBuilder {
            token_vec: Vec::new(),
            root: None,
            priority_map: [
                ("PLUS", 1),
                ("MINUS", 1),
                ("STAR", 2),
                ("SLASH", 2),
                ("CARET", 3),
                ("EQUAL", 4),
                ("INTEGER", 0),
                ("FLOAT", 0),
                ("STRING", 0),
                ("IF", 0),
                ("ELSE", 0),
                ("PRINT", 0),
                ("RETURN", 0),
                ("SIN", 0),
                ("COS", 0),
                ("TAN", 0),
                ("LOG", 0),
                ("LN", 0),
            ],
        }
    }
    pub fn set_token_vec(&mut self, token_vec: Vec<Token>) {
        self.token_vec = token_vec;
    }
    pub fn get_root(&self) -> Option<Rc<RefCell<AST>>> {
        self.root.clone()
    }
    pub fn is_higher_priority(&self, token1: &Token, token2: &Token) -> bool {
        let priority1 = self
            .priority_map
            .iter()
            .find(|(kind, _)| kind == &token1.get_type_str());
        let priority2 = self
            .priority_map
            .iter()
            .find(|(kind, _)| kind == &token2.get_type_str());

        priority1.unwrap().1 > priority2.unwrap().1
    }
    //将中缀表达式转换为后缀表达式
    pub fn infix_to_postfix(&self) -> Vec<Token> {
        //如果遇到整数、浮点数、字符串就直接放到结果Vector中
        //如果遇到左括号就将其压入栈中
        //若遇到右括号，将栈顶的运算符依次弹出并放入结果Vector中，直到遇到左括号为止，此时将这一对括号丢弃
        //若遇到运算符，如果该运算符的优先级大于栈顶运算符的优先级，就将该运算符压入栈中
        //否则，将栈顶运算符弹出并放入结果Vector中，继续与新的栈顶运算符相比较，直到该运算符优先级大于栈顶运算符的优先级为止，然后将该运算符压入栈中
        //重复上述过程，直到遇到EOF为止，将栈中所有运算符依次弹出并放入结果Vector中
        let mut result_vec: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::new();
        let mut is_assignment = false;
        //暂存单目运算符的栈
        let mut unary_stack: Vec<Token> = Vec::new();
        for token in &self.token_vec {
            match token.get_type_str().as_str() {
                "INTEGER" | "FLOAT" | "STRING" => {
                    result_vec.push(token.clone());
                }
                "LEFTPAREN" => {
                    stack.push(token.clone());
                }
                "RIGHTPAREN" => {
                    while stack.last().unwrap().get_type_str() != "LEFTPAREN" {
                        result_vec.push(stack.pop().unwrap());
                    }
                    stack.pop();
                    if !unary_stack.is_empty() {
                        result_vec.push(unary_stack.pop().unwrap());
                    }
                }
                "PLUS" | "MINUS" | "STAR" | "SLASH" | "CARET" => {
                    while !stack.is_empty()
                        && stack.last().unwrap().get_type_str() != "LEFTPAREN"
                        && !self.is_higher_priority(token, stack.last().unwrap())
                    {
                        result_vec.push(stack.pop().unwrap());
                    }
                    stack.push(token.clone());
                }
                //如果是这类单目运算符
                "SIN" | "COS" | "TAN" | "LOG" | "LN" => {
                    unary_stack.push(token.clone());
                }
                "EQUAL" => {
                    is_assignment = true;
                }
                _ => {}
            }
        }
        while !stack.is_empty() {
            result_vec.push(stack.pop().unwrap());
        }
        //压入等号
        if is_assignment {
            result_vec.push(Token::new(TokenType::EQUAL, "=".to_string()));
        }
        result_vec
    }

    pub fn process_token_vec(&mut self) {
        //现在处理好的token_vec是后缀表达式
        let postfix_vec = self.infix_to_postfix();
        let mut stack: Vec<Rc<RefCell<AST>>> = Vec::new();
        for token in postfix_vec {
            match token.get_type_str().as_str() {
                "INTEGER" | "FLOAT" | "STRING" => {
                    let ast = AST::new(token);
                    stack.push(Rc::new(RefCell::new(ast)));
                }
                "PLUS" | "MINUS" | "STAR" | "SLASH" | "CARET" | "EQUAL" => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    let mut ast = AST::new(token);
                    ast.borrow_mut().set_left(left);
                    ast.borrow_mut().set_right(right);
                    stack.push(Rc::new(RefCell::new(ast)));
                }
                "SIN" | "COS" | "TAN" | "LOG" | "LN" => {
                    let right = stack.pop().unwrap();
                    let mut ast = AST::new(token);
                    ast.borrow_mut().set_right(right);
                    stack.push(Rc::new(RefCell::new(ast)));
                }
                _ => {}
            }
        }
        self.root = stack.pop();
        self.token_vec.clear();
    }
}
