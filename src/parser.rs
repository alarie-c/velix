use crate::{
    eparser,
    lexer::{op, Lexer, Token},
};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Integer(i32),
    Float(f32),
    Operator(op::Operator),
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

    pub fn _step(&self) {
        println!("[PARSER] PARSER STEPPED\nSTACK:\n\t{:#?}\nOUTPUT:\n\t{:#?}\n\n", self.stack, self.output);
    }

    pub fn parse(&mut self) {
        while self.parsing {
            self.next_expr();
        }
    }

    fn next_expr(&mut self) {
        //self._step();
        match self.lexer.next() {
            Token::LiteralNumeric(number) => match parse_number(number) {
                Some(expr) => self.output.push(expr),
                None => {}
            },
            Token::Operator(operator) => {
                parse_operator(operator, &mut self.stack, &mut self.output)
            }
            Token::EndOfFile => {
                // Reverse and drain the stack to the output
                // This way, the highest precedence operators go on first
                self.stack.reverse();
                self.stack
                    .drain(0..)
                    .for_each(|expr| self.output.push(expr));

                // Push the EOF expr to the output
                self.output.push(Expr::End);

                // Quit parsing
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

fn parse_operator(operator: op::Operator, stack: &mut Vec<Expr>, output: &mut Vec<Expr>) {
    // Check to see if this operator is a closing parenthesis
    // If so, drain the stack to the output until an open par is found
    if operator.lexeme == ")" {
        'lookfor_par: while let Some(expr) = stack.pop() {
            match &expr {
                Expr::Operator(stack_op) => {
                    if stack_op.lexeme == "(" {
                        // Stop here
                        break 'lookfor_par;
                    }
                }
                _ => {},
            }
            output.push(expr)
        }
        stack.reverse();
        return; // early return out of this
    }

    match stack.last() {
        Some(expr) => {
            eparser(format!("New Op :: {:?} Last on stack :: {:?}", &operator, &expr));
            match expr {
                Expr::Operator(stack_op) => {
                    if stack_op.lexeme == "(" {
                        eparser(format!("The operator on the stack is an OPEN PAR"));
                        // just push the new op onto the stack regardless
                        let op = Expr::Operator(operator.clone());
                        stack.push(op);
                        return; // early return from this
                    }

                    // In this case, remove the stack op and push it to output
                    if stack_op.precedence >= operator.precedence {
                        eparser(format!("Stack op has a greater precendece than current op"));

                        // Push the stack op to output
                        let stack_op = stack.pop().unwrap(); // shadow stack_op, safe unwrap
                        output.push(stack_op);

                        // Create the current operator as an Expr
                        let op = Expr::Operator(operator.clone());

                        // Push the new op to the stack
                        stack.push(op);
                        drop(operator); // drop original Operator object from the match arm

                    // In this case, put the current op to the top of stack
                    } else {
                        // Create the current operator as an Expr
                        let op = Expr::Operator(operator.clone());

                        // Push the new op to the stack
                        stack.push(op);
                        drop(operator); // drop original Operator object from the match arm
                    }
                }
                _ => panic!("Expected an operator on the stack!"),
            }
        }
        None => {
            // Create the current operator as an Expr
            let op = Expr::Operator(operator.clone());
            eparser(format!("Creating a new operator :: {:?}", &op));

            // Push the new op to the stack
            stack.push(op);
            drop(operator); // drop original Operator object from the match arm
        }
    }
}
