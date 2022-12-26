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

fn compare(first: &eval::Value, second: &eval::Value, t: &lexer::Token) -> bool {
    match t {
        lexer::Token::Number => {
            (first.literal.parse::<i128>().unwrap() as i128)
                < (second.literal.parse::<i128>().unwrap() as i128)
        }
        lexer::Token::Float => {
            (first.literal.parse::<f64>().unwrap() as f64)
                < (second.literal.parse::<f64>().unwrap() as f64)
        }
        _ => false,
    }
}

fn type_check_helper(first: &lexer::Token, second: &lexer::Token) -> bool {
    match (first, second) {
        (lexer::Token::Number, lexer::Token::Number) => true,
        (lexer::Token::Number, lexer::Token::Float) => true,
        (lexer::Token::Float, lexer::Token::Float) => true,
        (_, _) => false,
    }
}

fn type_check(first: &lexer::Token, second: &lexer::Token) -> bool {
    match type_check_helper(first, second) {
        false => type_check_helper(second, first),
        true => true,
    }
}

fn check_less(mut arguments: VecDeque<eval::Value>) -> eval::Value {
    let mut curr = arguments.pop_front().unwrap();

    if type_check(&curr.t, &curr.t) == false {
        return eval::get_error();
    }

    while arguments.len() > 0 {
        let value = arguments.pop_front().unwrap();

        if type_check(&curr.t, &value.t) == false {
            return false_val();
        } else if compare(&curr, &value, &curr.t) == false {
            return false_val();
        }
        curr = value;
    }
    true_val()
}

pub fn less(arguments: VecDeque<eval::Value>) -> eval::Value {
    if arguments.len() < 2 {
        false_val()
    } else {
        check_less(arguments)
    }
}
