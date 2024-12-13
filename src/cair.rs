// Code Analysis Intermediate Representation (CAIR)
// Semantic analysis, name resolution, etc.

use std::collections::HashMap;

use crate::{lexer::op::Operator, parser::Expr};

#[derive(Debug)]
pub enum IrExpr {
    Identifier(String),
    Float(f32),
    Integer(i32),
    Exit(usize),
    Add(Box<IrExpr>, Box<IrExpr>),
    Sub(Box<IrExpr>, Box<IrExpr>),
    Mul(Box<IrExpr>, Box<IrExpr>),
    Div(Box<IrExpr>, Box<IrExpr>),
    Store(Box<IrExpr>, Box<IrExpr>),
}

impl From<Expr> for IrExpr {
    fn from(value: Expr) -> Self {
        match value {
            Expr::Integer(n) => IrExpr::Integer(n),
            Expr::Float(n) => IrExpr::Float(n),
            Expr::Operator(_) => panic!("Tried to make an IrExpr from an operator"),
            Expr::Identifier(n) => IrExpr::Identifier(n),
            Expr::End => IrExpr::Exit(0),
        }
    }
}

#[derive(Debug)]
pub struct IRGen {
    stream: Vec<Expr>,
    //stack: Vec<IrExpr>,
    output: Vec<IrExpr>
}

impl IRGen {
    pub fn new(stream: Vec<Expr>) -> Self {
        Self {
            //stack: vec![],
            stream,
            output: vec![],
        }
    }

    pub fn gen(&mut self) {
        self.stream.reverse();
        while let Some(expr) = self.stream.pop() {
            match expr {
                Expr::Operator(op) => self.on_operator(op),
                _ => self.output.push(IrExpr::from(expr)),
            }
        }


    }

    fn on_operator(&mut self, op: Operator) {
        match op.lexeme {
            "+" | "-" | "*" | "/" => {
                // Get the last two things off of the stack
                let lhs_expr = self.output.pop().unwrap_or_else(|| {
                    panic!("Expected an LHS value on the stack!");
                });
                let rhs_expr = self.output.pop().unwrap_or_else(|| {
                    panic!("Expected an LHS value on the stack!");
                });

                let lhs = IrExpr::from(lhs_expr);
                let rhs = IrExpr::from(rhs_expr);

                match op.lexeme {
                    "+" => self.output.push(IrExpr::Add(Box::new(rhs), Box::new(lhs))),
                    "-" => self.output.push(IrExpr::Sub(Box::new(rhs), Box::new(lhs))),
                    "*" => self.output.push(IrExpr::Mul(Box::new(rhs), Box::new(lhs))),
                    "/" => self.output.push(IrExpr::Div(Box::new(rhs), Box::new(lhs))),
                    _ => panic!("This case is literally not possible"),
                }
            },
            "=" => {
                // Get the value assigned to the identifier from the output
                let value = self.output.pop().unwrap_or_else(|| {
                    panic!("Expected a value on the stack!");
                });

                // Get the identifier off the output
                let ident = self.output.pop().unwrap_or_else(|| {
                    panic!("Expected an identifier on the stack!");
                });

                let value = IrExpr::from(value);
                let ident = IrExpr::from(ident);

                self.output.push(IrExpr::Store(Box::new(ident), Box::new(value)));
            }
            _ => panic!("Unknown operator -- {}", op.lexeme),
        }
    }
}