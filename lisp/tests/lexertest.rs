
use lisp;

#[test]
fn it_work(){
    let mut resutlt = lisp::lexer::lexer("input".to_string()).expect("Lexer function");
    
    assert_eq!(resutlt.pop_front().unwrap().lexeme, "input");

}


