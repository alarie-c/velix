use core::str;
use std::{collections::HashMap, iter::Peekable};

use crate::elexer;

mod lut {
    use std::collections::HashMap;

    /// Creates the lookup table for digits allowed in numeric literals
    /// -> `1234567890._`
    pub fn numeric_digits() -> HashMap<char, bool> {
        let mut lut = HashMap::<char, bool>::new();
        String::from("1234567890._").chars().for_each(|c| {
            lut.insert(c, true);
        });
        return lut;
    }

    /// Creates the lookup table for digits considered whitespace, which will be ignored
    /// -> `\t\r\n `
    pub fn whitespace_digits() -> HashMap<char, bool> {
        let mut lut = HashMap::<char, bool>::new();
        String::from("\t\r\n ").chars().for_each(|c| {
            println!("{}", c);
            lut.insert(c, true);
        });
        return lut;
    }

    pub fn operator_digits() -> HashMap<char, bool> {
        let mut lut = HashMap::<char, bool>::new();
        lut.insert('+', true);
        lut.insert('-', true);
        lut.insert('*', true);
        lut.insert('/', true);
        return lut;
    }
}

pub mod op {
    use std::collections::HashMap;

    /// Describes how and to what degree an operator
    /// affects the expressions already on the output
    ///
    /// Note: Operators like +, -, etc. are parsed into
    /// reverse polish notation, so they have an associativity of left
    #[derive(Debug, Clone, PartialEq)]
    pub enum Associativity {
        Left,
        Right,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Operator {
        lexeme: &'static str,
        assoc: Associativity,
        pub precedence: u8,
        pub n_args: u8,
    }

    pub fn operator_map() -> HashMap<&'static str, Operator> {
        let mut map = HashMap::<&'static str, Operator>::new();
        map.insert(
            "+",
            Operator {
                lexeme: "+",
                assoc: Associativity::Left,
                precedence: 1,
                n_args: 2,
            },
        );
        map.insert(
            "-",
            Operator {
                lexeme: "-",
                assoc: Associativity::Left,
                precedence: 1,
                n_args: 2,
            },
        );
        map.insert(
            "*",
            Operator {
                lexeme: "*",
                assoc: Associativity::Left,
                precedence: 2,
                n_args: 2,
            },
        );
        map.insert(
            "/",
            Operator {
                lexeme: "/",
                assoc: Associativity::Left,
                precedence: 1,
                n_args: 2,
            },
        );
        return map;
    }
}

#[derive(Debug)]
pub enum Token {
    LiteralNumeric(String),
    LiteralString(String),
    Operator(op::Operator),
    EndOfFile,
}

#[derive(Debug)]
pub struct Lexer<Iter: Iterator<Item = char>> {
    stream: Peekable<Iter>,
    lut_digits: HashMap<char, bool>,
    lut_whitespace: HashMap<char, bool>,
    lut_operators: HashMap<char, bool>,
    op_map: HashMap<&'static str, op::Operator>,
}

impl<Iter: Iterator<Item = char>> Lexer<Iter> {
    pub fn new(stream: Iter) -> Self {
        elexer(format!("Parser created"));
        Self {
            stream: stream.peekable(),
            lut_digits: lut::numeric_digits(),
            lut_whitespace: lut::whitespace_digits(),
            lut_operators: lut::operator_digits(),
            op_map: op::operator_map(),
        }
    }

    pub fn next(&mut self) -> Token {
        while let Some(c) = self.stream.next() {
            // Look for numeric digits
            if *self.lut_digits.get(&c).unwrap_or(&false) {
                // Consume until not numeric
                let mut buffer = String::from(c);
                while let Some(peek) = self.stream.peek() {
                    if *self.lut_digits.get(&peek).unwrap_or(&false) {
                        buffer.push(self.stream.next().unwrap()); // safe unwrap, we peek() above
                    } else {
                        break;
                    }
                }
                elexer(format!("Pushing Numeric :: {}", buffer));
                return Token::LiteralNumeric(buffer);

            // Look for operators if the current character is in the operator digits LUT
            } else if *self.lut_operators.get(&c).unwrap_or(&false) {
                // Check to see if the next thing is also an operator
                // Some ops are 2 digits lone, but are composed of digits found in lut_operatiors
                // lut_operators doesn't store multi-digit ops, that would be in the op map instead
                if let Some(peek) = self.stream.peek() {
                    match self.op_map.get(format!("{}{}", c, peek).as_str()) {
                        Some(new_op) => {
                            elexer(format!("Pushing Operator :: {:?}", &new_op));
                            return Token::Operator(new_op.clone()); // break here
                        }
                        None => {}
                    }
                    match self.op_map.get(str::from_utf8(&[c as u8]).unwrap()) {
                        Some(op) => {
                            elexer(format!("Pushing Operator :: {:?}", &op));
                            return Token::Operator(op.clone()); // break here
                        }
                        None => panic!(
                            "Character was found in the LUT but doesn't match an op in op_map"
                        ),
                    }
                }

            // Ignore whitespace and continue
            } else if *self.lut_whitespace.get(&c).unwrap_or(&false) {
                continue;

            // Anything else is a panic for now
            } else {
                panic!("Unexpected token!");
            }
        }

        // If the while loop ends, then .next() returns None,
        // thusly, we return an EOF token
        return Token::EndOfFile;
    }
}
