use lisp::stack;
use std::collections::VecDeque;
use lisp::lexer;
use lisp::parser;
use lisp::eval;

pub struct Fakerand {
    pub value: u128
}

impl Fakerand {

    pub fn seed(&mut self, value: u128 ) {
        self.value = value;
    }
    pub fn get_uint(&mut self) -> u128 {
        self.value ^= 1423424 + self.value + 234;
        self.value ^= 23491 + self.value;
        self.value
    }
}


pub fn get_stack() -> stack::Stack {
    
    let mut stack: stack::Stack = stack::Stack{frames: VecDeque::new()};
    stack.make_frame();
    
    stack
}


pub fn parse_input(input: String) -> VecDeque<lexer::Entry> {

    let mut a = match lexer::lexer(input) {
        Ok(e) => e,
        _ => {
            return VecDeque::new();
        }
    };

    match  parser::parse(&mut a) {
        true => a,
        false => VecDeque::new()
    }

}


//Just a simple wrapper if the user wants to run a single argument 
pub fn run_input(input: String) -> eval::Value {
    run_multiple(vec!(input))
}


pub fn run_multiple(input: Vec<String>) -> eval::Value {

    //The input should not be empty or length
    let mut stack = get_stack();

    let mut val: eval::Value = eval::get_error();

    for i in input {
        let mut instruction = parse_input(i);
        instruction.pop_front();
        assert_ne!(instruction.len(), 0);

        val = eval::process(&mut instruction, &mut stack);
    }

    val
}

