use crate::push::instructions::Instruction;
use crate::push::instructions::InstructionCache;
use crate::push::item::Item;
use crate::push::random::CodeGenerator;
use crate::push::state::PushState;
use std::collections::HashMap;

/// Integer numbers (that is, numbers without decimal points).
pub fn load_int_instructions(map: &mut HashMap<String, Instruction>) {
    map.insert(String::from("INTEGER.%"), Instruction::new(integer_modulus));
    map.insert(String::from("INTEGER.*"), Instruction::new(integer_mult));
    map.insert(String::from("INTEGER.+"), Instruction::new(integer_add));
    map.insert(
        String::from("INTEGER.-"),
        Instruction::new(integer_subtract),
    );
    map.insert(String::from("INTEGER./"), Instruction::new(integer_divide));
    map.insert(String::from("INTEGER.<"), Instruction::new(integer_smaller));
    map.insert(String::from("INTEGER.="), Instruction::new(integer_equal));
    map.insert(String::from("INTEGER.>"), Instruction::new(integer_greater));
    map.insert(
        String::from("INTEGER.DEFINE"),
        Instruction::new(integer_define),
    );
    map.insert(String::from("INTEGER.DUP"), Instruction::new(integer_dup));
    map.insert(
        String::from("INTEGER.FLUSH"),
        Instruction::new(integer_flush),
    );
    map.insert(
        String::from("INTEGER.FROMBOOLEAN"),
        Instruction::new(integer_from_boolean),
    );
    map.insert(
        String::from("INTEGER.FROMFLOAT"),
        Instruction::new(integer_from_float),
    );
    map.insert(String::from("INTEGER.MAX"), Instruction::new(integer_max));
    map.insert(String::from("INTEGER.MIN"), Instruction::new(integer_min));
    map.insert(String::from("INTEGER.POP"), Instruction::new(integer_pop));
    map.insert(String::from("INTEGER.RAND"), Instruction::new(integer_rand));
    map.insert(String::from("INTEGER.ROT"), Instruction::new(integer_rot));
    map.insert(
        String::from("INTEGER.SHOVE"),
        Instruction::new(integer_shove),
    );
    map.insert(
        String::from("INTEGER.STACKDEPTH"),
        Instruction::new(integer_stack_depth),
    );
    map.insert(String::from("INTEGER.SWAP"), Instruction::new(integer_swap));
    map.insert(String::from("INTEGER.YANK"), Instruction::new(integer_yank));
    map.insert(
        String::from("INTEGER.YANKDUP"),
        Instruction::new(integer_yank_dup),
    );
}

/// INTEGER.%: Pushes the second stack item modulo the top stack item. If the top item is zero this
/// acts as a NOOP. The modulus is computed as the remainder of the quotient, where the quotient
/// has first been truncated toward negative infinity.
pub fn integer_modulus(push_state: &mut PushState, _instruction_set: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.pop_vec(2) {
        if ivals[1] != 0i32 {
            push_state.int_stack.push(ivals[0] % ivals[1]);
        }
    }
}

/// INTEGER.*: Pushes the product of the top two items.
fn integer_mult(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.pop_vec(2) {
        push_state.int_stack.push(ivals[0] * ivals[1]);
    }
}

/// INTEGER.+: Pushes the sum of the top two items.
fn integer_add(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.pop_vec(2) {
        push_state.int_stack.push(ivals[0] + ivals[1]);
    }
}

/// INTEGER.-: Pushes the difference of the top two items; that is, the second item minus the top
/// item.
fn integer_subtract(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.pop_vec(2) {
        push_state.int_stack.push(ivals[0] - ivals[1]);
    }
}

/// INTEGER./: Pushes the quotient of the top two items; that is, the second item divided by the
/// top item. If the top item is zero this acts as a NOOP.
fn integer_divide(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.pop_vec(2) {
        if ivals[1] != 0i32 {
            push_state.int_stack.push(ivals[0] / ivals[1]);
        }
    }
}

/// INTEGER.<: Pushes TRUE onto the BOOLEAN stack if the second item is less than the top item, or
/// FALSE otherwise.
fn integer_smaller(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.pop_vec(2) {
        push_state.bool_stack.push(ivals[0] < ivals[1]);
    }
}

/// INTEGER.=: Pushes TRUE onto the BOOLEAN stack if the top two items are equal, or FALSE
/// otherwise.
fn integer_equal(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.pop_vec(2) {
        push_state.bool_stack.push(ivals[0] == ivals[1]);
    }
}

