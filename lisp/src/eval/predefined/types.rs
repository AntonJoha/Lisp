use super::super::super::eval;
use super::super::super::stack;
use std::collections::VecDeque;
use super::super::super::lexer;



fn handle_list(arguments: VecDeque<eval::Value>, t: lexer::Token) -> VecDeque<eval::Value> {

    let mut to_return = VecDeque::new();

    for arg in arguments {
        to_return.push_back(convert(arg, t.clone()));
    }

    to_return
}


fn convert(args: eval::Value, t: lexer::Token) -> eval::Value {


    if args.list.len() == 0 {
        eval::Value {
            literal: args.literal,
            t: t,
            list: VecDeque::new()
        }
    } else {
        eval::Value {
            literal: "".to_string(),
            t: lexer::Token::Pure,
            list: handle_list(args.list, t)
        }
    }
}


fn type_conversion(mut arguments: VecDeque<eval::Value>, t: lexer::Token ) -> eval::Value {

    if arguments.len() > 1 {
        eval::Value {
            literal: "".to_string(),
            t: lexer::Token::Pure,
            list: handle_list(arguments, t)
        }
    }
    else if arguments.len() == 1 {
        convert(arguments.pop_front().unwrap(), t)
    }
    else {
        eval::get_error()
    }


}

pub fn string(_stack: &mut stack::Stack, arguments: VecDeque<eval::Value> ) -> eval::Value {
    type_conversion(arguments, lexer::Token::String)
}

pub fn float(_stack: &mut stack::Stack, arguments: VecDeque<eval::Value> ) -> eval::Value {
    type_conversion(arguments, lexer::Token::Float)
}

pub fn integer(_stack: &mut stack::Stack, arguments: VecDeque<eval::Value> ) -> eval::Value {
    type_conversion(arguments, lexer::Token::Number)
}

pub fn id(_stack: &mut stack::Stack, arguments: VecDeque<eval::Value> ) -> eval::Value {
    type_conversion(arguments, lexer::Token::Id)
}
