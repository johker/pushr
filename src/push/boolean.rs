use crate::push::instructions::Instruction;
use crate::push::instructions::InstructionCache;
use crate::push::item::{Item, PushType};
use crate::push::state::PushState;
use rand::Rng;
use std::collections::HashMap;

pub fn load_boolean_instructions(map: &mut HashMap<String, Instruction>) {
    map.insert(String::from("BOOLEAN.="), Instruction::new(boolean_eq));
    map.insert(String::from("BOOLEAN.AND"), Instruction::new(boolean_and));
    map.insert(
        String::from("BOOLEAN.DEFINE"),
        Instruction::new(boolean_def),
    );
    map.insert(String::from("BOOLEAN.DUP"), Instruction::new(boolean_dup));
    map.insert(
        String::from("BOOLEAN.FLUSH"),
        Instruction::new(boolean_flush),
    );
    map.insert(
        String::from("BOOLEAN.FROMFLOAT"),
        Instruction::new(boolean_from_float),
    );
    map.insert(
        String::from("BOOLEAN.FROMINTEGER"),
        Instruction::new(boolean_from_integer),
    );
    map.insert(String::from("BOOLEAN.NOT"), Instruction::new(boolean_not));
    map.insert(String::from("BOOLEAN.OR"), Instruction::new(boolean_or));
    map.insert(String::from("BOOLEAN.POP"), Instruction::new(boolean_pop));
    map.insert(String::from("BOOLEAN.RAND"), Instruction::new(boolean_rand));
    map.insert(String::from("BOOLEAN.ROT"), Instruction::new(boolean_rot));
    map.insert(
        String::from("BOOLEAN.SHOVE"),
        Instruction::new(boolean_shove),
    );
    map.insert(
        String::from("BOOLEAN.STACKDEPTH"),
        Instruction::new(boolean_stack_depth),
    );
    map.insert(String::from("BOOLEAN.SWAP"), Instruction::new(boolean_swap));
    map.insert(String::from("BOOLEAN.YANK"), Instruction::new(boolean_yank));
    map.insert(
        String::from("BOOLEAN.YANKDUP"),
        Instruction::new(boolean_yank_dup),
    );
}

//
// ------------------ Type: BOOLEAN ---------------------
//

pub fn boolean_eq(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(pv) = push_state.bool_stack.pop_vec(2) {
        push_state.bool_stack.push(pv[0] == pv[1]);
    }
}

pub fn boolean_and(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(pv) = push_state.bool_stack.pop_vec(2) {
        push_state.bool_stack.push(pv[0] && pv[1]);
    }
}

pub fn boolean_or(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(pv) = push_state.bool_stack.pop_vec(2) {
        push_state.bool_stack.push(pv[0] || pv[1]);
    }
}

pub fn boolean_def(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(name) = push_state.name_stack.pop() {
        if let Some(bval) = push_state.bool_stack.pop() {
            push_state.name_bindings.insert(
                name,
                Item::Literal {
                    push_type: PushType::PushBoolType { val: bval },
                },
            );
        }
    }
}

pub fn boolean_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(pv) = push_state.bool_stack.copy_vec(1) {
        push_state.bool_stack.push(pv[0]);
    }
}

pub fn boolean_flush(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.bool_stack.flush();
}

pub fn boolean_from_float(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(pv) = push_state.float_stack.copy_vec(1) {
        // TODO: Float comparison?
        let x = pv[0] == 0.0;
        push_state.bool_stack.push(x);
    }
}

pub fn boolean_from_integer(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(pv) = push_state.int_stack.copy_vec(1) {
        let x = pv[0] == 0;
        push_state.bool_stack.push(x);
    }
}

pub fn boolean_not(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(pv) = push_state.bool_stack.pop() {
        push_state.bool_stack.push(!pv);
    }
}

pub fn boolean_pop(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.bool_stack.pop();
}

pub fn boolean_rand(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    let mut rng = rand::thread_rng();
    let bval = rng.gen_range(0..2) == 1;
    push_state.bool_stack.push(bval);
}

pub fn boolean_rot(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.bool_stack.yank(2);
}

pub fn boolean_shove(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ival) = push_state.int_stack.pop() {
        push_state.bool_stack.shove(ival as usize);
    }
}

pub fn boolean_stack_depth(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state
        .int_stack
        .push(push_state.bool_stack.size() as i32);
}

pub fn boolean_swap(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.bool_stack.shove(1);
}

pub fn boolean_yank(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ival) = push_state.int_stack.pop() {
        push_state.bool_stack.yank(ival as usize);
    }
}

pub fn boolean_yank_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ival) = push_state.int_stack.pop() {
        if let Some(pv) = push_state.bool_stack.copy_vec(ival as usize) {
            push_state.bool_stack.push(pv[pv.len() - 1]);
        }
    }
}
