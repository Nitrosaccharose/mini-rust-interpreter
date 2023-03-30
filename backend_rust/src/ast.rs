use std::{cell::RefCell, rc::Rc};

use crate::token::Token;
#[derive(Clone)]
pub struct AST {
    data: Token,
    left: Option<Rc<RefCell<AST>>>,
    right: Option<Rc<RefCell<AST>>>,
}
impl AST {
    pub fn new(data: Token) -> AST {
        AST {
            data,
            left: None,
            right: None,
        }
    }
    pub fn get_left(&self) -> Option<Rc<RefCell<AST>>> {
        self.left.clone()
    }
    pub fn get_right(&self) -> Option<Rc<RefCell<AST>>> {
        self.right.clone()
    }
    pub fn set_left(&mut self, left: Rc<RefCell<AST>>) {
        self.left = Some(left);
    }
    pub fn set_right(&mut self, right: Rc<RefCell<AST>>) {
        self.right = Some(right);
    }
    pub fn get_data(&self) -> Token {
        self.data.clone()
    }
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
    // pub fn print_ast_inor(&self) {
    //     // println!("被执行了");
    //     //中序遍历 AST
    //     //如果是叶子节点，直接打印
    //     if self.is_leaf() {
    //         print!("{} ", self.data.get_lexeme());
    //         return;
    //     }
    //     //递归地打印左子树
    //     if let Some(left) = self.left.clone() {
    //         left.borrow_mut().print_ast_inor();
    //     }
    //     //打印根节点
    //     print!("{} ", self.data.get_lexeme());
    //     //递归地打印右子树
    //     if let Some(right) = self.right.clone() {
    //         right.borrow_mut().print_ast_inor();
    //     }
    // }

    // pub fn print_ast_pre(&self) {
    //     //如果是叶子节点，直接打印
    //     if self.is_leaf() {
    //         print!("{} ", self.data.get_lexeme());
    //         return;
    //     }
    //     //打印根节点
    //     print!("{} ", self.data.get_lexeme());
    //     //递归地打印左子树
    //     if let Some(left) = self.left.clone() {
    //         left.borrow_mut().print_ast_pre();
    //     }
    //     //递归地打印右子树
    //     if let Some(right) = self.right.clone() {
    //         right.borrow_mut().print_ast_pre();
    //     }
    // }
}
