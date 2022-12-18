use std::collections::VecDeque;

use super::super::eval;
use super::super::lexer;


fn false_val() -> eval::Value {
    eval::Value {
        literal: "0".to_string(),
        t: lexer::Token::Number,
        list: VecDeque::new(),
    }
}


fn true_val() -> eval::Value {
    eval::Value {
        literal: "1".to_string(),
        t: lexer::Token::Number,
        list: VecDeque::new(),
    }
}



pub fn and(mut arguments: VecDeque<eval::Value> ) -> eval::Value {

    
    for i in arguments {
        if i.literal == "0".to_string() {
            return false_val();
        }
    }
    return true_val();
}

