use crate::push::atoms::{Atom, PushType};
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
// ------------------ Type: BOOLEAN ---------------------
//

fn boolean_eq(push_state: &mut PushState) {
    if let Some(pv) = push_state.bool_stack.pop_vec(2) {
        push_state.bool_stack.push(pv[0] == pv[1]);
    }
}

fn boolean_and(push_state: &mut PushState) {
    if let Some(pv) = push_state.bool_stack.pop_vec(2) {
        push_state.bool_stack.push(pv[0] && pv[1]);
    }
}

fn boolean_or(push_state: &mut PushState) {
    if let Some(pv) = push_state.bool_stack.pop_vec(2) {
        push_state.bool_stack.push(pv[0] || pv[1]);
    }
}

fn boolean_def(push_state: &mut PushState) {
    if let Some(name) = push_state.name_stack.pop() {
        if let Some(bval) = push_state.bool_stack.pop() {
            push_state.name_bindings.insert(
                name,
                Atom::Literal {
                    push_type: PushType::PushBoolType { val: bval },
                },
            );
        }
    }
}

fn boolean_dup(push_state: &mut PushState) {
    if let Some(pv) = push_state.bool_stack.observe_vec(1) {
        push_state.bool_stack.push(pv[0]);
    }
}

fn boolean_flush(push_state: &mut PushState) {
    push_state.bool_stack.flush();
}

fn boolean_from_float(push_state: &mut PushState) {
    if let Some(pv) = push_state.float_stack.observe_vec(1) {
        // TODO: Float comparison?
        let x = pv[0] == 0.0;
        push_state.bool_stack.push(x);
    }
}

fn boolean_from_integer(push_state: &mut PushState) {
    if let Some(pv) = push_state.int_stack.observe_vec(1) {
        let x = pv[0] == 0;
        push_state.bool_stack.push(x);
    }
}

fn boolean_not(push_state: &mut PushState) {
    if let Some(pv) = push_state.bool_stack.pop() {
        push_state.bool_stack.push(!pv);
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