/// INTEGER.>: Pushes TRUE onto the BOOLEAN stack if the second item is greater than the top item,
/// or FALSE otherwise.
fn integer_greater(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.pop_vec(2) {
        push_state.bool_stack.push(ivals[0] > ivals[1]);
    }
}

/// INTEGER.DEFINE: Defines the name on top of the NAME stack as an instruction that will push the
/// top item of the INTEGER stack onto the EXEC stack.
pub fn integer_define(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(name) = push_state.name_stack.pop() {
        if let Some(ival) = push_state.int_stack.pop() {
            push_state.name_bindings.insert(name, Item::int(ival));
        }
    }
}

/// INTEGER.DUP: Duplicates the top item on the INTEGER stack. Does not pop its argument (which, if
/// it did, would negate the effect of the duplication!).
pub fn integer_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ival) = push_state.int_stack.copy(0) {
        push_state.int_stack.push(ival);
    }
}

/// INTEGER.FLUSH: Empties the INTEGER stack.
pub fn integer_flush(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.int_stack.flush();
}

/// INTEGER.FROMBOOLEAN: Pushes 1 if the top BOOLEAN is TRUE, or 0 if the top BOOLEAN is FALSE.
pub fn integer_from_boolean(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(is_true) = push_state.bool_stack.pop() {
        if is_true {
            push_state.int_stack.push(1);
        } else {
            push_state.int_stack.push(0);
        }
    }
}

/// INTEGER.FROMFLOAT: Pushes the result of truncating the top FLOAT.
pub fn integer_from_float(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fval) = push_state.float_stack.pop() {
        push_state.int_stack.push(fval as i32);
    }
}
/// INTEGER.MAX: Pushes the maximum of the top two items.
pub fn integer_max(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.pop_vec(2) {
        if ivals[0] > ivals[1] {
            push_state.int_stack.push(ivals[0]);
        } else {
            push_state.int_stack.push(ivals[1]);
        }
    }
}

/// INTEGER.MIN: Pushes the minimum of the top two items.
pub fn integer_min(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ivals) = push_state.int_stack.pop_vec(2) {
        if ivals[0] > ivals[1] {
            push_state.int_stack.push(ivals[1]);
        } else {
            push_state.int_stack.push(ivals[0]);
        }
    }
}

/// INTEGER.POP: Pops the INTEGER stack.
pub fn integer_pop(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.int_stack.pop();
}

/// INTEGER.RAND: Pushes a newly generated random INTEGER that is greater than or equal to
/// MIN-RANDOM-INTEGER and less than or equal to MAX-RANDOM-INTEGER.
pub fn integer_rand(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(rval) = CodeGenerator::random_integer(push_state) {
        push_state.int_stack.push(rval);
    }
}

/// INTEGER.ROT: Rotates the top three items on the INTEGER stack, pulling the third item out and
/// pushing it on top. This is equivalent to "2 INTEGER.YANK".
pub fn integer_rot(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.int_stack.yank(2);
}

/// INTEGER.SHOVE: Inserts the second INTEGER "deep" in the stack, at the position indexed by the
/// top INTEGER. The index position is calculated after the index is removed.
pub fn integer_shove(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(shove_index) = push_state.int_stack.pop() {
        push_state.int_stack.shove(shove_index as usize);
    }
}

/// INTEGER.STACKDEPTH: Pushes the stack depth onto the INTEGER stack (thereby increasing it!).
pub fn integer_stack_depth(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state
        .int_stack
        .push(push_state.int_stack.size() as i32 + 1);
}

/// INTEGER.SWAP: Swaps the top two INTEGERs.
pub fn integer_swap(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.int_stack.shove(1);
}

