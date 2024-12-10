use std::fs;

mod lexer;
mod parser;

const PATH: &str = "main.vx";
pub const ECHO_STACK_OPS: bool = true;

pub fn eparser(msg: String) {
    if !ECHO_STACK_OPS {
        return;
    }
    println!("[PARSER] {}", msg);
}

pub fn elexer(msg: String) {
    if !ECHO_STACK_OPS {
        return;
    }
    println!("[LEXER] {}", msg);
}

fn main() {
    let source = match fs::read_to_string(PATH) {
        Ok(source) => source,
        Err(e) => panic!("Error: {}", e),
    };

    let mut parser = parser::Parser::new(source.chars());
    parser.parse();
    dbg!(&parser);
}
