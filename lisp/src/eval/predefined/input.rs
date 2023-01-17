use super::super::super::eval;
use super::super::super::stack;
use std::collections::VecDeque;
use super::super::super::lexer;




pub fn input(_stack: &mut stack::Stack, arguments: VecDeque<eval::Value> ) -> eval::Value {


    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    eval::Value {
        literal: line,
        t: lexer::Token::String,
        list: VecDeque::new()
    }
}



