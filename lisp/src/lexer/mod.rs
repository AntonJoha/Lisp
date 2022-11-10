use std::collections::VecDeque;

pub enum Token{
    Plus,
    Minus,
    Equal,
    Less,
    Mult,
    Div,
    Not,
    And,
    Or,
    Open,
    Close,
    Id,
    String,
    Number,
    Float,
    Error,
}

pub struct Entry{
    pub lexeme : String,
    pub t: Token,
}


//Valid character in id as specified in the documentation
fn valid_id_char(c: char) -> bool {
    (c >= 65 as char && c <= 90 as char )||
        ( c >= 97 as char && c <= 122 as char) ||
        ( valid_number(c) ) ||
        ( c == 95 as char)
}

fn valid_number(c: char) -> bool {
     c >= 48 as char && c <= 57 as char
}

fn entry_id(string: String) -> Entry {
    Entry { lexeme: string,
        t: Token::Id}
}


fn handle_equal(chars: &mut VecDeque<char>) -> Entry {

    if chars.len() != 0 {
        match chars.pop_front().unwrap() {
            '=' => return Entry{ lexeme: "==".to_string(), t: Token::Equal},
            e => chars.push_front(e),
        };
    }
    entry_id("=".to_string())
}
fn handle_and(chars: &mut VecDeque<char>) -> Entry {

    if chars.len() != 0{
        match chars.pop_front().unwrap() {
            '&' => return Entry{ lexeme: "&&".to_string(), t: Token::And},
            e => chars.push_front(e),
        };
    }
    entry_id("&".to_string())
}

fn handle_or(chars: &mut VecDeque<char>) -> Entry {
    
    if chars.len() != 0{
        match chars.pop_front().unwrap() {
            '|' => return Entry{ lexeme: "||".to_string(), t: Token::Or },
            e => chars.push_front(e),
        };
    }

    entry_id("|".to_string())
}

fn construct_float(chars: &mut VecDeque<char>, mut curr: String) -> Entry{
    if chars.len() > 0 {
        let c = chars.get(0).unwrap();
        if valid_number(*c) {
            curr.push(chars.pop_front().unwrap());
            return construct_float(chars, curr);
        }
    }
    return Entry {lexeme: curr, t: Token::Float};
}

//Construct either a number or a float
fn construct_number(chars: &mut VecDeque<char>,mut curr: String) -> Entry {
    if chars.len() > 0{
        let c = chars.get(0).unwrap();
        if valid_number(*c) {
            curr.push(chars.pop_front().unwrap());
            return construct_number(chars, curr);
        }
        if *c == '.' {
            curr.push(chars.pop_front().unwrap());
            return construct_float(chars, curr);
        }
    }
    return Entry { lexeme: curr, t: Token::Number};
}

fn construct_id(chars: &mut VecDeque<char>,mut curr: String) -> Entry {
   if chars.len() > 0 {
        let c = chars.get(0).unwrap();
        if valid_id_char(*c) {
            curr.push(chars.pop_front().unwrap());
            return construct_id(chars, curr);
        }
    }
    return Entry {lexeme: curr, t: Token::Id}
}

fn handle_rest(chars: &mut VecDeque<char>, curr: char) -> Entry {
    if valid_number(curr) {
        return construct_number(chars, curr.to_string());
    }
    if valid_id_char(curr) {
        return construct_id(chars, curr.to_string());
    }
    Entry {lexeme: curr.to_string(), t: Token::Error}
}

pub fn lexer(input: String) -> Result<VecDeque<Entry>, String> {

    let mut to_return: VecDeque<Entry> = VecDeque::new();

    let mut chars: VecDeque<char> = input.chars().collect();

    while chars.len() > 0 {
        
        match chars.pop_front().unwrap(){
            '+' => to_return.push_front(Entry {lexeme: "+".to_string(), t: Token::Plus}),
            '-' => to_return.push_front(Entry {lexeme: "-".to_string(), t: Token::Minus}),
            '*' => to_return.push_front(Entry {lexeme: "*".to_string(), t: Token::Mult}),
            '/' => to_return.push_front(Entry {lexeme: "/".to_string(), t: Token::Div}),
            '=' => to_return.push_front(handle_equal(&mut chars)),
            '<' => to_return.push_front(Entry {lexeme: "<".to_string(), t: Token::Less}),
            '!' => to_return.push_front(Entry {lexeme: "!".to_string(), t: Token::Not}),
            '&' => to_return.push_front(handle_and(&mut chars)),
            '|' => to_return.push_front(handle_or(&mut chars)),
            ' ' | '\n' | '\t' | '\r' => (),
            '(' => to_return.push_front(Entry {lexeme: "(".to_string(), t: Token::Open}),
            ')' => to_return.push_front(Entry {lexeme: ")".to_string(), t: Token::Close}),
            e => to_return.push_front(handle_rest(&mut chars, e)),
        };

    }
 
    return Ok(to_return);
}

