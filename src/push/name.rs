use crate::push::instructions::Instruction;
use crate::push::instructions::InstructionCache;
use crate::push::random::CodeGenerator;
use crate::push::state::PushState;
use crate::push::state::*;
use std::collections::HashMap;

/// For creating bindings between symbolic identifiers and values of various types; that is,
/// for implementing (global) variables and defined instructions. Bindings are created with
/// DEFINE instructions. Any identifier that is not a known Push instruction or a known literal
/// of any other type is considered a NAME and will be pushed onto the NAME stack when
/// encountered, unless it has a definition (in which case its associated value will be pushed
/// on the EXEC stack when it is encountered. The NAME.QUOTE instruction can be used to get a
/// name that already has a definition onto the NAME stack.
pub fn load_name_instructions(map: &mut HashMap<String, Instruction>) {
    map.insert(String::from("NAME.="), Instruction::new(name_equal));
    map.insert(String::from("NAME.DUP"), Instruction::new(name_dup));
    map.insert(String::from("NAME.FLUSH"), Instruction::new(name_flush));
    map.insert(String::from("NAME.ID"), Instruction::new(name_id));
    map.insert(String::from("NAME.POP"), Instruction::new(name_pop));
    map.insert(String::from("NAME.QUOTE"), Instruction::new(name_quote));
    map.insert(String::from("NAME.RAND"), Instruction::new(name_rand));
    map.insert(
        String::from("NAME.RANDBOUNDNAME"),
        Instruction::new(name_rand_bound),
    );
    map.insert(String::from("NAME.ROT"), Instruction::new(name_rot));
    map.insert(String::from("NAME.SHOVE"), Instruction::new(name_shove));
    map.insert(
        String::from("NAME.STACKDEPTH"),
        Instruction::new(name_stack_depth),
    );
    map.insert(String::from("NAME.SWAP"), Instruction::new(name_swap));
    map.insert(String::from("NAME.YANK"), Instruction::new(name_yank));
    map.insert(
        String::from("NAME.YANKDUP"),
        Instruction::new(name_yank_dup),
    );
}

/// NAME.ID: Pushes the ID of the NAME stack to the INTEGER stack.
pub fn name_id(push_state: &mut PushState, _instruction_set: &InstructionCache) {
    push_state.int_stack.push(NAME_STACK_ID);
}

/// NAME.=: Pushes TRUE if the top two NAMEs are equal, or FALSE otherwise.
fn name_equal(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(nvals) = push_state.name_stack.pop_vec(2) {
        push_state.bool_stack.push(nvals[0] == nvals[1]);
    }
}

/// NAME.DUP: Duplicates the top item on the NAME stack. Does not pop its argument (which, if it
/// did, would negate the effect of the duplication!).
pub fn name_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(nval) = push_state.name_stack.copy(0) {
        push_state.name_stack.push(nval);
    }
}

/// NAME.FLUSH: Empties the NAME stack.
pub fn name_flush(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.name_stack.flush();
}

/// NAME.POP: Pops the NAME stack.
pub fn name_pop(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.name_stack.pop();
}

/// NAME.QUOTE: Sets a flag indicating that the next name encountered will be pushed onto the NAME
/// stack (and not have its associated value pushed onto the EXEC stack), regardless of whether or
/// not it has a definition. Upon encountering such a name and pushing it onto the NAME stack the
/// flag will be cleared (whether or not the pushed name had a definition).
pub fn name_quote(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.quote_name = true;
}

/// NAME.RAND: Pushes a newly generated random NAME.
pub fn name_rand(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.name_stack.push(CodeGenerator::new_random_name());
}

/// NAME.RANDBOUNDNAME: Pushes a randomly selected NAME that already has a definition.
pub fn name_rand_bound(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state
        .name_stack
        .push(CodeGenerator::existing_random_name(push_state));
}

/// NAME.ROT: Rotates the top three items on the NAME stack, pulling the third item out and pushing
/// it on top. This is equivalent to "2 NAME.YANK".
pub fn name_rot(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.name_stack.yank(2);
}

/// NAME.SHOVE: Inserts the top NAME "deep" in the stack, at the position indexed by the top
/// INTEGER.
pub fn name_shove(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(shove_index) = push_state.int_stack.pop() {
        let corr_index = i32::max(
            i32::min((push_state.name_stack.size() as i32) - 1, shove_index),
            0,
        ) as usize;
        push_state.name_stack.shove(corr_index as usize);
    }
}

/// NAME.STACKDEPTH: Pushes the stack depth onto the INTEGER stack.
pub fn name_stack_depth(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state
        .int_stack
        .push(push_state.name_stack.size() as i32);
}

/// NAME.SWAP: Swaps the top two NAMEs.
pub fn name_swap(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.name_stack.shove(1);
}

/// NAME.YANK: Removes an indexed item from "deep" in the stack and pushes it on top of the stack.
/// The index is taken from the INTEGER stack.
pub fn name_yank(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        let corr_index = i32::max(
            i32::min((push_state.name_stack.size() as i32) - 1, index),
            0,
        ) as usize;
        push_state.name_stack.yank(corr_index as usize);
    }
}

