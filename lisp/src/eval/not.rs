use std::collections::VecDeque;

use super::super::eval;
use super::super::lexer;

pub fn not(arguments: VecDeque<eval::Value>) -> eval::Value {
    let mut to_return = eval::equal::equal(arguments);

    if to_return.literal == "1".to_string() {
        to_return.literal = "0".to_string();
    } else {
        to_return.literal = "1".to_string();
    }
    to_return
}
