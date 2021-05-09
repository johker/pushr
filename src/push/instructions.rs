use crate::push::state::PushState;
use std::collections::HashMap;

use crate::push::boolean::*;

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
            .insert(String::from("BOOLEAN.="), Instruction::new(boolean_eq, 0));
        self.map.insert(
            String::from("BOOLEAN.AND"),
            Instruction::new(boolean_and, 0),
        );
        self.map.insert(
            String::from("BOOLEAN.DEFINE"),
            Instruction::new(boolean_def, 0),
        );
        self.map.insert(
            String::from("BOOLEAN.DUP"),
            Instruction::new(boolean_dup, 0),
        );
        self.map.insert(
            String::from("BOOLEAN.FLUSH"),
            Instruction::new(boolean_flush, 0),
        );
        self.map.insert(
            String::from("BOOLEAN.FROMFLOAT"),
            Instruction::new(boolean_from_float, 0),
        );
        self.map.insert(
            String::from("BOOLEAN.FROMINTEGER"),
            Instruction::new(boolean_from_integer, 0),
        );
        self.map.insert(
            String::from("BOOLEAN.NOT"),
            Instruction::new(boolean_not, 0),
        );
        self.map
            .insert(String::from("BOOLEAN.OR"), Instruction::new(boolean_or, 0));
        self.map.insert(
            String::from("BOOLEAN.POP"),
            Instruction::new(boolean_pop, 0),
        );
        self.map.insert(
            String::from("BOOLEAN.RAND"),
            Instruction::new(boolean_rand, 0),
        );
        self.map
            .insert(String::from("INTEGER.+"), Instruction::new(integer_add, 0));
        self.map.insert(
            String::from("INTEGER.*"),
            Instruction::new(integer_multiply, 0),
        );
        self.map
            .insert(String::from("FLOAT.+"), Instruction::new(float_add, 0));
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

//
// ------------------ Type: INTEGER ---------------------
//

fn integer_add(push_state: &mut PushState) {
    if let Some(pv) = push_state.int_stack.pop_vec(2) {
        push_state.int_stack.push(pv[0] + pv[1]);
    }
}

fn integer_multiply(push_state: &mut PushState) {
    if let Some(pv) = push_state.int_stack.pop_vec(2) {
        push_state.int_stack.push(pv[0] * pv[1]);
    }
}

//
// ------------------ Type: FLOAT ---------------------
//

fn float_add(push_state: &mut PushState) {
    if let Some(pv) = push_state.float_stack.pop_vec(2) {
        push_state.float_stack.push(pv[0] + pv[1]);
    }
}
//pub trait Instruction {
//    fn evaluate(&self) -> String;
//}
