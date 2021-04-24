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
    pub map: HashMap<String, Box<Instruction>>,
}

impl InstructionSet {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn load(&mut self) {
        self.map.insert(
            String::from("INTEGER.+"),
            Instruction {
                execute: add_integers,
                code_blocks: 0,
            },
        );
    }
}

struct Instruction<F>
where
    F: FnMut(&mut PushState),
{
    pub execute: F,
    pub code_blocks: u32,
}

impl<F> Instruction<F>
where
    F: FnMut(&mut PushState),
{
    fn new(execute: F, code_blocks: u32) -> Self {
        Self {
            execute,
            code_blocks,
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
