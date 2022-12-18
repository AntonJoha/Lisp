use std::collections::VecDeque;

use super::super::eval;
use super::super::lexer;

fn string_mult(first: eval::Value, second: eval::Value) -> eval::Value{

    let mut amount: i128 = 0;
    let mut s: String = String::new();

    if first.t == lexer::Token::Number {
        amount = first.literal.parse::<i128>().unwrap() as i128;
        s.push_str(second.literal.as_str());
    }
    else {
        amount = second.literal.parse::<i128>().unwrap() as i128;
        s.push_str(first.literal.as_str());
    }

    let mut to_return: String = String::new();

    for _ in 0..amount {
        to_return.push_str(s.as_str());
    }
    eval::Value { literal: to_return,
        t: lexer::Token::String,
        list: VecDeque::new()
    }
}


fn more(mut arguments: VecDeque<eval::Value>) -> eval::Value {
    let mut to_return: VecDeque<eval::Value> = VecDeque::new();

    let first = match arguments.pop_front() {
        Some(e) => e,
        _ => eval::get_error(),
    };
    let second = match arguments.pop_front() {
        Some(e) => e,
        _ => eval::get_error(),
    };

    let new_type: lexer::Token =
        lexer::operator(lexer::Token::Mult, first.t.clone(), second.t.clone());

    match new_type {
        lexer::Token::Number => {
            arguments.push_front(eval::Value {
                literal: {
                    (first.literal.parse::<i128>().unwrap() as i128
                        * second.literal.parse::<i128>().unwrap() as i128)
                        .to_string()
                },
                t: { lexer::Token::Number },
                list: { VecDeque::new() },
            });
        }
        lexer::Token::Float => {
            arguments.push_front(eval::Value {
                literal: {
                    (first.literal.parse::<f64>().unwrap() as f64
                        * second.literal.parse::<f64>().unwrap() as f64)
                        .to_string()
                },
                t: { lexer::Token::Float },
                list: { VecDeque::new() },
            });
        }
        lexer::Token::String => {
            arguments.push_front(string_mult(first, second));
        }
        _ => {
            arguments.push_front(eval::Value {
                literal: { "".to_string() },
                t: { lexer::Token::Error },
                list: { VecDeque::new() },
            });
        }
    };
    arguments.pop_front().unwrap()
}

pub fn mult(mut arguments: VecDeque<eval::Value>) -> eval::Value {
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
        arguments.push_back(res);
        return mult(arguments);
    }
    to_return.pop_front().unwrap()
}
