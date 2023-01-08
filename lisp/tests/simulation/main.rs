pub mod utils;
mod operators;
mod lists;

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



//Test a simple if case
#[test]
fn if_test() {

    let input = vec!("( def a ( b c ) ( + b c ) )".to_string(),
        "( if ( == ( a 2 3 ) 5 ) 2 0 ) ".to_string()
        );

    let a = utils::run_multiple(input);
    assert_eq!(a.literal,"2");

}






