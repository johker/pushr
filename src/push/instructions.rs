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
    pub instructionMap: HashMap<String, Instruction>,
}

impl InstructionSet {
    pub fn new(&mut self) {
        self.instructionMap.insert(
            String::from("INTEGER.+"),
            Instruction::IntegerAdd(IntegerAdd {}),
        );
    }
}

pub enum Instruction {
    IntegerAdd(IntegerAdd),
}

pub struct IntegerAdd {}

impl IntegerAdd {
    fn execute(push_state: &mut PushState) {
        match push_state.int_stack.pop_vec(2) {}
    }
}

//pub trait Instruction {
//    fn evaluate(&self) -> String;
//}
