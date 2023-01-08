use super::utils;
use std::collections::VecDeque;


#[test]
fn first() {

    let res = utils::run_input("( first '( 1 2 3 ) )".to_string());
    assert_eq!(res.literal,"1");
}


#[test]
fn get() {

    let res = utils::run_input("( get 2 '( 1 2 3 ) )".to_string());
    assert_eq!(res.literal,"3");
}

#[test]
fn rest() {

    let res = utils::run_input("( rest '( 1 2 3 ) )".to_string());
    assert_eq!(res.list.get(0).unwrap().literal, "2");
}


#[test]
fn list() {

    let res = utils::run_input("( list 1 2 3 )".to_string());
    assert_eq!(res.list.get(0).unwrap().literal, "1");
}



#[test]
fn complex_rest() {

    let res = utils::run_input("(rest ( list 1 2 3 ))".to_string());
    assert_eq!(res.list.get(0).unwrap().literal, "2");
}


