use crate::push::atoms::Atom;
use crate::push::stack::PushStack;
use std::collections::HashMap;

pub struct PushState<'a> {
    pub float_stack: PushStack<f32>,
    pub exec_stack: PushStack<Atom<'a>>,
    pub code_stack: PushStack<Atom<'a>>,
    pub int_stack: PushStack<i32>,
    pub bool_stack: PushStack<bool>,
    pub name_stack: PushStack<&'a str>,
    pub name_bindings: HashMap<&'a str, Atom<'a>>,
}

impl<'a> PushState<'a> {
    pub fn new() -> Self {
        Self {
            float_stack: PushStack::new(),
            exec_stack: PushStack::new(),
            code_stack: PushStack::new(),
            int_stack: PushStack::new(),
            bool_stack: PushStack::new(),
            name_stack: PushStack::new(),
            name_bindings: HashMap::new(),
        }
    }
}
