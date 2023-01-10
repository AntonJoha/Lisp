use lisp;
use lisp::lexer;
use lisp::parser;

fn test(input: String, expected: bool) {
    let mut r = lexer::lexer(input).unwrap();
    let res = parser::parse(&mut r);

    println!("\n{}\n", res);

    assert_eq!(expected, res);
}

#[test]
fn basic() {
    let input = "( a ) ( + d ( d aawd 2.33 ))".to_string();
    test(input, true)
}

#[test]
fn error() {
    let input = "( a ) ( + d ( d aawd 2.33 )".to_string();
    test(input, false);
}

#[test]
fn empty() {
    let input = "( )".to_string();
    test(input, true);
}

#[test]
fn list() {
    let input = "(hello a '( why l ) ( && hello 2.4 676576575 ) ) ( < '( d ) )".to_string();
    test(input, true);
}

#[test]
fn string_test() {
    let input = "( + \" oad pmrmp3 345 335 \" )".to_string();
    test(input, true);
}

#[test]
fn wrong_op_place() {
    let input = "( + + )".to_string();
    test(input, false);
}
