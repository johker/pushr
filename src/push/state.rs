#![feature(map_into_keys_values)]

use crate::push::configuration::PushConfiguration;
use crate::push::item::Item;
use crate::push::stack::PushStack;
use std::collections::HashMap;

pub struct PushState<'a> {
    pub float_stack: PushStack<f32>,
    pub exec_stack: PushStack<Item<'a>>,
    pub code_stack: PushStack<Item<'a>>,
    pub int_stack: PushStack<i32>,
    pub bool_stack: PushStack<bool>,
    pub name_stack: PushStack<&'a str>,
    pub name_bindings: HashMap<&'a str, Item<'a>>,
    pub configuration: PushConfiguration,
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
            configuration: PushConfiguration::new(),
        }
    }
}
