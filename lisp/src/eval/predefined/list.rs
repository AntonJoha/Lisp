use super::super::super::eval;
use super::super::super::stack;
use std::collections::VecDeque;
use super::super::super::lexer;


pub fn rest(_stack: &mut stack::Stack,mut arguments: VecDeque<eval::Value>) -> eval::Value {
    
    let mut to_return = arguments.pop_front().unwrap();
    to_return.list.pop_front();
    to_return
}

pub fn get(_stack: &mut stack::Stack,mut arguments: VecDeque<eval::Value>) -> eval::Value {
    
    let pos: eval::Value = arguments.pop_front().unwrap();

    
    match pos.t.clone() {
        lexer::Token::Number => {
            match arguments.pop_front() {
                Some(list) => {
                    list.list.get(pos.literal.parse::<usize>().unwrap()).unwrap().to_owned()
                },
                _ => {
                    panic!("Error, not enough arguments. Expected list");
                }
            }
        },
        _ => {
            panic!("Error, expected positional argument");
        }
    }

}


pub fn first(_stack: &mut stack::Stack,mut arguments: VecDeque<eval::Value> ) -> eval::Value {

    arguments.push_front(eval::Value {
        literal: "0".to_string(),
        t: lexer::Token::Number,
        list: VecDeque::new()
    });
    get(_stack, arguments)
}

pub fn list(_stack: &mut stack::Stack, mut arguments: VecDeque<eval::Value>) -> eval::Value {
    let mut to_return = VecDeque::new();
    while arguments.len() > 0 {
        to_return.push_back(arguments.pop_front().unwrap());
    }
    eval::Value {
        literal: "".to_string(),
        t: lexer::Token::Pure,
        list: to_return,
    }
}



pub fn insert(_stack: &mut stack::Stack, mut argument: VecDeque<eval::Value>) -> eval::Value {

    if argument.len() != 3 {
        return eval::get_error( );
    }

    let insert = argument.pop_front().unwrap();
    let pos = argument.pop_front().unwrap().literal.parse::<usize>().unwrap();
    let mut list = argument.pop_front().unwrap();

    if pos > list.list.len() {
        list.list.push_back(insert);
    }
    else {
        list.list.insert(pos, insert);
    }
    eval::Value {
        literal: "".to_string(),
        t: lexer::Token::Pure,
        list: list.list,
    }
}


pub fn len(_stack: &mut stack::Stack, mut argument: VecDeque<eval::Value>) -> eval::Value {

    let to_return = argument.pop_front().unwrap();
    eval::Value {
        literal: to_return.list.len().to_string(),
        t: lexer::Token::Number,
        list: VecDeque::new()
    }

}

