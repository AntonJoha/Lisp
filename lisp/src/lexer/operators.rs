use super::super::lexer;

fn plus(first: lexer::Token, second: lexer::Token) -> lexer::Token {
    match (first, second) {
        (lexer::Token::Number, lexer::Token::Number) => lexer::Token::Number,
        (lexer::Token::String, lexer::Token::Number) => lexer::Token::String,
        (lexer::Token::Char, lexer::Token::Number) => lexer::Token::Number,
        (lexer::Token::Char, lexer::Token::Char) => lexer::Token::Number,
        (lexer::Token::Float, lexer::Token::Float) => lexer::Token::Float,
        (lexer::Token::Char, lexer::Token::String) => lexer::Token::String,
        (lexer::Token::Float, lexer::Token::Number) => lexer::Token::Float,
        (lexer::Token::Char, lexer::Token::Float) => lexer::Token::Float,
        (lexer::Token::Float, lexer::Token::String) => lexer::Token::String,
        (lexer::Token::String, lexer::Token::String) => lexer::Token::String,
        (lexer::Token::Open, lexer::Token::Open) => lexer::Token::Open,
        _ => lexer::Token::Error,
    }
}

fn minus(first: lexer::Token, second: lexer::Token) -> lexer::Token {
    match (first, second) {
        (lexer::Token::Number, lexer::Token::Number) => lexer::Token::Number,
        (lexer::Token::Float, lexer::Token::Number) => lexer::Token::Float,
        (lexer::Token::Char, lexer::Token::Number) => lexer::Token::Number,
        (lexer::Token::Float, lexer::Token::Float) => lexer::Token::Float,
        (lexer::Token::Float, lexer::Token::Char) => lexer::Token::Float,
        (lexer::Token::Char, lexer::Token::Char) => lexer::Token::Number,
        _ => lexer::Token::Error,
    }
}

fn mult(first: lexer::Token, second: lexer::Token) -> lexer::Token {
    match (first, second) {
        (lexer::Token::Number, lexer::Token::Number) => lexer::Token::Number,
        (lexer::Token::Float, lexer::Token::Number) => lexer::Token::Float,
        (lexer::Token::Char, lexer::Token::Number) => lexer::Token::String,
        (lexer::Token::Float, lexer::Token::Float) => lexer::Token::Float,
        (lexer::Token::Char, lexer::Token::Char) => lexer::Token::Number,
        (lexer::Token::String, lexer::Token::Number) => lexer::Token::String,
        _ => lexer::Token::Error,
    }
}

fn div(first: lexer::Token, second: lexer::Token) -> lexer::Token {
    match (first, second) {
        (lexer::Token::Number, lexer::Token::Number) => lexer::Token::Number,
        (lexer::Token::Float, lexer::Token::Float) => lexer::Token::Float,
        (lexer::Token::Float, lexer::Token::Number) => lexer::Token::Float,
        _ => lexer::Token::Error,
    }
}

fn find_operator(
    operator: lexer::Token,
    first: lexer::Token,
    second: lexer::Token,
) -> lexer::Token {
    match operator {
        lexer::Token::Plus => plus(first, second),
        lexer::Token::Minus => minus(first, second),
        lexer::Token::Mult => mult(first, second),
        lexer::Token::Div => div(first, second),
        _ => lexer::Token::Error,
    }
}

pub fn operator_result(
    operator: lexer::Token,
    first: lexer::Token,
    second: lexer::Token,
) -> lexer::Token {
    match find_operator(operator.clone(), first.clone(), second.clone()) {
        lexer::Token::Error => find_operator(operator.clone(), second.clone(), first.clone()),
        t => t,
    }
}
