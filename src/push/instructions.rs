use crate::push::state::PushState;
use std::collections::HashMap;

use crate::push::boolean::*;
use crate::push::code::*;

// Instructions
//
//
// Instruction as trait (Abstract class)
//
// Each instrcution is a struct
// Instruction Set is a hashmap with string key and struct as value

pub struct InstructionSet {
    pub map: HashMap<String, Instruction>,
}

impl InstructionSet {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn load(&mut self) {
        self.map
            .insert(String::from("NOOP"), Instruction::new(noop));
        load_boolean_instructions(&mut self.map);
        load_code_instructions(&mut self.map);
        self.map
            .insert(String::from("INTEGER.+"), Instruction::new(integer_add));
        self.map.insert(
            String::from("INTEGER.*"),
            Instruction::new(integer_multiply),
        );
        self.map
            .insert(String::from("FLOAT.+"), Instruction::new(float_add));
    }

    pub fn cache(&self) -> InstructionCache {
        InstructionCache::new(self.map.keys().cloned().collect())
    }
}

pub struct InstructionCache {
    pub list: Vec<String>,
}

impl InstructionCache {
    pub fn new(arg_list: Vec<String>) -> Self {
        Self { list: arg_list }
    }
}

pub struct Instruction {
    pub execute: Box<dyn FnMut(&mut PushState, &InstructionCache)>,
}

impl Instruction {
    pub fn new(execute: impl FnMut(&mut PushState, &InstructionCache) + 'static) -> Self {
        Self {
            execute: Box::new(execute),
        }
    }
}

//
// ------------------ Type: INTEGER ---------------------
//

fn noop(_push_state: &mut PushState, _instruction_cache: &InstructionCache) {}

fn integer_add(push_state: &mut PushState, _instruction_set: &InstructionCache) {
    if let Some(pv) = push_state.int_stack.pop_vec(2) {
        push_state.int_stack.push(pv[0] + pv[1]);
    }
}

fn integer_multiply(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(pv) = push_state.int_stack.pop_vec(2) {
        push_state.int_stack.push(pv[0] * pv[1]);
    }
}

//
// ------------------ Type: FLOAT ---------------------
//

fn float_add(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(pv) = push_state.float_stack.pop_vec(2) {
        push_state.float_stack.push(pv[0] + pv[1]);
    }
}
