mod scanner;
mod token;
mod token_type;

fn main() {
    let str = String::from("(1.02+3*3.4^2)^(3-1.1)");
    let scanner = scanner::Scanner::new(str);
    let tokens = scanner.get_token_vec();
    for token in tokens {
        token.print_token();
    }
}
