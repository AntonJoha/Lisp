use super::lexer;
use std::collections::VecDeque;

//This is like a match_entry but not supposed to generate an error
fn peek_match(input: &mut VecDeque<lexer::Entry>, t: lexer::Token) -> bool {
    if peek(input, t.clone()) {
        match_entry(input, t)
    } else {
        false
    }
}

fn peek(input: &mut VecDeque<lexer::Entry>, t: lexer::Token) -> bool {
    match input.get(0) {
        Some(v) => {
            if v.t == t {
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn match_entry(input: &mut VecDeque<lexer::Entry>, t: lexer::Token) -> bool {
    if peek(input, t.clone()) {
        match input.pop_front() {
            Some(e) => {
                input.push_back(e);
                true
            }
            None => false, //This should never happen due to the previous check
        }
    } else {
        println!(
            "Error tried to match {:?}, found {:?} lexeme: {}",
            t,
            match input.get(0) {
                Some(v) => v.t.clone(),
                None => lexer::Token::Error,
            },
            match input.get(0) {
                Some(v) => v.lexeme.clone(),
                None => "".to_string(),
            }
        );
        false
    }
}

fn entry(input: &mut VecDeque<lexer::Entry>) -> bool {
    if peek(input, lexer::Token::Open) {
        expression_list(input)
    } else if peek(input, lexer::Token::Pure) {
        list(input)
    } else {
        peek_match(input, lexer::Token::Number)
            || peek_match(input, lexer::Token::Id)
            || peek_match(input, lexer::Token::Float)
            || peek_match(input, lexer::Token::String)

    }
}

fn entry_list(input: &mut VecDeque<lexer::Entry>) -> bool {
    if peek(input, lexer::Token::Close) {
        true
    } else {
        entry(input) && entry_list(input)
    }
}

fn list(input: &mut VecDeque<lexer::Entry>) -> bool {
    match_entry(input, lexer::Token::Pure)
        && match_entry(input, lexer::Token::Open)
        && entry_list(input)
        && match_entry(input, lexer::Token::Close)
}

fn operator(input: &mut VecDeque<lexer::Entry>) -> bool {
    peek_match(input, lexer::Token::Plus)
        || peek_match(input, lexer::Token::Minus)
        || peek_match(input, lexer::Token::Equal)
        || peek_match(input, lexer::Token::Less)
        || peek_match(input, lexer::Token::Mult)
        || peek_match(input, lexer::Token::Div)
        || peek_match(input, lexer::Token::Not)
        || peek_match(input, lexer::Token::And)
        || peek_match(input, lexer::Token::Or)
}

fn func(input: &mut VecDeque<lexer::Entry>) -> bool {
    match input.get(0) {
        Some(e) => match e.lexeme.chars().nth(0) {
            Some(t) => {
                if lexer::valid_id_char(t) {
                    match_entry(input, lexer::Token::Id)
                } else {
                    operator(input)
                }
            }
            _ => operator(input),
        },
        _ => operator(input),
    }
}

fn evals(input: &mut VecDeque<lexer::Entry>) -> bool {
    match_entry(input, lexer::Token::Open)
    &&
    if peek_match(input, lexer::Token::Close) {
        true
    }
    else {
        func(input)
        && entry_list(input)
        && match_entry(input, lexer::Token::Close)
    }
}

fn expression_list(input: &mut VecDeque<lexer::Entry>) -> bool {
    expression(input) && {
        if peek(input, lexer::Token::Open) {
            expression_list(input)
        } else {
            true
        }
    }
}

fn expression(input: &mut VecDeque<lexer::Entry>) -> bool {
    //If it's eval
    if peek(input, lexer::Token::Open) {
        evals(input)
    }
    //If it's a pure list
    else {
        list(input)
    }
}

pub fn parse(input: &mut VecDeque<lexer::Entry>) -> bool {
    expression_list(input) && match_entry(input, lexer::Token::EOF)
}
