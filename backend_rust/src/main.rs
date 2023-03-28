use std::{borrow::Borrow, io::Write};

mod actuator;
mod ast;
mod ast_builder;
mod scanner;
mod token;
mod token_type;
mod variable_memory;

fn main() {
    let mut str_r = String::new();
    let mut scanner = scanner::Scanner::new();
    let mut astbuilder = ast_builder::ASTBuilder::new();
    let mut actuator = actuator::Actuator::new();
    loop {
        print!(">");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut str_r).unwrap();
        scanner.set_code_str(str_r.clone());
        scanner.process_code_str();
        astbuilder.set_token_vec(scanner.get_token_vec().to_vec());
        astbuilder.process_token_vec();
        actuator.set_root(astbuilder.get_root().unwrap());
        let (result, status) = actuator.actuate();
        if status == "EXPRESSION" {
            println!(">{}", result);
        }
        str_r = "".to_owned();
    }
}
