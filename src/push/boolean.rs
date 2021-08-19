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

/// BOOLEAN.=: Pushes TRUE if the top two BOOLEANs are equal, or FALSE otherwise.
pub fn boolean_eq(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(pv) = push_state.bool_stack.pop_vec(2) {
        push_state.bool_stack.push(pv[0] == pv[1]);
    }
}

/// BOOLEAN.AND: Pushes the logical AND of the top two BOOLEANs.
pub fn boolean_and(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(pv) = push_state.bool_stack.pop_vec(2) {
        push_state.bool_stack.push(pv[0] && pv[1]);
    }
}

/// BOOLEAN.OR: Pushes the logical OR of the top two BOOLEANs.
pub fn boolean_or(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(pv) = push_state.bool_stack.pop_vec(2) {
        push_state.bool_stack.push(pv[0] || pv[1]);
    }
}

/// BOOLEAN.DEFINE: Defines the name on top of the NAME stack as an instruction that will push the
/// top item of the BOOLEAN stack onto the EXEC stack.
pub fn boolean_def(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(name) = push_state.name_stack.pop() {
        if let Some(bval) = push_state.bool_stack.pop() {
            push_state.name_bindings.insert(
                name,
                Item::Literal {
                    push_type: PushType::Bool { val: bval },
                },
            );
        }
    }
}

/// BOOLEAN.DUP: Duplicates the top item on the BOOLEAN stack. Does not pop its argument (which, if
/// it did, would negate the effect of the duplication!).
pub fn boolean_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(pv) = push_state.bool_stack.copy_vec(1) {
        push_state.bool_stack.push(pv[0]);
    }
}

/// BOOLEAN.FLUSH: Empties the BOOLEAN stack.
pub fn boolean_flush(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.bool_stack.flush();
}

/// BOOLEAN.FROMFLOAT: Pushes FALSE if the top FLOAT is 0.0, or TRUE otherwise.
pub fn boolean_from_float(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(pv) = push_state.float_stack.copy_vec(1) {
        let x = pv[0] == 0.0;
        push_state.bool_stack.push(x);
    }
}

/// BOOLEAN.FROMINTEGER: Pushes FALSE if the top INTEGER is 0, or TRUE otherwise.
pub fn boolean_from_integer(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(pv) = push_state.int_stack.copy_vec(1) {
        let x = pv[0] == 0;
        push_state.bool_stack.push(x);
    }
}

/// BOOLEAN.NOT: Pushes the logical NOT of the top BOOLEAN.
pub fn boolean_not(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(pv) = push_state.bool_stack.pop() {
        push_state.bool_stack.push(!pv);
    }
}

/// BOOLEAN.OR: Pushes the logical OR of the top two BOOLEANs.
pub fn boolean_pop(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.bool_stack.pop();
}

/// BOOLEAN.POP: Pops the BOOLEAN stack.
pub fn boolean_rand(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    let mut rng = rand::thread_rng();
    let bval = rng.gen_range(0..2) == 1;
    push_state.bool_stack.push(bval);
}

/// BOOLEAN.ROT: Rotates the top three items on the BOOLEAN stack, pulling the third item out and
/// pushing it on top. This is equivalent to "2 BOOLEAN.YANK".
pub fn boolean_rot(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.bool_stack.yank(2);
}

/// BOOLEAN.SHOVE: Inserts the top BOOLEAN "deep" in the stack, at the position indexed by the top
/// INTEGER.
pub fn boolean_shove(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ival) = push_state.int_stack.pop() {
        push_state.bool_stack.shove(ival as usize);
    }
}

/// BOOLEAN.STACKDEPTH: Pushes the stack depth onto the INTEGER stack.
pub fn boolean_stack_depth(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state
        .int_stack
        .push(push_state.bool_stack.size() as i32);
}

/// BOOLEAN.SWAP: Swaps the top two BOOLEANs.
pub fn boolean_swap(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.bool_stack.shove(1);
}

/// BOOLEAN.YANK: Removes an indexed item from "deep" in the stack and pushes it on top of the
/// stack. The index is taken from the INTEGER stack.
pub fn boolean_yank(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ival) = push_state.int_stack.pop() {
        push_state.bool_stack.yank(ival as usize);
    }
}

