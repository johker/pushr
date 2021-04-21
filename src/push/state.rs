use crate::push::stack::PushStack;
use std::collections::HashMap;

pub struct PushState {
    pub float_stack: PushStack<f32>,
    pub exec_stack: PushStack<String>,
    pub code_stack: PushStack<String>,
    pub int_stack: PushStack<i32>,
    pub bool_stack: PushStack<bool>,
}

impl PushState {
    fn observe_int_stack(&self) -> i32 {
        0
    }
}
