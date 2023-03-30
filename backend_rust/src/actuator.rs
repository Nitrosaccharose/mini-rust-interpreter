use std::{cell::RefCell, rc::Rc};

use crate::{ast::AST, token::Token, token_type::TokenType};

pub struct Actuator {
    //执行器拥有一个AST树的根节点
    root: Rc<RefCell<AST>>,
    variable_memory: std::collections::HashMap<String, f64>,
}
impl Actuator {
    pub fn new() -> Actuator {
        Actuator {
            root: Rc::new(RefCell::new(AST::new(Token::new(
                TokenType::EOF,
                String::new(),
            )))),
            variable_memory: std::collections::HashMap::new(),
        }
    }
    // pub fn get_root(&self) -> Rc<RefCell<AST>> {
    //     self.root.clone()
    // }
    pub fn set_root(&mut self, root: Rc<RefCell<AST>>) {
        self.root = root;
    }
    //对AST树进行分析并求值
    //多返回值，一个f64，一个状态码
    pub fn actuate(&mut self) -> (f64, String) {
        let mut is_assignment = false;
        let mut variable_name = String::new();
        let mut root = self.root.clone();
        //判断根结点的类型是否为等号，如果是，则是赋值语句，否则为普通的计算表达式
        if self.root.borrow().get_data().get_type_str() == "EQUAL" {
            is_assignment = true;
            variable_name = self
                .root
                .borrow()
                .get_left()
                .unwrap()
                .borrow()
                .get_data()
                .get_lexeme()
                .to_string();
            root = self.root.borrow().get_right().unwrap().clone();
        }
        //对root进行求值，不需要考虑赋值语句的情况，因为已经处理过了
        let result = self.calculate_expression(root);
        if is_assignment {
            self.variable_memory.insert(variable_name, result);
            return (result, "ASSIGNMENT".to_string());
        }
        (result, "EXPRESSION".to_string())
    }
    pub fn calculate_expression(&self, root: Rc<RefCell<AST>>) -> f64 {
        //如果是叶子节点，直接返回
        if root.borrow().is_leaf() {
            //返回前先判断是否是变量
            if root.borrow().get_data().get_type_str() == "STRING" {
                let variable_name = root.borrow().get_data().get_lexeme().to_string();
                let variable_value = self.variable_memory.get(&variable_name);
                if variable_value.is_none() {
                    panic!("Variable {} is not defined!", variable_name);
                }
                return *variable_value.unwrap();
            }
            return root
                .borrow()
                .get_data()
                .get_lexeme()
                .parse::<f64>()
                .unwrap();
        }
        match root.borrow().get_data().get_type_str().as_str() {
            "PLUS" => {
                let left = self.calculate_expression(root.borrow().get_left().unwrap().clone());
                let right = self.calculate_expression(root.borrow().get_right().unwrap().clone());
                left + right
            }
            "MINUS" => {
                let left = self.calculate_expression(root.borrow().get_left().unwrap().clone());
                let right = self.calculate_expression(root.borrow().get_right().unwrap().clone());
                left - right
            }
            "STAR" => {
                let left = self.calculate_expression(root.borrow().get_left().unwrap().clone());
                let right = self.calculate_expression(root.borrow().get_right().unwrap().clone());
                left * right
            }
            "SLASH" => {
                let left = self.calculate_expression(root.borrow().get_left().unwrap().clone());
                let right = self.calculate_expression(root.borrow().get_right().unwrap().clone());
                left / right
            }
            "CARET" => {
                let left = self.calculate_expression(root.borrow().get_left().unwrap().clone());
                let right = self.calculate_expression(root.borrow().get_right().unwrap().clone());
                left.powf(right)
            }
            "SIN" => {
                let right = self.calculate_expression(root.borrow().get_right().unwrap().clone());
                right.sin()
            }
            "COS" => {
                let right = self.calculate_expression(root.borrow().get_right().unwrap().clone());
                right.cos()
            }
            "TAN" => {
                let right = self.calculate_expression(root.borrow().get_right().unwrap().clone());
                right.tan()
            }
            "LN" => {
                let right = self.calculate_expression(root.borrow().get_right().unwrap().clone());
                right.ln()
            }
            _ => {
                panic!("未知的运算符");
            }
        }
    }
}