/// BOOLEAN.YANKDUP: Pushes a copy of an indexed item "deep" in the stack onto the top of the
/// stack, without removing the deep item. The index is taken from the INTEGER stack.
pub fn boolean_yank_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(idx) = push_state.int_stack.pop() {
        if let Some(deep_item) = push_state.bool_stack.copy(idx as usize) {
            push_state.bool_stack.push(deep_item);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    pub fn icache() -> InstructionCache {
        InstructionCache::new(vec![])
    }

    #[test]
    fn boolean_equal_pushes_result() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(true);
        boolean_eq(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
    }

    #[test]
    fn boolean_and_pushes_result() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(false);
        boolean_and(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), false);
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(true);
        boolean_and(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
    }

    #[test]
    fn boolean_define_creates_name_binding() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        test_state.name_stack.push(String::from("TEST"));
        boolean_def(&mut test_state, &icache());
        assert_eq!(
            *test_state.name_bindings.get("TEST").unwrap().to_string(),
            Item::bool(true).to_string()
        );
    }

    #[test]
    fn boolean_dup_copies_top_element() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(false);
        boolean_dup(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.to_string(), "1:false; 2:false;");
    }

    #[test]
    fn boolean_flush_empties_stack() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(false);
        boolean_flush(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.to_string(), "");
    }

    #[test]
    fn boolean_from_float_compares_to_zero() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(0.0);
        boolean_from_float(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.to_string(), "1:true;");
        test_state.float_stack.push(0.01);
        boolean_from_float(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.to_string(), "1:false; 2:true;");
    }

    #[test]
    fn boolean_from_integer_compares_to_zero() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(0);
        boolean_from_integer(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.to_string(), "1:true;");
        test_state.int_stack.push(1);
        boolean_from_integer(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.to_string(), "1:false; 2:true;");
    }

    #[test]
    fn boolean_or_pushes_result() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(false);
        boolean_or(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
        test_state.bool_stack.push(false);
        test_state.bool_stack.push(false);
        boolean_or(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), false);
    }

    #[test]
    fn boolean_not_pushes_result() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        boolean_not(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), false);
        test_state.bool_stack.push(false);
        boolean_not(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
    }

    #[test]
    fn boolean_pop_removes_top_element() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(false);
        boolean_pop(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.to_string(), "1:true;");
    }

    #[test]
    fn boolean_rand_generates_value() {
        let mut test_state = PushState::new();
        boolean_rand(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.size(), 1);
    }

    #[test]
    fn boolean_rot_shuffles_elements() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(false);
        assert_eq!(
            test_state.bool_stack.to_string(),
            "1:false; 2:true; 3:true;"
        );
        boolean_rot(&mut test_state, &icache());
        assert_eq!(
            test_state.bool_stack.to_string(),
            "1:true; 2:false; 3:true;"
        );
    }

    #[test]
    fn boolean_shove_inserts_at_right_position() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(false);
        assert_eq!(
            test_state.bool_stack.to_string(),
            "1:false; 2:true; 3:true; 4:true;"
        );
        test_state.int_stack.push(2);
        boolean_shove(&mut test_state, &icache());
        assert_eq!(
            test_state.bool_stack.to_string(),
            "1:true; 2:true; 3:false; 4:true;"
        );
    }

    #[test]
    fn boolean_stack_depth_returns_size() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(false);
        boolean_stack_depth(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1:4;");
    }

    #[test]
    fn boolean_swaps_top_elements() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(false);
        assert_eq!(test_state.bool_stack.to_string(), "1:false; 2:true;");
        boolean_swap(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.to_string(), "1:true; 2:false;");
    }

    #[test]
    fn boolean_yank_brings_item_to_top() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(false);
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(true);
        assert_eq!(
            test_state.bool_stack.to_string(),
            "1:true; 2:true; 3:false; 4:true;"
        );
        test_state.int_stack.push(2);
        boolean_yank(&mut test_state, &icache());
        assert_eq!(
            test_state.bool_stack.to_string(),
            "1:false; 2:true; 3:true; 4:true;"
        );
    }

    #[test]
    fn boolean_yank_dup_copies_item_to_top() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(false);
        test_state.bool_stack.push(true);
        test_state.bool_stack.push(true);
        assert_eq!(
            test_state.bool_stack.to_string(),
            "1:true; 2:true; 3:false; 4:true;"
        );
        test_state.int_stack.push(2);
        boolean_yank_dup(&mut test_state, &icache());
        assert_eq!(
            test_state.bool_stack.to_string(),
            "1:false; 2:true; 3:true; 4:false; 5:true;"
        );
    }
}
