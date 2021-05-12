use crate::push::atoms::{Atom, PushType};
use crate::push::instructions::Instruction;
use crate::push::state::PushState;
use rand::Rng;
use std::collections::HashMap;

pub fn load_code_instructions(map: &mut HashMap<String, Instruction>) {
    map.insert(String::from("CODE.="), Instruction::new(code_eq, 0));
}

//
// ------------------ Type: BOOLEAN ---------------------
//

pub fn boolean_eq(push_state: &mut PushState) {
    if let Some(pv) = push_state.code_stack.observe_vec(2) {
        push_state
            .bool_stack
            .push(pv[0].to_string() == pv[1].to_string());
    }
}
