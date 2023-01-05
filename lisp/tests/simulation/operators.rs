use super::utils;
use std::time::SystemTime;
use std::collections::VecDeque;


fn ints_to_input(input: &Vec<u32> ) -> String {

    let mut to_return : String = "".to_string();

    for i in input {
        to_return.push_str(i.to_string().as_str());
        to_return.push_str(" ");
    }
    to_return
}


fn dints_to_input(input: &VecDeque<u32> ) -> String {

    let mut to_return : String = "".to_string();

    for i in input {
        to_return.push_str(i.to_string().as_str());
        to_return.push_str(" ");
    }
    to_return
}



#[test]
fn addition() {

    let i = "( + 1 2 3 4 5 6 )".to_string();
    let res = utils::run_input(i);

    assert_eq!(res.literal, (1 + 2 + 3 + 4 + 5 + 6 ).to_string());

}

fn random_addition() {

    let mut input: Vec<u32> = Vec::new();
    let mut rand: utils::Fakerand = utils::Fakerand {value: 1};
    rand.seed(SystemTime::UNIX_EPOCH.elapsed().unwrap().as_millis());


    for _ in 0..100 {
        input.push(rand.get_uint() as u32);
    }
    let mut sum: i128 = 0;

    for i in &input {
        sum += *i as i128;
    }

    let mut to_operate: String = String::new();
    
    to_operate.push_str("( + ");
    to_operate.push_str(ints_to_input(&input).as_str());
    to_operate.push_str(" )");

    let res = utils::run_input(to_operate);
    
    assert_eq!(sum.to_string(), res.literal);

}

//Run a random addition 100 times just to see if something ends up wrong
#[test]
fn more_addition() {
    for _ in 0..100 {
        random_addition();
    }
}


fn random_subtraction() {

    let mut input: VecDeque<u32> = VecDeque::new();
    let mut rand: utils::Fakerand = utils::Fakerand {value: 1};
    rand.seed(SystemTime::UNIX_EPOCH.elapsed().unwrap().as_millis());


    for _ in 0..((rand.get_uint() % 100) + 1) {
        input.push_back(rand.get_uint() as u32);
    }
    let mut sum: i128 = input.pop_front().unwrap() as i128;

    let copy: u32 = sum as u32;

    for i in &input {
        sum -= *i as i128;
    }
    
    input.push_front(copy);

    let mut to_operate: String = String::new();
    
    to_operate.push_str("( - ");
    to_operate.push_str(dints_to_input(&input).as_str());
    to_operate.push_str(" )");

    let res = utils::run_input(to_operate);
    
    println!("{} {}",sum.to_string(), res.literal.clone());

    assert_eq!(sum.to_string(), res.literal);



}

#[test]
fn more_subtraction() {
    for _ in 0..100 {
        random_subtraction();
    }
}


fn random_division() {

    let mut input: VecDeque<u32> = VecDeque::new();
    let mut rand: utils::Fakerand = utils::Fakerand {value: 1};
    rand.seed(SystemTime::UNIX_EPOCH.elapsed().unwrap().as_millis());


    for _ in 0..((rand.get_uint() % 100) + 1) {
        input.push_back((rand.get_uint() as u32 % 10) + 1);
    }
    let mut sum: i128 = rand.get_uint() as i128;

    let copy: u32 = sum as u32;

    for i in &input {
        sum /= *i as i128;
    }
    
    input.push_front(copy);

    let mut to_operate: String = String::new();
    
    to_operate.push_str("( / ");
    to_operate.push_str(dints_to_input(&input).as_str());
    to_operate.push_str(" )");

    let res = utils::run_input(to_operate);
    
    println!("{} {}",sum.to_string(), res.literal.clone());

    assert_eq!(sum.to_string(), res.literal);

}

#[test]
fn more_div() {

    for _ in 0..100 {
        random_division();
    }
}


fn random_mult() {

    let mut input: VecDeque<u32> = VecDeque::new();
    let mut rand: utils::Fakerand = utils::Fakerand {value: 1};
    rand.seed(SystemTime::UNIX_EPOCH.elapsed().unwrap().as_millis());


    for _ in 0..((rand.get_uint() % 25) + 1) {
        input.push_back((rand.get_uint() as u32 % 10) + 1);
    }
    let mut sum: i128 = input.pop_front().unwrap() as i128;

    let copy: u32 = sum as u32;

    for i in &input {
        sum *= *i as i128;
    }
    
    input.push_front(copy);

    let mut to_operate: String = String::new();
    
    to_operate.push_str("( * ");
    to_operate.push_str(dints_to_input(&input).as_str());
    to_operate.push_str(" )");

    let res = utils::run_input(to_operate);
    
    println!("{} {}",sum.to_string(), res.literal.clone());

    assert_eq!(sum.to_string(), res.literal);

}

#[test]
fn more_mult() {
    for _ in 0..100 {
        random_mult();
    }
}



