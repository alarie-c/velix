use crate::lexer::{Lexer, Token};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Integer(i32),
    Float(f32),
    End,
}

#[derive(Debug)]
pub struct Parser<Iter: Iterator<Item = char>> {
    stack: Vec<Expr>,
    output: Vec<Expr>,
    lexer: Lexer<Iter>,
    parsing: bool,
}

impl<Iter: Iterator<Item = char>> Parser<Iter> {
    pub fn new(stream: Iter) -> Self {
        Self {
            stack: Vec::new(),
            output: Vec::new(),
            lexer: Lexer::new(stream),
            parsing: true,
        }
    }

    pub fn parse(&mut self) {
        while self.parsing {
            self.next_expr();
        }
    }

    fn next_expr(&mut self) {
        match self.lexer.next() {
            Token::LiteralNumeric(number) => match parse_number(number) {
                Some(expr) => self.output.push(expr),
                None => {}
            },
            Token::EndOfFile => {
                self.output.push(Expr::End);
                self.parsing = false;
            }
            _ => panic!("Unexpected token in parser!"),
        }
    }
}

/// Basic helper function to parse a `String` into an `Expr`
fn parse_number(number: String) -> Option<Expr> {
    let number = number.replace("_", "");

    // Parse float
    if number.contains(".") {
        return match number.parse::<f32>() {
            Ok(n) => Some(Expr::Float(n)),
            Err(_) => None,
        };
    // Parse integer
    } else {
        return match number.parse::<i32>() {
            Ok(n) => Some(Expr::Integer(n)),
            Err(_) => None,
        };
    }
}
