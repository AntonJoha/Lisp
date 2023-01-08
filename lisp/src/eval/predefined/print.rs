use super::super::super::eval;
use super::super::super::stack;
use std::collections::VecDeque;
use super::super::super::lexer;




pub fn print_rec( arguments: VecDeque<eval::Value> ) {

    for a in arguments {
        if a.list.len() == 0 {
            print!("{} ", a.literal);
        }
        else {
            print!("( ");
            print_rec(a.list);
            print!(")");
        }

    }

}

pub fn print(_stack: &mut stack::Stack, arguments: VecDeque<eval::Value> ) -> eval::Value {

    print_rec(arguments);

    println!("");
    eval::none_value()
}


