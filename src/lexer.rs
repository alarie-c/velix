use std::{collections::HashMap, iter::Peekable};

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
}

#[derive(Debug)]
pub enum Token {
    LiteralNumeric(String),
    LiteralString(String),
    EndOfFile,
}

#[derive(Debug)]
pub struct Lexer<Iter: Iterator<Item = char>> {
    stream: Peekable<Iter>,
    lut_digits: HashMap<char, bool>,
    lut_whitespace: HashMap<char, bool>,
}

impl<Iter: Iterator<Item = char>> Lexer<Iter> {
    pub fn new(stream: Iter) -> Self {
        Self {
            stream: stream.peekable(),
            lut_digits: lut::numeric_digits(),
            lut_whitespace: lut::whitespace_digits(),
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
                        // Safe unwrap, we peek() above
                        println!("Pusshing to buffer");
                        buffer.push(self.stream.next().unwrap());
                    } else {
                        break;
                    }
                }
                return Token::LiteralNumeric(buffer);

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
