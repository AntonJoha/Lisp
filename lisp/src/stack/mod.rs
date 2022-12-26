use std::collections::HashMap;
use std::collections::VecDeque;

use super::eval;

struct Functions{


}


pub struct StackFrame {
    pub values: HashMap<eval::Value>,
    pub func: HashMap<Functions>,
}

pub struct Stack {
    pub frames: VecDeque<StackFrame>,
}

impl Stack {
    
    fn make_frame(&mut self) {
        frames.push_front(StackFrame { values: HashMap::new() } );
    }
    

}

