mod test;

pub enum Token{
    Plus,
    Minus,
    Equal,
    Less,
    Mult,
    Not,
    And,
    Or,
    Open,
    Close,
    Id,
    String,
    Number,
}

pub struct Entry{
    lexeme : String,
    t: Token,
}


pub fn lexer(input: String) -> Result<Vec<Entry>, String> {
    

    return Err("Not done".to_string());
}

