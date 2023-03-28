mod actuator;
mod ast;
mod ast_builder;
mod scanner;
mod token;
mod token_type;
mod variable_memory;

fn main() {
    let str = String::from("a=(1+2)*3/4");
    let scanner = scanner::Scanner::new(str);
    let token_vec = scanner.get_token_vec();
    let mut astbuilder = ast_builder::ASTBuilder::new(token_vec.to_vec());

    let token_vec_pos = astbuilder.infix_to_postfix();
    for token in token_vec_pos {
        print!("{} ", token.get_lexeme());
    }

    astbuilder.process_token_vec();
    println!("\nAST inorder:");
    astbuilder.get_root().unwrap().borrow().print_ast_inor();
    println!("\nAST preorder:");
    astbuilder.get_root().unwrap().borrow().print_ast_pre();
}
