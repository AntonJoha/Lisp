use super::lexer;
use super::super::eval;
use std::collections::VecDeque;


fn place_holder() -> eval::Value {
    eval::get_error()
}

fn mult(arguments: VecDeque<eval::Value>) -> eval::Value {
    todo!();
    place_holder()
}


fn minus( arguments: VecDeque<eval::Value>) -> eval::Value {
    todo!();
    place_holder()
}


fn equal(arguments: VecDeque<eval::Value>) -> eval::Value {
    todo!();
    place_holder()
}


fn less(arguments: VecDeque<eval::Value>) -> eval::Value {
    todo!();
    place_holder();
}


fn div(arguments: VecDeque<eval::Value>) -> eval::Value {
    todo!();
    place_holder()
}


fn not(arguments: VecDeque<eval::Value>) -> eval::Value {
    todo!();
    place_holder()
}


fn and(arguments: VecDeque<eval::Value>) -> eval::Value {
    todo!();
    place_holder()
}


fn or(arguments: VecDeque<eval::Value>) -> eval::Value {
    todo!();
    place_holder()
}



pub fn operator_eval(fun: lexer::Entry, arguments: VecDeque<eval::Value>) -> eval::Value {
    match fun.t.clone() {
        lexer::Token::Plus => {
            eval::plus::plus(arguments)
        },
        lexer::Token::Mult => {
            mult(arguments)
        },
        lexer::Token::Minus => {
            minus(arguments)
        },
        lexer::Token::Equal => {
            equal(arguments)
        },
        lexer::Token::Less => {
            less(arguments)
        },
        lexer::Token::Div => {
            div(arguments)
        },
        lexer::Token::Not => {
            not(arguments)
        },
        lexer::Token::And => {
            and(arguments)
        },
        lexer::Token::Or => {
            or(arguments)
        }
        _ => {
            panic!("Should not be here operator_eval");
        }
    }
}
