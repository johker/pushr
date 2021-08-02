use crate::push::instructions::Instruction;
use crate::push::instructions::InstructionCache;
use crate::push::item::Item;
use crate::push::state::PushState;
use std::collections::HashMap;

/// Integer numbers (that is, numbers without decimal points).
pub fn load_int_instructions(map: &mut HashMap<String, Instruction>) {
    map.insert(String::from("INTEGER.+"), Instruction::new(integer_add));
    map.insert(
        String::from("INTEGER.*"),
        Instruction::new(integer_multiply),
    );
}

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
