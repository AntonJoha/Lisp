use super::lexer;
use std::collections::VecDeque;
use super::stack;

pub mod and;
pub mod div;
pub mod equal;
pub mod less;
pub mod minus;
pub mod mult;
pub mod not;
pub mod operator;
pub mod or;
pub mod plus;
pub mod condition;
pub mod predefined;


pub mod function;

#[derive(Clone)]
pub struct Value {
    pub literal: String,
    pub t: lexer::Token,
    pub list: VecDeque<Value>,
}


pub fn none_value() -> Value {
    Value {
        literal: "1".to_string(),
        t: lexer::Token::Number,
        list: VecDeque::new()
    }
}

pub fn get_error() -> Value {
    Value {
        literal: "0".to_string(),
        t: lexer::Token::Error,
        list: VecDeque::new(),
    }
}

fn get_list(input:&mut VecDeque<lexer::Entry>, stack: &mut stack::Stack) -> Value {
    
    let mut to_return: VecDeque<Value> = VecDeque::new();

    //Get rid of the opening parentheses
    input.pop_front();

    loop {

        let value = match input.pop_front() {
            Some(e) => e,
            _ => {panic!("Error no value found");}
        };

        match value.t.clone() {
            lexer::Token::Pure => {
                to_return.push_back(get_list(input, stack));
            },
            lexer::Token::Close => {
                return Value {
                    literal: "".to_string(),
                    t: lexer::Token::Pure,
                    list: to_return,
                };
            },
            lexer::Token::Open => {
                to_return.push_back(process(input, stack));
            },
            _ => {
                to_return.push_back(Value {
                    literal: value.lexeme,
                    t: value.t,
                    list: VecDeque::new(),
                });
            },
        }
    }
}



fn get_arguments(arguments:VecDeque<Value>, stack: &mut stack::Stack) -> VecDeque<Value> {
    let mut to_return: VecDeque<Value> = VecDeque::new();


    for value in arguments {
        if value.t.clone() == lexer::Token::Id {
            to_return.push_back(stack.get_value(value.literal));
        }
        else {
            to_return.push_back(value);
        }
    }
    to_return
}

pub fn call_func(fun: lexer::Entry, arguments: VecDeque<Value>, stack: &mut stack::Stack) -> Value {

    let args = get_arguments(arguments, stack);


    match fun.t.clone() {
        lexer::Token::Id => {
            function::function_eval(fun, stack, args)
        }
        _ => operator::operator_eval(fun, args),
    }
}

pub fn process(input: &mut VecDeque<lexer::Entry>, stack: &mut stack::Stack) -> Value {
    

    let mut to_return: VecDeque<Value> = VecDeque::new();

    let fun = match input.pop_front() {
        Some(e) => e,
        _ => lexer::Entry {
            lexeme: "Error".to_string(),
            t: lexer::Token::Error,
        },
    };

    match fun.lexeme.clone().as_ref(){
        "def" => {
        let s = stack.insert_function(input);
        //Remove the last ')' as well 
        input.pop_front();
        return Value {literal: s,
            t: lexer::Token::Id,
            list: VecDeque::new()};
        },
        "if" => {
            return condition::handle_if(input, stack);
        }
        _ => ()
    };

    match fun.t.clone() {
        lexer::Token::Open => {
            to_return.push_back(process(input, stack));
        }
        _ => {
            let mut result: Value;
            let mut argument: VecDeque<Value> = VecDeque::new();
            loop {
                let t: lexer::Entry = match input.pop_front() {
                    Some(e) => e,
                    _ => lexer::Entry {
                        lexeme: ")".to_string(),
                        t: lexer::Token::Close,
                    },
                };
                match t.t.clone() {
                    lexer::Token::Close => {
                        result = call_func(fun, argument, stack);
                        break;
                    }
                    lexer::Token::Open => {
                        argument.push_back(process(input, stack));
                    }
                    lexer::Token::Id => {
                        if t.lexeme.clone() == "def".to_string()
                        {
                            stack.insert_function(input);
                        }
                        else {
                            argument.push_back(Value {literal: t.lexeme,
                                               t: t.t,
                                               list: VecDeque::new()});
                        }
                    },
                    lexer::Token::Pure => {

                        argument.push_back(get_list(input, stack));

                    }
                    _ => {
                        argument.push_back(Value {
                            literal: t.lexeme,
                            t: t.t,
                            list: VecDeque::new(),
                        });
                    }
                }
            }
            to_return.push_back(result);
        }
    }


    if to_return.len() > 1 {
        Value {
            literal: "".to_string(),
            t: lexer::Token::Open,
            list: to_return,
        }
    } else if to_return.len() == 1 {
        match to_return.pop_front() {
            Some(v) => Value {
                literal: v.literal,
                t: v.t,
                list: v.list,
            },
            _ => Value {
                literal: "Error".to_string(),
                t: lexer::Token::Error,
                list: VecDeque::new(),
            },
        }
    } else {
        Value {
            literal: "0".to_string(),
            t: lexer::Token::Number,
            list: VecDeque::new(),
        }
    }
}

fn print_value(v: Value) {
    if v.t == lexer::Token::Open {
        print!("( ");
        for a in v.list {
            print_value(a);
        }
        print!(") ");
    } else {
        print!("{} ", v.literal);
    }
}

pub fn evaluate(mut input: VecDeque<lexer::Entry>, stack: &mut stack::Stack) {
    while input.len() > 0 {
        let entry = match input.pop_front() {
            Some(e) => e,
            _ => lexer::Entry {
                lexeme: "Error".to_string(),
                t: lexer::Token::Error,
            },
        };

        match entry.t.clone() {
            lexer::Token::Open => {
                let result = process(&mut input,  stack);
                print_value(result);
            }
            //lexer::Token::Pure => {
            //    let result = purelist(&input);
            //},
            _ => {
                print!("{}", entry.lexeme);
            }
        }
    }
}

