use std::collections::VecDeque;
mod operators;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
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
    Char,
    Number,
    Float,
    Error,
    Pure,
    EOF,
}

pub struct Entry {
    pub lexeme: String,
    pub t: Token,
}

pub fn operator(op: Token, right: Token, left: Token) -> Token {
    operators::operator_result(op, right, left)
}

//Valid character in id as specified in the documentation
pub fn valid_id_char(c: char) -> bool {
    (c >= 65 as char && c <= 90 as char)
        || (c >= 97 as char && c <= 122 as char)
        || (valid_number(c))
        || (c == 95 as char)
}

fn valid_number(c: char) -> bool {
    c >= 48 as char && c <= 57 as char
}

fn entry_id(string: String) -> Entry {
    Entry {
        lexeme: string,
        t: Token::Id,
    }
}

fn handle_equal(chars: &mut VecDeque<char>) -> Entry {
    if chars.len() != 0 {
        match chars.pop_front().unwrap() {
            '=' => {
                return Entry {
                    lexeme: "==".to_string(),
                    t: Token::Equal,
                }
            }
            e => chars.push_front(e),
        };
    }
    entry_id("=".to_string())
}
fn handle_and(chars: &mut VecDeque<char>) -> Entry {
    if chars.len() != 0 {
        match chars.pop_front().unwrap() {
            '&' => {
                return Entry {
                    lexeme: "&&".to_string(),
                    t: Token::And,
                }
            }
            e => chars.push_front(e),
        };
    }
    entry_id("&".to_string())
}

fn handle_or(chars: &mut VecDeque<char>) -> Entry {
    if chars.len() != 0 {
        match chars.pop_front().unwrap() {
            '|' => {
                return Entry {
                    lexeme: "||".to_string(),
                    t: Token::Or,
                }
            }
            e => chars.push_front(e),
        };
    }

    entry_id("|".to_string())
}

fn construct_float(chars: &mut VecDeque<char>, mut curr: String) -> Entry {
    if chars.len() > 0 {
        let c = chars.get(0).unwrap();
        if valid_number(*c) {
            curr.push(chars.pop_front().unwrap());
            return construct_float(chars, curr);
        }
    }
    return Entry {
        lexeme: curr,
        t: Token::Float,
    };
}

//Construct either a number or a float
fn construct_number(chars: &mut VecDeque<char>, mut curr: String) -> Entry {
    if chars.len() > 0 {
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
    return Entry {
        lexeme: curr,
        t: Token::Number,
    };
}

fn construct_id(chars: &mut VecDeque<char>, mut curr: String) -> Entry {
    if chars.len() > 0 {
        let c = chars.get(0).unwrap();
        if valid_id_char(*c) {
            curr.push(chars.pop_front().unwrap());
            return construct_id(chars, curr);
        }
    }
    return Entry {
        lexeme: curr,
        t: Token::Id,
    };
}

fn handle_string(chars: &mut VecDeque<char>) -> Entry {
    let mut s: String = String::new();

    while chars.len() > 0 {
        match chars.pop_front() {
            Some(c) => {
                if c == '"' {
                    return Entry {
                        lexeme: s,
                        t: Token::String,
                    };
                }
                if c == '\\' {
                    match chars.pop_front() {
                        Some(e) => s.push(e),
                        None => (),
                    }
                } else {
                    s.push(c);
                }
            }
            None => (),
        }
    }
    return Entry {
        lexeme: s,
        t: Token::String,
    };
}

fn handle_rest(chars: &mut VecDeque<char>, curr: char) -> Entry {
    if valid_number(curr) {
        return construct_number(chars, curr.to_string());
    }
    if valid_id_char(curr) {
        return construct_id(chars, curr.to_string());
    }
    Entry {
        lexeme: curr.to_string(),
        t: Token::Error,
    }
}

pub fn lexer(input: String) -> Result<VecDeque<Entry>, String> {
    let mut to_return: VecDeque<Entry> = VecDeque::new();

    let mut chars: VecDeque<char> = input.chars().collect();

    while chars.len() > 0 {
        match chars.pop_front().unwrap() {
            '+' => to_return.push_back(Entry {
                lexeme: "+".to_string(),
                t: Token::Plus,
            }),
            '-' => to_return.push_back(Entry {
                lexeme: "-".to_string(),
                t: Token::Minus,
            }),
            '*' => to_return.push_back(Entry {
                lexeme: "*".to_string(),
                t: Token::Mult,
            }),
            '/' => to_return.push_back(Entry {
                lexeme: "/".to_string(),
                t: Token::Div,
            }),
            '=' => to_return.push_back(handle_equal(&mut chars)),
            '<' => to_return.push_back(Entry {
                lexeme: "<".to_string(),
                t: Token::Less,
            }),
            '!' => to_return.push_back(Entry {
                lexeme: "!".to_string(),
                t: Token::Not,
            }),
            '&' => to_return.push_back(handle_and(&mut chars)),
            '|' => to_return.push_back(handle_or(&mut chars)),
            ' ' | '\n' | '\t' | '\r' => (),
            '(' => to_return.push_back(Entry {
                lexeme: "(".to_string(),
                t: Token::Open,
            }),
            ')' => to_return.push_back(Entry {
                lexeme: ")".to_string(),
                t: Token::Close,
            }),
            '\'' => to_return.push_back(Entry {
                lexeme: "'".to_string(),
                t: Token::Pure,
            }),
            '"' => to_return.push_back(handle_string(&mut chars)),
            e => to_return.push_back(handle_rest(&mut chars, e)),
        };
    }
    to_return.push_back(Entry {
        lexeme: "".to_string(),
        t: Token::EOF,
    });
    return Ok(to_return);
}
