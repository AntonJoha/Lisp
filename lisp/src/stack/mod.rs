use std::collections::HashMap;
use std::collections::VecDeque;

use super::eval;
use super::lexer;

#[derive(Clone)]
pub struct Function{
    pub args: VecDeque<String>,
    pub input: VecDeque<lexer::Entry>,
}




fn get_input(input: &mut VecDeque<lexer::Entry>) -> VecDeque<lexer::Entry> {
    let mut to_return: VecDeque<lexer::Entry> = VecDeque::new();

    let mut count = 1;
    
    //The first open parentheses
    let v = input.pop_front().unwrap();
    to_return.push_back(v);


    loop {
        
        let v = input.pop_front().unwrap();

        match v.t {
            lexer::Token::Open => {count += 1; to_return.push_back(v);},
            lexer::Token::Close => { count -= 1;
                to_return.push_back(v); 
                if count < 1 {
                    let v = input.pop_front().unwrap();
                    match  v.t.clone() {
                        lexer::Token::Open => {to_return.push_back(v); 
                            count += 1 ;
                            continue;
                        },
                        _ => {input.push_front(v); return to_return;},
                    }
                }
            },
            _ => to_return.push_back(v)
        };

    }

}

fn get_args(input: &mut VecDeque<lexer::Entry>) -> VecDeque<String> {

    let mut to_return: VecDeque<String> = VecDeque::new();


    let val = input.pop_front().unwrap();

    println!("{:?} {}", val.t, val.lexeme);

    loop {
    
        let curr = input.pop_front().unwrap();
        

        if curr.t == lexer::Token::Close {
            return to_return;
        }
        else {
            to_return.push_back(curr.lexeme);
        }
    }
}

pub struct StackFrame {
    values: HashMap<String, eval::Value>,
    func: HashMap<String, Function>,
}

impl StackFrame {
    
    pub fn insert_value(&mut self, value: eval::Value, index: String) -> String{
        self.values.insert(index.clone(),  value);
        index
    }
    pub fn get_value(&self, index: &String) -> Option<eval::Value> {
        
        match self.values.get(index) 
        {
            Some(e) =>  Some(e.clone()),
            _ => None,
        }
    }

    pub fn insert_function(&mut self,  input: &mut VecDeque<lexer::Entry>) -> String{
        
    
        let name: String = match input.pop_front() {
            Some(e) => e.lexeme.clone(),
            _ => panic!("Error with function declaration"),
        };

        
        let args: VecDeque<String> = get_args(input);
        

        let input: VecDeque<lexer::Entry> = get_input(input);

        self.func.insert(name.clone(), Function { args: args, input: input });
        name
    }

    pub fn get_function(&self, index: &String) -> Option<Function> {
 
        match self.func.get(index) 
        {
            Some(e) =>  Some(e.clone()),
            _ => None,
        }

    }
}

pub struct Stack {
   pub frames: VecDeque<StackFrame>,
}

impl Stack {
    

    //Function that adds the given input values to the stack
    pub fn add_to_stack(&mut self, function: &Function, mut arguments: VecDeque<eval::Value>) {

        for i in 0..function.args.len() {

            let name = function.args.get(i).unwrap();

            let val: eval::Value = match arguments.pop_front() {
                Some(e) => e,
                None => eval::get_error()
            };
        
            self.insert_value(val, name.clone());

        }

}




    pub fn make_frame(&mut self) {
        self.frames.push_front(StackFrame { values: HashMap::new(), func:HashMap::new() } );
    }

    
    pub fn pop_frame(&mut self) {
        self.frames.pop_front();
    }


    fn empty_stack(&self) -> bool {
        self.frames.len() == 0
    }

    pub fn insert_function(&mut self, input: &mut VecDeque<lexer::Entry>) -> String{
    
        if self.empty_stack() {
            panic!("Can't add functions to empty stack");
        }
        self.frames.get_mut(0).unwrap().insert_function(input)

    }

    pub fn insert_value(&mut self, value: eval::Value, index: String) -> String{
    
        if self.empty_stack() {
            panic!("Can't add functions to empty stack");
        }
        self.frames.get_mut(0).unwrap().insert_value(value, index)

    }


    //Goes up the whole stack to the end
    pub fn get_value(&self, index: String) -> eval::Value {

        for frame in &self.frames {
            match frame.get_value(&index) {
                Some(e) => return e,
                _ => ()
            };
        }
        return eval::get_error();
    }

    //Goes up the whole stack to the end
    pub fn get_function(&self, index: String) -> Option<Function> {

        for frame in &self.frames {
            match frame.get_function(&index) {
                Some(e) => return Some(e),
                _ => ()
            };
        }
        None
    }

}


