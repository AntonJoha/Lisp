use lisp::stack;
use std::collections::VecDeque;
use lisp::lexer;
use lisp::parser;
use lisp::eval;
use std::fs;

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


pub fn test_file(filepath: String) -> eval::Value {

     

    let content = fs::read_to_string(filepath.clone())
        .expect("Could not read file");

        let mut a = match lexer::lexer(content) {
            Ok(e) => e,
            _ => {
                println!("Weird input");
                return eval::get_error();
            }
        };

        if !parser::parse(&mut a) {
            //panic!("Parse failed {}", filepath);
        }


    //Initial stackframe
    let mut stack: stack::Stack = stack::Stack{frames: VecDeque::new()};
    stack.make_frame();

    eval::evaluate(a, &mut stack)


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

