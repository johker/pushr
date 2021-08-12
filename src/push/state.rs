use crate::push::configuration::PushConfiguration;
use crate::push::item::Item;
use crate::push::stack::PushStack;
use crate::push::vector::{BoolVector, FloatVector, IntVector};
use std::collections::HashMap;
use std::fmt;

pub struct PushState {
    // Scalar Types
    pub bool_stack: PushStack<bool>,
    pub code_stack: PushStack<Item>,
    pub exec_stack: PushStack<Item>,
    pub float_stack: PushStack<f32>,
    pub int_stack: PushStack<i32>,
    pub name_stack: PushStack<String>,

    // Vector Types
    pub bool_vector_stack: PushStack<BoolVector>,
    pub float_vector_stack: PushStack<FloatVector>,
    pub int_vector_stack: PushStack<IntVector>,

    pub name_bindings: HashMap<String, Item>,
    pub configuration: PushConfiguration,
    pub quote_name: bool,
}

impl PushState {
    pub fn new() -> Self {
        Self {
            float_stack: PushStack::new(),
            exec_stack: PushStack::new(),
            code_stack: PushStack::new(),
            int_stack: PushStack::new(),
            bool_stack: PushStack::new(),
            name_stack: PushStack::new(),
            bool_vector_stack: PushStack::new(),
            float_vector_stack: PushStack::new(),
            int_vector_stack: PushStack::new(),
            name_bindings: HashMap::new(),
            configuration: PushConfiguration::new(),
            quote_name: false,
        }
    }
}

impl fmt::Display for PushState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "> BOOL  : {}\n> CODE  : {}\n> EXEC  : {}\n> FLOAT : {}\n> INT   : {}\n> NAME  : {}\n",
            self.bool_stack.to_string(),
            self.code_stack.to_string(),
            self.exec_stack.to_string(),
            self.float_stack.to_string(),
            self.int_stack.to_string(),
            self.name_stack.to_string()
        )
    }
}
