
use std::collections::VecDeque;
use super::super::stack;
use super::super::lexer;
use super::super::eval;
use super::predefined;

//Function that adds the given input values to the stack
fn add_to_stack(function: &stack::Function, mut arguments: VecDeque<eval::Value>, stack: &mut stack::Stack) {

    for i in 0..function.args.len() {

        let name = function.args.get(i).unwrap();

        let val: eval::Value = match arguments.pop_front() {
            Some(e) => e,
            None => eval::get_error()
        };
        
        stack.insert_value(val, name.clone());

    }

}


fn lisp_function(mut function: stack::Function, stack: &mut stack::Stack, arguments: VecDeque<eval::Value> ) -> eval::Value {

    stack.make_frame();

    stack.add_to_stack(&function, arguments);
    
    let res = eval::evaluate(function.input, stack);

    stack.pop_frame();
    res

}

pub fn function_eval(fun: lexer::Entry, stack: &mut stack::Stack, arguments: VecDeque<eval::Value>) -> eval::Value {


    match stack.get_function(fun.lexeme.clone()) {
        Some(function) => lisp_function(function, stack, arguments),
        _ => predefined::predefined_function(fun, stack, arguments)
    }

}
