mod eval;
mod stack;
mod lexer;
mod parser;
use std::env;
use std::io;
use std::io::Write;
use std::collections::VecDeque;
use std::fs;

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


fn repl() {

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
        
        eval::print_value(eval::evaluate(a, &mut stack));
    }
}

fn run_file(mut args: Vec<String> ) {

    let filename = args.pop().unwrap();
    
    println!("Filename {}", filename.clone());

    let content = fs::read_to_string(filename)
        .expect("Could not read file");

        let mut a = match lexer::lexer(content) {
            Ok(e) => e,
            _ => {
                println!("Weird input");
                return;
            }
        };

        if !parser::parse(&mut a) {
            println!("Parse failed");
        }


    //Initial stackframe
    let mut stack: stack::Stack = stack::Stack{frames: VecDeque::new()};
    stack.make_frame();

    eval::evaluate(a, &mut stack);

}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        repl();
    }
    else {
        run_file(args);
    }

}
