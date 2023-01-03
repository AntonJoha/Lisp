use std::collections::VecDeque;
use super::super::lexer;
use super::super::eval;
use super::super::stack;


fn evaluate(input: &mut VecDeque<lexer::Entry>, stack: &mut stack::Stack) -> eval::Value {

    let mut to_operate: VecDeque<lexer::Entry> = VecDeque::new();

    let mut count: i32 = 1;
    
    let first = input.pop_front().unwrap();
    
    match first.t.clone() {
        lexer::Token::Open => (),
        _ => {return eval::Value{
            literal: first.lexeme,
            t: first.t,
            list: VecDeque::new()
            };
        }
    };

    loop {
        let val = input.pop_front().unwrap();
        
        match val.t.clone() {
            lexer::Token::Open => {count += 1;},
            lexer::Token::Close => {count -= 1;}
            _ => ()
        };
        to_operate.push_back(val);
        if count == 0 {
            return eval::process(&mut to_operate, stack);
        }
    }
}

fn get_execution(input: &mut VecDeque<lexer::Entry>) -> VecDeque<lexer::Entry> {
    
    let mut to_return: VecDeque<lexer::Entry> = VecDeque::new();

    let mut count: i32 = 1;

    let first = input.pop_front().unwrap();
 
    match first.t.clone() {
        lexer::Token::Open => (),
        _ => {
            to_return.push_front(first);
            return to_return;
        }
    }


    loop {
        let val = input.pop_front().unwrap();
        
        match val.t.clone() {
            lexer::Token::Open => {count += 1;},
            lexer::Token::Close => {count -= 1;}
            _ => ()
        };
        to_return.push_back(val);
        if count == 0 {
            return to_return;
        }
    }


}


fn execute(input: &mut VecDeque<lexer::Entry>, stack: &mut stack::Stack) -> eval::Value {

    if input.len() == 1 {
        let value = input.pop_front().unwrap();
        eval::Value {
            literal: value.lexeme,
            t: value.t,
            list: VecDeque::new()
        }
    }
    else {
        eval::process(input, stack)
    }

}

fn execute_first(input: &mut VecDeque<lexer::Entry>, stack: &mut stack::Stack) -> eval::Value {

    let mut first = get_execution(input);
    //Throw the second
    get_execution(input);

    //Remove an extra closing parentheses
    input.pop_front();

    execute(&mut first, stack)
}


fn execute_second(input: &mut VecDeque<lexer::Entry>, stack: &mut stack::Stack) -> eval::Value {
    get_execution(input);
    let mut second = get_execution(input);
    
    //Remove an extra closing parentheses
    input.pop_front();

    execute(&mut second, stack)
}

pub fn handle_if(input: &mut VecDeque<lexer::Entry>, stack: &mut stack::Stack) -> eval::Value {


    let result: eval::Value = evaluate(input, stack);

    if result.literal != "0" {
        execute_first(input, stack)
    }
    else {
        execute_second(input, stack)
    }
}
