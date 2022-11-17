use super::lexer;
use std::collections::VecDeque;


fn peek(input: &mut VecDeque<lexer::Entry>, t: lexer::Token) -> bool {
    match input.get(0) {
        Some(v) => {
            if v.t == t {
                true
            }
            else {
                false
            }
        },
        _ => false
    }
}

fn match_entry(input: &mut VecDeque<lexer::Entry>, t: lexer::Token) -> bool {
    if peek(input, t) {
        match input.pop_front() {
            Some(e) => {
                input.push_back(e);
                    true
            },
            None => false //This should never happen due to the previous check
        }
    }
    else {
        true
    }
    
}

fn entry(input: &mut VecDeque<lexer::Entry>) -> bool {
    if peek(input, lexer::Token::Open) {
        expression_list(input)
    }
    else if peek(input, lexer::Token::Pure) {
        list(input)
    }
    else {
        match_entry(input, lexer::Token::Number) ||
        match_entry(input, lexer::Token::Id) ||
        match_entry(input, lexer::Token::Float)
    }
}

fn entry_list(input: &mut VecDeque<lexer::Entry>) -> bool {
    if peek(input, lexer::Token::Close) {
        true
    }
    else {
        entry(input) && 
        entry_list(input)
    }
}

fn list(input: &mut VecDeque<lexer::Entry>) -> bool {
    match_entry(input, lexer::Token::Pure) &&
    match_entry(input, lexer::Token::Open) &&
    entry_list(input) &&
    match_entry(input, lexer::Token::Close)
}

fn func(input: &mut VecDeque<lexer::Entry>) -> bool {
    panic!("TODO THIS");
}

fn evals(input: &mut VecDeque<lexer::Entry>) -> bool {
    match_entry(input, lexer::Token::Open) &&
    func(input) &&
    entry_list(input) &&
    match_entry(input, lexer::Token::Close)
}

fn expression_list(input: &mut VecDeque<lexer::Entry>) -> bool{
    expression(input) && 
    {
        if peek(input, lexer::Token::Open) {
            expression_list(input)
        }
        else { true }
    }

}

fn expression(input: &mut VecDeque<lexer::Entry>) ->  bool{
    //If it's eval
   if peek(input, lexer::Token::Open) {
        evals(input)
   }
   //If it's a pure list
   else if peek(input, lexer::Token::Pure) {
        list(input)
   }
   else {
        false
   }
}


pub fn parse(input: &mut VecDeque<lexer::Entry>) -> bool {
    return expression_list(input);
}


