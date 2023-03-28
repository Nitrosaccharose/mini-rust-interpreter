use std::{cell::RefCell, rc::Rc};

use crate::{ast::AST, variable_memory::VariableMemory};

pub struct Actuator {
    //执行器拥有一个AST树的根节点
    root: Rc<RefCell<AST>>,
    variable_memory: std::collections::HashMap<String, f64>,
}
impl Actuator {
    pub fn new(root: Rc<RefCell<AST>>) -> Actuator {
        Actuator {
            root,
            variable_memory: std::collections::HashMap::new(),
        }
    }
    pub fn get_root(&self) -> Rc<RefCell<AST>> {
        self.root.clone()
    }
    pub fn set_root(&mut self, root: Rc<RefCell<AST>>) {
        self.root = root;
    }
    //对AST树进行分析并求值
}
