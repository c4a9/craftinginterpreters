use std::os::macos::raw::stat;

use super::{
    ast::{Expression, Statement},
    token::Token,
    token::TokenType,
};
use crate::lexer::{lexer::Lexer, token::Object};
use TokenType::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[allow(dead_code)]
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = vec![];
        while !self.is_at_end() {
            statements.push(self.statement())
        }
        statements
    }

    fn statement(&mut self) -> Statement {
        if self.expect(vec![PRINT]) {
            return self.print();
        }

        self.expression_statement()
    }

    fn print(&mut self) -> Statement {
        let value = self.expression();

        Statement::Print(value)
    }

    fn expression_statement(&mut self) -> Statement {
        Statement::Expression(self.expression())
    }

    // expression     → equality ;
    // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    // term           → factor ( ( "-" | "+" ) factor )* ;
    // factor         → unary ( ( "/" | "*" ) unary )* ;
    // unary          → ( "!" | "-" ) unary
    //                | primary ;
    // primary        → NUMBER | STRING | "true" | "false" | "nil"
    //                | "(" expression ")" ;
    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> Expression {
        println!("final step");
        let mut expr = self.comparison();

        while self.expect(vec![BANG_EQUAL, EQUAL_EQUAL]) {
            let op = self.previous();
            let cmp = self.comparison();
            expr = Expression::Binary(Box::new(expr), op, Box::new(cmp));
        }

        expr
    }

    fn comparison(&mut self) -> Expression {
        println!("fifth step");

        let mut expr = self.term();

        while self.expect(vec![GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let op = self.previous();
            let right = self.term();
            expr = Expression::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn term(&mut self) -> Expression {
        println!("fourth step");
        let mut expr = self.factor();

        while self.expect(vec![MINUS, PLUS]) {
            let op = self.previous();
            let right = self.factor();
            expr = Expression::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn factor(&mut self) -> Expression {
        println!("third setp");
        let mut expr = self.unary();

        while self.expect(vec![SLASH, STAR]) {
            let op = self.previous();
            let right = self.unary();
            expr = Expression::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn unary(&mut self) -> Expression {
        println!("second step");
        if self.expect(vec![BANG, MINUS]) {
            let op = self.previous();
            let right = self.unary();
            return Expression::Unary(op, Box::new(right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Expression {
        println!("first reach ");
        if self.expect(vec![FALSE]) {
            return Expression::Literal(Object::Bool(false));
        }

        if self.expect(vec![TRUE]) {
            return Expression::Literal(Object::Bool(true));
        }

        if self.expect(vec![NUMBER, STRING]) {
            return Expression::Literal(self.previous().literal);
        }

        if self.expect(vec![LEFT_PAREN]) {
            let expr = self.expression();
            self.consume(RIGHT_PAREN, "expect ')' after expression".to_string());
            return Expression::Grouping(Box::new(expr));
        }

        Expression::Mark
    }

    fn consume(&mut self, tag: TokenType, message: String) -> Token {
        if self.check(tag) {
            return self.advance();
        }

        panic!("{}", message)
    }

    /// 如果找到了一个符合条件的token，同时指针后移
    fn expect(&mut self, tokens: Vec<TokenType>) -> bool {
        if tokens.iter().any(|t| self.check(*t)) {
            self.advance();
            return true;
        }

        false
    }

    /// 给定一个 token 类型，判断当前 token 是否符合
    fn check(&mut self, tag: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().tag == tag
    }

    /// current 指针向前移动，但是返回当前 token
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        };
        self.previous()
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    /// 返回上一个token，current 指针不变
    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn is_at_end(&mut self) -> bool {
        self.current == self.tokens.len()
    }
}

#[test]
fn test() {
    // FIXME: Option Unwrap Error
    let mut l = Lexer::new(String::from("1+6/(3+3)*2"));
    l.scan_tokens();

    let mut parser = Parser::new(l.tokens);
    let exp = parser.parse();

    println!("{:#?}", exp);
}
