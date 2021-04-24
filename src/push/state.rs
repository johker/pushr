use crate::push::atoms::Atom;
use crate::push::stack::PushStack;

pub struct PushState {
    pub float_stack: PushStack<f32>,
    pub exec_stack: PushStack<Atom>,
    pub code_stack: PushStack<Atom>,
    pub int_stack: PushStack<i32>,
    pub bool_stack: PushStack<bool>,
}

impl PushState {
    pub fn new() -> Self {
        Self {
            float_stack: PushStack::new(),
            exec_stack: PushStack::new(),
            code_stack: PushStack::new(),
            int_stack: PushStack::new(),
            bool_stack: PushStack::new(),
        }
    }
}
