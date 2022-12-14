use super::lexer;
use std::collections::VecDeque;

pub mod operator;
pub mod plus;
pub mod minus;

pub struct Value {
    pub literal: String,
    pub t: lexer::Token,
    pub list: VecDeque<Value>,
}

pub fn get_error() -> Value {
    Value {
        literal: "".to_string(),
        t: lexer::Token::Error,
        list: VecDeque::new(),
    }
}

pub fn call_func(fun: lexer::Entry, arguments: VecDeque<Value>) -> Value {
    match fun.t.clone() {
        lexer::Token::Id => {
            panic!("Not implemented");
        }
        _ => operator::operator_eval(fun, arguments),
    }
}

pub fn process(input: &mut VecDeque<lexer::Entry>) -> Value {
    let mut to_return: VecDeque<Value> = VecDeque::new();

    let fun = match input.pop_front() {
        Some(e) => e,
        _ => lexer::Entry {
            lexeme: "Error".to_string(),
            t: lexer::Token::Error,
        },
    };

    match fun.t.clone() {
        lexer::Token::Open => {
            to_return.push_back(process(input));
        }
        l => {
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
                        result = call_func(fun, argument);
                        break;
                    }
                    lexer::Token::Open => {
                        argument.push_back(process(input));
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

pub fn evaluate(mut input: VecDeque<lexer::Entry>) {
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
                let result = process(&mut input);
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
    println!("");
}
