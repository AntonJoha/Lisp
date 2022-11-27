use super::lexer;
use super::super::eval;
use std::collections::VecDeque;

fn plus(arguments: VecDeque<eval::Value>) -> VecDeque<eval::Value> {
    arguments
}


fn mult(arguments: VecDeque<eval::Value>) -> VecDeque<eval::Value> {
    arguments
}


fn minus( arguments: VecDeque<eval::Value>) -> VecDeque<eval::Value> {
    arguments
}


fn equal(arguments: VecDeque<eval::Value>) -> VecDeque<eval::Value> {
    arguments
}


fn less(arguments: VecDeque<eval::Value>) -> VecDeque<eval::Value> {
    arguments
}


fn div(arguments: VecDeque<eval::Value>) -> VecDeque<eval::Value> {
    arguments
}


fn not(arguments: VecDeque<eval::Value>) -> VecDeque<eval::Value> {
    arguments
}


fn and(arguments: VecDeque<eval::Value>) -> VecDeque<eval::Value> {
    arguments
}


fn or(arguments: VecDeque<eval::Value>) -> VecDeque<eval::Value> {
    arguments
}




pub fn operator_eval(fun: lexer::Entry, arguments: VecDeque<eval::Value>) -> eval::Value {
    let res = match fun.t.clone() {
        lexer::Token::Plus => {
            plus(arguments)
        },
        lexer::Token::Mult => {
            mult(arguments)
        },
        lexer::Token::Minus => {
            minus(arguments)
        },
        lexer::Token::Equal => {
            equal(arguments)
        },
        lexer::Token::Less => {
            less(arguments)
        },
        lexer::Token::Div => {
            div(arguments)
        },
        lexer::Token::Not => {
            not(arguments)
        },
        lexer::Token::And => {
            and(arguments)
        },
        lexer::Token::Or => {
            or(arguments)
        }
        _ => {
            panic!("Should not be here operator_eval");
        }
    };
    eval::Value { literal: "".to_string(), t: lexer::Token::Open, list: res}
}
