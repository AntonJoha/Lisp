pub mod utils;
mod operators;
mod lists;
mod files;

//Test a simple addition
#[test]
fn sum_test() {

    let a = utils::run_input("( + 1 1 ) ".to_string());
    assert_eq!(a.literal,"2");

}

//Test the most simple of functions
#[test]
fn func_test() {

    let input = vec!("( def a ( b c ) ( + b c ) )".to_string(), "( a 1 1 )".to_string());

    let a = utils::run_multiple(input);
    assert_eq!(a.literal,"2");

}


#[test]
fn basic_file() {
    let v = utils::test_file("tests/basic.lisp".to_string());

    assert_eq!(v.literal, "3");
}


//Test a simple if case
#[test]
fn if_test() {

    let input = vec!("( def a ( b c ) ( + b c ) )".to_string(),
        "( if ( == ( a 2 3 ) 5 ) 2 0 ) ".to_string()
        );

    let a = utils::run_multiple(input);
    assert_eq!(a.literal,"2");

}






