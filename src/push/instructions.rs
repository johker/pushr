use crate::push::state::PushState;
use std::collections::HashMap;

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
            .insert(String::from("INTEGER.+"), Instruction::new(add_integers, 0));
    }
}

pub struct Instruction {
    pub execute: Box<dyn FnMut(&mut PushState)>,
    pub code_blocks: u32,
}

impl Instruction {
    fn new(execute: impl FnMut(&mut PushState) + 'static, code_blocks: u32) -> Self {
        Self {
            execute: Box::new(execute),
            code_blocks: code_blocks,
        }
    }
}

fn add_integers(push_state: &mut PushState) {
    match push_state.int_stack.pop_vec(2) {
        None => return,
        Some(pv) => push_state.int_stack.push(pv[0] + pv[1]),
    }
}

//pub trait Instruction {
//    fn evaluate(&self) -> String;
//}
