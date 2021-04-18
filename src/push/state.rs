use crate::push::stack::PushStack;

pub struct PushState {
    float_stack: PushStack<f32>,
    exec_stack: PushStack<String>,
    code_stack: PushStack<String>,
    int_stack: PushStack<i32>,
    bool_stack: PushStack<bool>,
}
