use super::super::eval;
use super::super::stack;
use super::super::lexer;
use std::collections::VecDeque;

mod list;
mod exit;
mod print;


fn empty_function(_stack: &mut stack::Stack, _arguments: VecDeque<eval::Value> ) -> eval::Value {

    eval::Value {
        literal: "Error, no function found".to_string(),
        t: lexer::Token::Error,
        list: VecDeque::new()
    }
}

fn get_function(function: lexer::Entry) -> fn(&mut stack::Stack, VecDeque<eval::Value>) -> eval::Value {


    match function.lexeme.as_str() {
        "exit" =>  exit::exit,
        "first" => list::first,
        "rest" => list::rest,
        "get" => list::get,
        "list" => list::list,
        "len" => list::len,
        "print" => print::print,
        _ =>  empty_function
    }

}


pub fn predefined_function(function: lexer::Entry, stack: &mut stack::Stack, arguments: VecDeque<eval::Value> ) -> eval::Value {


    let f = get_function(function);
 
    //A new stack is made just to not have any weird illegal access
    //No new values will actually be pushed with stack.add_to_stack()
    stack.make_frame();
    let res  = f(stack, arguments);
    stack.pop_frame();

    res
}


