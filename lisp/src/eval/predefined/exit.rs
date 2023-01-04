use super::super::super::eval;
use super::super::super::stack;
use std::collections::VecDeque;
use std::process;



pub fn exit(_stack: &mut stack::Stack, _arguments: VecDeque<eval::Value> ) -> eval::Value {


    println!("Exit received, turning off...");
    process::exit(0);
}
