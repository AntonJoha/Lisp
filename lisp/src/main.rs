mod eval;
mod stack;
mod lexer;
mod parser;
use std::io;
use std::io::Write;
use std::collections::VecDeque;

fn get_input() -> String {
    let stdin = io::stdin();
    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    s
}

fn print() {
    print!("lisp-cli>");
    io::stdout().flush().unwrap();
}

fn main() {


    //Initial stackframe
    let mut stack: stack::Stack = stack::Stack{frames: VecDeque::new()};
    stack.make_frame();


    loop {
        print();
        let s = get_input();
        let mut a = match lexer::lexer(s) {
            Ok(e) => e,
            _ => {
                println!("Weird input");
                continue;
            }
        };

        if !parser::parse(&mut a) {
            println!("Parse failed");
            continue;
        }
        eval::evaluate(a, &mut stack);
    }
}