/// INTEGER.YANK: Removes an indexed item from "deep" in the stack and pushes it on top of the
/// stack. The index is taken from the INTEGER stack, and the indexing is done after the index is
/// removed.
pub fn integer_yank(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(idx) = push_state.int_stack.pop() {
        push_state.int_stack.yank(idx as usize);
    }
}
/// INTEGER.YANKDUP: Pushes a copy of an indexed item "deep" in the stack onto the top of the
/// stack, without removing the deep item. The index is taken from the INTEGER stack, and the
/// indexing is done after the index is removed.
pub fn integer_yank_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(idx) = push_state.int_stack.pop() {
        if let Some(deep_item) = push_state.int_stack.copy(idx as usize) {
            push_state.int_stack.push(deep_item);
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
    fn integer_modulus_pushes_result() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(-13);
        test_state.int_stack.push(10);
        integer_modulus(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), -3);
    }

    #[test]
    fn integer_mult_pushes_result() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(4);
        test_state.int_stack.push(2);
        integer_mult(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), 8);
    }

    #[test]
    fn integer_add_pushes_result() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(4);
        test_state.int_stack.push(2);
        integer_add(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), 6);
    }

    #[test]
    fn integer_subtract_pushes_result() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(4);
        test_state.int_stack.push(2);
        integer_subtract(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), 2);
    }

    #[test]
    fn integer_divide_pushes_result() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(4);
        test_state.int_stack.push(2);
        integer_divide(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.pop().unwrap(), 2);
    }

    #[test]
    fn integer_smaller_pushes_result() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(4);
        test_state.int_stack.push(10);
        integer_smaller(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
    }

    #[test]
    fn integer_equal_pushes_result() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(4);
        test_state.int_stack.push(4);
        integer_equal(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
    }

    #[test]
    fn integer_greater_pushes_result() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(10);
        test_state.int_stack.push(4);
        integer_greater(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
    }

    #[test]
    fn integer_define_creates_name_binding() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(2);
        test_state.name_stack.push(&"TEST");
        integer_define(&mut test_state, &icache());
        assert_eq!(
            *test_state.name_bindings.get("TEST").unwrap().to_string(),
            Item::int(2).to_string()
        );
    }

    #[test]
    fn integer_dup_copies_top_element() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(2);
        integer_dup(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1:2; 2:2;");
    }

    #[test]
    fn integer_flush_empties_stack() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(213);
        test_state.int_stack.push(2);
        integer_flush(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "");
    }

    #[test]
    fn integer_from_boolean_pushes_one_if_true() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        integer_from_boolean(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1:1;");
    }

    #[test]
    fn integer_from_float_pushes_one_if_true() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(1.0);
        integer_from_float(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1:1;");
    }

    #[test]
    fn integer_max_pushes_greater_item() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(1);
        test_state.int_stack.push(3);
        integer_max(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1:3;");
    }

    #[test]
    fn integer_min_pushes_smaller_item() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(1);
        test_state.int_stack.push(3);
        integer_max(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1:3;");
    }

    #[test]
    fn integer_pop_removes_top_element() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(2);
        test_state.int_stack.push(1);
        integer_pop(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1:2;");
    }

    #[test]
    fn integer_rand_generates_value() {
        let mut test_state = PushState::new();
        integer_rand(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.size(), 1);
    }

    #[test]
    fn integer_rot_shuffles_elements() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(3);
        test_state.int_stack.push(2);
        test_state.int_stack.push(1);
        assert_eq!(test_state.int_stack.to_string(), "1:1; 2:2; 3:3;");
        integer_rot(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1:3; 2:1; 3:2;");
    }

    #[test]
    fn integer_shove_inserts_at_right_position() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(4);
        test_state.int_stack.push(3);
        test_state.int_stack.push(2);
        test_state.int_stack.push(1);
        assert_eq!(test_state.int_stack.to_string(), "1:1; 2:2; 3:3; 4:4;");
        test_state.int_stack.push(2);
        integer_shove(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1:2; 2:3; 3:1; 4:4;");
    }

    #[test]
    fn integer_stack_depth_returns_size() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(4);
        test_state.int_stack.push(3);
        test_state.int_stack.push(2);
        test_state.int_stack.push(1);
        integer_stack_depth(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1:5; 2:1; 3:2; 4:3; 5:4;");
    }

    #[test]
    fn integer_swaps_top_elements() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(0);
        test_state.int_stack.push(1);
        assert_eq!(test_state.int_stack.to_string(), "1:1; 2:0;");
        integer_swap(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1:0; 2:1;");
    }

    #[test]
    fn integer_yank_brings_item_to_top() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(5);
        test_state.int_stack.push(4);
        test_state.int_stack.push(3);
        test_state.int_stack.push(2);
        test_state.int_stack.push(1);
        assert_eq!(test_state.int_stack.to_string(), "1:1; 2:2; 3:3; 4:4; 5:5;");
        test_state.int_stack.push(3);
        integer_yank(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "1:4; 2:1; 3:2; 4:3; 5:5;");
    }

    #[test]
    fn integer_yank_dup_copies_item_to_top() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(5);
        test_state.int_stack.push(4);
        test_state.int_stack.push(3);
        test_state.int_stack.push(2);
        test_state.int_stack.push(1);
        assert_eq!(test_state.int_stack.to_string(), "1:1; 2:2; 3:3; 4:4; 5:5;");
        test_state.int_stack.push(3);
        integer_yank_dup(&mut test_state, &icache());
        assert_eq!(
            test_state.int_stack.to_string(),
            "1:4; 2:1; 3:2; 4:3; 5:4; 6:5;"
        );
    }
}
