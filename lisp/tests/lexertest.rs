
use std::collections::VecDeque;
use lisp;
use lisp::lexer;

fn correct(e: lexer::Entry, s: String, t: lexer::Token){
    assert_eq!(e.lexeme, s);
    assert_eq!(e.t, t);
}

fn compare_more(input: String, mut lexeme: Vec<String>, mut types: Vec<lexer::Token>){
    let mut result = lexer::lexer(input).expect("Lexer function");

    let mut lexeme = VecDeque::from_iter(lexeme);
    let mut types = VecDeque::from_iter(types);

    while result.len() > 0 {
        correct(result.pop_front().unwrap(), 
                lexeme.pop_front().unwrap(),
                types.pop_front().unwrap());
    }
}


//Should be an id
#[test]
fn singular_id(){
    let mut resutlt = lexer::lexer("input".to_string()).expect("Lexer function");
    assert_eq!(resutlt.pop_front().unwrap().t, lexer::Token::Id);
}


//Should be three id:s
#[test]
fn multiple_id(){
    let result = lexer::lexer("input dawdawd awdwwa "
                                  .to_string()).expect("Lexer function");

    assert_eq!(result.len(), 3);
    for i in result {
        assert_eq!(i.t, lexer::Token::Id);
    }

}


//Example to handle a basic case of addition
#[test]
fn addition(){
    let mut result = lexer::lexer("(+ 1 2)"
                              .to_string()).expect("Lexer function");
    let e = result.pop_front().unwrap();
    correct(e, "(".to_string(), lexer::Token::Open);
    let e = result.pop_front().unwrap();
    correct(e, "+".to_string(), lexer::Token::Plus);
    let e = result.pop_front().unwrap();
    correct(e, "1".to_string(), lexer::Token::Number);
    let e = result.pop_front().unwrap();
    correct(e, "2".to_string(), lexer::Token::Number);
    let e = result.pop_front().unwrap();
    correct(e, ")".to_string(), lexer::Token::Close)
}

#[test]
fn subtraction() {

    let input = "(- 2 1)".to_string();
    let lexeme = vec!("(".to_string(),
                        "-".to_string(),
                        "2".to_string(),
                        "1".to_string(),
                        ")".to_string());
    let t = vec!(lexer::Token::Open,
                 lexer::Token::Minus,
                 lexer::Token::Number,
                 lexer::Token::Number,
                 lexer::Token::Close);
    compare_more(input, lexeme, t);
}

#[test]
fn float(){
    let input = "(- 2.342343241 1)".to_string();
    let lexeme = vec!("(".to_string(),
                        "-".to_string(),
                        "2.342343241".to_string(),
                        "1".to_string(),
                        ")".to_string());
    let t = vec!(lexer::Token::Open,
                 lexer::Token::Minus,
                 lexer::Token::Float,
                 lexer::Token::Number,
                 lexer::Token::Close);
    compare_more(input, lexeme, t);
}

#[test]
fn list_test(){
    let input = "(+ (+ hello 1) (+ 1 1.2))".to_string();
    let lexeme = vec!("(".to_string(),
                    "+".to_string(),
                    "(".to_string(),
                    "+".to_string(),
                    "hello".to_string(),
                    "1".to_string(),
                    ")".to_string(),
                    "(".to_string(),
                    "+".to_string(),
                    "1".to_string(),
                    "1.2".to_string(),
                    ")".to_string(),
                    ")".to_string());
    let t = vec!(lexer::Token::Open,
                 lexer::Token::Plus,
                 lexer::Token::Open,
                 lexer::Token::Plus,
                 lexer::Token::Id,
                 lexer::Token::Number,
                 lexer::Token::Close,
                 lexer::Token::Open,
                 lexer::Token::Plus,
                 lexer::Token::Number,
                 lexer::Token::Float,
                 lexer::Token::Close,
                 lexer::Token::Close);
    compare_more(input, lexeme,t);

}
