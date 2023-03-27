mod scanner;
mod token;
mod token_type;

fn main() {
    let str = String::from("if(1+2==3^1){_a1=p21+2}else{print(1)return (1/3)}");
    let scanner = scanner::Scanner::new(str);
    let tokens = scanner.get_token_vec();
    for token in tokens {
        token.print_token();
    }
}
