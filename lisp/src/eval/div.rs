use std::collections::VecDeque;

use super::super::eval;
use super::super::lexer;

fn more(mut arguments: VecDeque<eval::Value>) -> eval::Value {
    let first = match arguments.pop_front() {
        Some(e) => e,
        _ => eval::get_error(),
    };

    let second = match arguments.pop_front() {
        Some(e) => e,
        _ => eval::get_error(),
    };

    let new_type: lexer::Token =
        lexer::operator(lexer::Token::Div, first.t.clone(), second.t.clone());

    match new_type {
        lexer::Token::Number => eval::Value {
            literal: {
                (first.literal.parse::<i128>().unwrap() as i128
                    / second.literal.parse::<i128>().unwrap() as i128)
                    .to_string()
            },
            t: { lexer::Token::Number },
            list: { VecDeque::new() },
        },
        lexer::Token::Float => eval::Value {
            literal: {
                (first.literal.parse::<f64>().unwrap() as f64
                    / second.literal.parse::<f64>().unwrap() as f64)
                    .to_string()
            },
            t: { lexer::Token::Float },
            list: { VecDeque::new() },
        },
        _ => eval::get_error(),
    }
}

pub fn div(mut arguments: VecDeque<eval::Value>) -> eval::Value {
    let mut to_return: VecDeque<eval::Value> = VecDeque::new();
    if arguments.len() == 1 {
        to_return.push_back(match arguments.pop_front() {
            Some(e) => e,
            _ => eval::get_error(),
        });
    } else {
        let mut t: VecDeque<eval::Value> = VecDeque::new();
        t.push_back(arguments.pop_front().unwrap());
        t.push_back(arguments.pop_front().unwrap());
        let res = more(t);
        arguments.push_front(res);
        return div(arguments);
    }
    to_return.pop_front().unwrap()
}
