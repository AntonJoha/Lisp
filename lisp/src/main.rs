mod lexer;
mod parser;
mod eval;
use std::io;
use std::io::Write;

fn get_input() -> String {
    let stdin = io::stdin();
    let mut s = String::new();
    stdin.read_line(&mut s).unwrap();
    s
}


fn print(){
    
    print!("lisp-cli>");
    io::stdout().flush().unwrap();
}

fn main() {
    loop {
        
        print();
        let s = get_input();
        let mut a = match lexer::lexer(s) {
            Ok(e) => e, 
            _ => {println!("Weird input"); continue;}
        };
        
        if !parser::parse(&mut a) {
            println!("Parse failed"); continue;
        }
        eval::evaluate(a);
    }
}
