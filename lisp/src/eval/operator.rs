use super::super::eval;
use super::lexer;
use std::collections::VecDeque;

fn place_holder() -> eval::Value {
    eval::get_error()
}

fn mult(arguments: VecDeque<eval::Value>) -> eval::Value {
    todo!();
    place_holder()
}

fn minus(arguments: VecDeque<eval::Value>) -> eval::Value {
    todo!();
    place_holder()
}

fn equal(arguments: VecDeque<eval::Value>) -> eval::Value {
    todo!();
    place_holder()
}

fn less(arguments: VecDeque<eval::Value>) -> eval::Value {
    todo!();
    place_holder();
}

fn div(arguments: VecDeque<eval::Value>) -> eval::Value {
    todo!();
    place_holder()
}

fn not(arguments: VecDeque<eval::Value>) -> eval::Value {
    todo!();
    place_holder()
}

fn and(arguments: VecDeque<eval::Value>) -> eval::Value {
    todo!();
    place_holder()
}

fn or(arguments: VecDeque<eval::Value>) -> eval::Value {
    todo!();
    place_holder()
}

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
