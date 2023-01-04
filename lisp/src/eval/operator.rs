use super::super::eval;
use super::lexer;
use std::collections::VecDeque;


pub fn operator_eval(fun: lexer::Entry, arguments: VecDeque<eval::Value>) -> eval::Value {
    match fun.t.clone() {
        lexer::Token::Plus => eval::plus::plus(arguments),
        lexer::Token::Mult => eval::mult::mult(arguments),
        lexer::Token::Minus => eval::minus::minus(arguments),
        lexer::Token::Equal => eval::equal::equal(arguments),
        lexer::Token::Less => eval::less::less(arguments),
        lexer::Token::Div => eval::div::div(arguments),
        lexer::Token::Not => eval::not::not(arguments),
        lexer::Token::And => eval::and::and(arguments),
        lexer::Token::Or => eval::or::or(arguments),
        _ => {
            panic!("Should not be here operator_eval");
        }
    }
}
