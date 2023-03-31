use std::io::Write;

mod actuator;
mod ast;
mod ast_builder;
mod scanner;
mod token;
mod token_type;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let mut scanner = scanner::Scanner::new();
    let mut astbuilder = ast_builder::ASTBuilder::new();
    let mut actuator = actuator::Actuator::new();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &mut scanner, &mut astbuilder, &mut actuator);
    }
}

fn handle_connection(
    mut stream: TcpStream,
    scanner: &mut scanner::Scanner,
    astbuilder: &mut ast_builder::ASTBuilder,
    actuator: &mut actuator::Actuator,
) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let mut input_str = String::new();
    let _response = request.split("input=").collect::<Vec<&str>>();
    if _response.len() > 1 {
        input_str = _response[1].split(" ").collect::<Vec<&str>>()[0].to_string();
    }
    println!("input_str: {}", input_str);
    if input_str == "" {
        let data = "{\"str\":\"\"}".to_owned();
        let response = format!(
            "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
            data.len(),
            data
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        return;
    }
    scanner.set_code_str(input_str.clone());
    scanner.process_code_str();

    astbuilder.set_token_vec(scanner.get_token_vec().to_vec());
    astbuilder.process_token_vec();
    actuator.set_root(astbuilder.get_root().unwrap());
    let (output_str, status) = actuator.actuate();

    let data = if status == "EXPRESSION" {
        "{\"str\":\"".to_string() + &output_str.to_string() + "\"}"
    } else {
        "{\"str\":\"\"}".to_owned()
    };

    let response = format!(
        "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
        data.len(),
        data
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
