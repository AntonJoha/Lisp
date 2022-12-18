
use std::collections::VecDeque;

use super::super::eval;
use super::super::lexer;

fn false_val() -> eval::Value {
    eval::Value {
        literal: "0".to_string(),
        t: lexer::Token::Number,
        list: VecDeque::new(),
    }
}


fn true_val() -> eval::Value {
    eval::Value {
        literal: "1".to_string(),
        t: lexer::Token::Number,
        list: VecDeque::new(),
    }
}

fn one_arg(mut arguments: VecDeque<eval::Value>) -> eval::Value {
        if arguments.pop_front().unwrap().literal != "0" {
            true_val()
        }
        else {
            false_val()
        }
}

fn multiple_equal(mut arguments: VecDeque<eval::Value>) -> eval::Value {

    let first: String = arguments.pop_front().unwrap().literal;

    while arguments.len() > 0 {
        if first != arguments.pop_front().unwrap().literal {
            return false_val();
        }
    }
    true_val()
}


pub fn equal(arguments: VecDeque<eval::Value>) -> eval::Value {

    if arguments.len() == 1 {
        one_arg(arguments)
    }
    else if arguments.len() == 0 {
        false_val()
    }
    else {
        multiple_equal(arguments)
    }
}
