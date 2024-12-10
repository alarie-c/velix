use std::fs;

mod lexer;
mod parser;

const PATH: &str = "main.vx";

fn main() {
    let source = match fs::read_to_string(PATH) {
        Ok(source) => source,
        Err(e) => panic!("Error: {}", e),
    };

    let mut parser = parser::Parser::new(source.chars());
    parser.parse();
    dbg!(&parser);
}