/// NAME.YANKDUP: Pushes a copy of an indexed item "deep" in the stack onto the top of the stack,
/// without removing the deep item. The index is taken from the INTEGER stack.
pub fn name_yank_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        let corr_index = i32::max(
            i32::min((push_state.name_stack.size() as i32) - 1, index),
            0,
        ) as usize;
        if let Some(deep_item) = push_state.name_stack.copy(corr_index) {
            push_state.name_stack.push(deep_item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::push::item::Item;

    pub fn icache() -> InstructionCache {
        InstructionCache::new(vec![])
    }
    #[test]
    fn name_equal_pushes_result() {
        let mut test_state = PushState::new();
        test_state.name_stack.push(String::from("Test"));
        test_state.name_stack.push(String::from("Test"));
        name_equal(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
    }

    #[test]
    fn name_dup_copies_top_element() {
        let mut test_state = PushState::new();
        test_state.name_stack.push(String::from("Test"));
        name_dup(&mut test_state, &icache());
        assert_eq!(test_state.name_stack.to_string(), "1:Test; 2:Test;");
    }

    #[test]
    fn name_flush_empties_stack() {
        let mut test_state = PushState::new();
        test_state.name_stack.push(String::from("I1"));
        test_state.name_stack.push(String::from("I2"));
        name_flush(&mut test_state, &icache());
        assert_eq!(test_state.name_stack.to_string(), "");
    }
    #[test]
    fn name_rand_generates_value() {
        let mut test_state = PushState::new();
        name_rand(&mut test_state, &icache());
        assert_eq!(test_state.name_stack.size(), 1);
    }
    #[test]
    fn name_rand_bound_generates_value() {
        let mut test_state = PushState::new();
        test_state
            .name_bindings
            .insert(CodeGenerator::new_random_name(), Item::int(1));
        name_rand_bound(&mut test_state, &icache());
        assert_eq!(test_state.name_stack.size(), 1);
    }

    #[test]
    fn name_rot_shuffles_elements() {
        let mut test_state = PushState::new();
        test_state.name_stack.push(String::from("Test3"));
        test_state.name_stack.push(String::from("Test2"));
        test_state.name_stack.push(String::from("Test1"));
        assert_eq!(
            test_state.name_stack.to_string(),
            "1:Test1; 2:Test2; 3:Test3;"
        );
        name_rot(&mut test_state, &icache());
        assert_eq!(
            test_state.name_stack.to_string(),
            "1:Test3; 2:Test1; 3:Test2;"
        );
    }

    #[test]
    fn name_shove_inserts_at_right_position() {
        let mut test_state = PushState::new();
        test_state.name_stack.push(String::from("Test4"));
        test_state.name_stack.push(String::from("Test3"));
        test_state.name_stack.push(String::from("Test2"));
        test_state.name_stack.push(String::from("Test1"));
        assert_eq!(
            test_state.name_stack.to_string(),
            "1:Test1; 2:Test2; 3:Test3; 4:Test4;"
        );
        test_state.int_stack.push(2);
        name_shove(&mut test_state, &icache());
        assert_eq!(
            test_state.name_stack.to_string(),
            "1:Test2; 2:Test3; 3:Test1; 4:Test4;"
        );
    }

    #[test]
    fn name_stack_depth_returns_size() {
        let mut test_state = PushState::new();
        test_state.name_stack.push(String::from("Test4"));
        test_state.name_stack.push(String::from("Test3"));
        test_state.name_stack.push(String::from("Test2"));
        test_state.name_stack.push(String::from("Test1"));
        name_stack_depth(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1:4;");
    }

    #[test]
    fn name_swaps_top_elements() {
        let mut test_state = PushState::new();
        test_state.name_stack.push(String::from("Test2"));
        test_state.name_stack.push(String::from("Test1"));
        assert_eq!(test_state.name_stack.to_string(), "1:Test1; 2:Test2;");
        name_swap(&mut test_state, &icache());
        assert_eq!(test_state.name_stack.to_string(), "1:Test2; 2:Test1;");
    }

    #[test]
    fn name_yank_brings_item_to_top() {
        let mut test_state = PushState::new();
        test_state.name_stack.push(String::from("Test5"));
        test_state.name_stack.push(String::from("Test4"));
        test_state.name_stack.push(String::from("Test3"));
        test_state.name_stack.push(String::from("Test2"));
        test_state.name_stack.push(String::from("Test1"));
        assert_eq!(
            test_state.name_stack.to_string(),
            "1:Test1; 2:Test2; 3:Test3; 4:Test4; 5:Test5;"
        );
        test_state.int_stack.push(3);
        name_yank(&mut test_state, &icache());
        assert_eq!(
            test_state.name_stack.to_string(),
            "1:Test4; 2:Test1; 3:Test2; 4:Test3; 5:Test5;"
        );
    }

    #[test]
    fn name_yank_dup_copies_item_to_top() {
        let mut test_state = PushState::new();
        test_state.name_stack.push(String::from("Test5"));
        test_state.name_stack.push(String::from("Test4"));
        test_state.name_stack.push(String::from("Test3"));
        test_state.name_stack.push(String::from("Test2"));
        test_state.name_stack.push(String::from("Test1"));
        assert_eq!(
            test_state.name_stack.to_string(),
            "1:Test1; 2:Test2; 3:Test3; 4:Test4; 5:Test5;"
        );
        test_state.int_stack.push(3);
        name_yank_dup(&mut test_state, &icache());
        assert_eq!(
            test_state.name_stack.to_string(),
            "1:Test4; 2:Test1; 3:Test2; 4:Test3; 5:Test4; 6:Test5;"
        );
    }
}
