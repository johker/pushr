use crate::push::instructions::Instruction;
use crate::push::instructions::InstructionCache;
use crate::push::item::Item;
use crate::push::random::CodeGenerator;
use crate::push::state::PushState;
use crate::push::state::*;
use std::collections::HashMap;

/// Floating-point numbers (that is, numbers with decimal points).
pub fn load_float_instructions(map: &mut HashMap<String, Instruction>) {
    map.insert(String::from("FLOAT.%"), Instruction::new(float_modulus));
    map.insert(String::from("FLOAT.*"), Instruction::new(float_mult));
    map.insert(String::from("FLOAT.+"), Instruction::new(float_add));
    map.insert(String::from("FLOAT.-"), Instruction::new(float_subtract));
    map.insert(String::from("FLOAT./"), Instruction::new(float_divide));
    map.insert(String::from("FLOAT.<"), Instruction::new(float_smaller));
    map.insert(String::from("FLOAT.="), Instruction::new(float_equal));
    map.insert(String::from("FLOAT.>"), Instruction::new(float_greater));
    map.insert(String::from("FLOAT.COS"), Instruction::new(float_cosine));
    map.insert(String::from("FLOAT.DEFINE"), Instruction::new(float_define));
    map.insert(String::from("FLOAT.EXP"), Instruction::new(float_exp));
    map.insert(String::from("FLOAT.DUP"), Instruction::new(float_dup));
    map.insert(String::from("FLOAT.FLUSH"), Instruction::new(float_flush));
    map.insert(
        String::from("FLOAT.FROMBOOLEAN"),
        Instruction::new(float_from_boolean),
    );
    map.insert(String::from("FLOAT.ID"), Instruction::new(float_id));
    map.insert(
        String::from("FLOAT.FROMINTEGER"),
        Instruction::new(float_from_integer),
    );
    map.insert(String::from("FLOAT.MAX"), Instruction::new(float_max));
    map.insert(String::from("FLOAT.MIN"), Instruction::new(float_min));
    map.insert(String::from("FLOAT.POP"), Instruction::new(float_pop));
    map.insert(String::from("FLOAT.RAND"), Instruction::new(float_rand));
    map.insert(String::from("FLOAT.ROT"), Instruction::new(float_rot));
    map.insert(String::from("FLOAT.SHOVE"), Instruction::new(float_shove));
    map.insert(String::from("FLOAT.SIN"), Instruction::new(float_sine));
    map.insert(
        String::from("FLOAT.STACKDEPTH"),
        Instruction::new(float_stack_depth),
    );
    map.insert(String::from("FLOAT.SWAP"), Instruction::new(float_swap));
    map.insert(String::from("FLOAT.TAN"), Instruction::new(float_tan));
    map.insert(String::from("FLOAT.YANK"), Instruction::new(float_yank));
    map.insert(
        String::from("FLOAT.YANKDUP"),
        Instruction::new(float_yank_dup),
    );
}

/// FLOAT.ID: Pushes the ID of the FLOAT stack to the INTEGER stack.
pub fn float_id(push_state: &mut PushState, _instruction_set: &InstructionCache) {
    push_state.int_stack.push(FLOAT_STACK_ID);
}

/// INTEGER.ID: Pushes the ID of the INTEGER stack to the INTEGER stack.
/// FLOAT.%: Pushes the second stack item modulo the top stack item. If the top item is zero this
/// acts as a NOOP. The modulus is computed as the remainder of the quotient, where the quotient
/// has first been truncated toward negative infinity.
fn float_modulus(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fvals) = push_state.float_stack.pop_vec(2) {
        if fvals[1] != 0f32 {
            push_state.float_stack.push(fvals[0] % fvals[1]);
        }
    }
}

/// FLOAT.*: Pushes the product of the top two items.
fn float_mult(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fvals) = push_state.float_stack.pop_vec(2) {
        push_state.float_stack.push(fvals[0] * fvals[1]);
    }
}

/// FLOAT.+: Pushes the sum of the top two items.
fn float_add(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fvals) = push_state.float_stack.pop_vec(2) {
        push_state.float_stack.push(fvals[0] + fvals[1]);
    }
}

/// FLOAT.-: Pushes the difference of the top two items; that is, the second item minus the top
/// item.
fn float_subtract(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fvals) = push_state.float_stack.pop_vec(2) {
        push_state.float_stack.push(fvals[0] - fvals[1]);
    }
}

/// FLOAT./: Pushes the quotient of the top two items; that is, the second item divided by the top
/// item. If the top item is zero this acts as a NOOP.
fn float_divide(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fvals) = push_state.float_stack.pop_vec(2) {
        if fvals[1] != 0f32 {
            push_state.float_stack.push(fvals[0] / fvals[1]);
        }
    }
}

/// FLOAT.EXP: Pushes exp(i) to the float stack where i is taken from the top item on the FLOAT stack.
fn float_exp(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fval) = push_state.float_stack.pop() {
        push_state.float_stack.push(fval.exp());
    }
}

/// FLOAT.<: Pushes TRUE onto the BOOLEAN stack if the second item is less than the top item, or
/// FALSE otherwise.
fn float_smaller(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fvals) = push_state.float_stack.pop_vec(2) {
        push_state.bool_stack.push(fvals[0] < fvals[1]);
    }
}

/// FLOAT.=: Pushes TRUE onto the BOOLEAN stack if the top two items are equal, or FALSE otherwise.
fn float_equal(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fvals) = push_state.float_stack.pop_vec(2) {
        push_state.bool_stack.push(fvals[0] == fvals[1]);
    }
}

/// FLOAT.>: Pushes TRUE onto the BOOLEAN stack if the second item is greater than the top item, or
/// FALSE otherwise.
fn float_greater(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fvals) = push_state.float_stack.pop_vec(2) {
        push_state.bool_stack.push(fvals[0] > fvals[1]);
    }
}

/// FLOAT.COS: Pushes the cosine of the top item.
fn float_cosine(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fval) = push_state.float_stack.pop() {
        push_state.float_stack.push(fval.cos());
    }
}

/// FLOAT.DEFINE: Defines the name on top of the NAME stack as an instruction that will push the
/// top item of the FLOAT stack onto the EXEC stack.
pub fn float_define(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(name) = push_state.name_stack.pop() {
        if let Some(fval) = push_state.float_stack.pop() {
            push_state.name_bindings.insert(name, Item::float(fval));
        }
    }
}

/// FLOAT.DUP: Duplicates the top item on the FLOAT stack. Does not pop its argument (which, if it
/// did, would negate the effect of the duplication!).
pub fn float_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fval) = push_state.float_stack.copy(0) {
        push_state.float_stack.push(fval);
    }
}

/// FLOAT.FLUSH: Empties the FLOAT stack.
pub fn float_flush(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.float_stack.flush();
}

/// FLOAT.FROMBOOLEAN: Pushes 1.0 if the top BOOLEAN is TRUE, or 0.0 if the top BOOLEAN is FALSE.
pub fn float_from_boolean(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(is_true) = push_state.bool_stack.pop() {
        if is_true {
            push_state.float_stack.push(1.0);
        } else {
            push_state.float_stack.push(0.0);
        }
    }
}

/// FLOAT.FROMINTEGER: Pushes a floating point version of the top INTEGER.
pub fn float_from_integer(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(ival) = push_state.int_stack.pop() {
        push_state.float_stack.push(ival as f32);
    }
}

/// FLOAT.MAX: Pushes the maximum of the top two items.
pub fn float_max(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fvals) = push_state.float_stack.pop_vec(2) {
        if fvals[0] > fvals[1] {
            push_state.float_stack.push(fvals[0]);
        } else {
            push_state.float_stack.push(fvals[1]);
        }
    }
}

/// FLOAT.MIN: Pushes the minimum of the top two items.
pub fn float_min(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fvals) = push_state.float_stack.pop_vec(2) {
        if fvals[0] > fvals[1] {
            push_state.float_stack.push(fvals[1]);
        } else {
            push_state.float_stack.push(fvals[0]);
        }
    }
}

/// FLOAT.POP: Pops the FLOAT stack.
pub fn float_pop(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.float_stack.pop();
}

/// FLOAT.RAND: Pushes a newly generated random FLOAT that is greater than or equal to
/// MIN-RANDOM-FLOAT and less than or equal to MAX-RANDOM-FLOAT.
pub fn float_rand(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(rval) = CodeGenerator::random_float(push_state) {
        push_state.float_stack.push(rval);
    }
}

/// FLOAT.ROT: Rotates the top three items on the FLOAT stack, pulling the third item out and
/// pushing it on top. This is equivalent to "2 FLOAT.YANK".
pub fn float_rot(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.float_stack.yank(2);
}

/// FLOAT.SHOVE: Inserts the top FLOAT "deep" in the stack, at the position indexed by the top
/// INTEGER.
pub fn float_shove(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(shove_index) = push_state.int_stack.pop() {
        let corr_index = i32::max(
            i32::min((push_state.float_stack.size() as i32) - 1, shove_index),
            0,
        ) as usize;
        push_state.float_stack.shove(corr_index as usize);
    }
}

/// FLOAT.SIN: Pushes the sine of the top item.
fn float_sine(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fval) = push_state.float_stack.pop() {
        push_state.float_stack.push(fval.sin());
    }
}

/// FLOAT.STACKDEPTH: Pushes the stack depth onto the INTEGER stack.
pub fn float_stack_depth(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state
        .int_stack
        .push(push_state.float_stack.size() as i32);
}

/// FLOAT.SWAP: Swaps the top two FLOATs.
pub fn float_swap(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    push_state.float_stack.shove(1);
}

/// FLOAT.TAN: Pushes the tangent of the top item.
pub fn float_tan(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fval) = push_state.float_stack.pop() {
        push_state.float_stack.push(fval.tan());
    }
}

/// FLOAT.YANK: Removes an indexed item from "deep" in the stack and pushes it on top of the stack.
/// The index is taken from the INTEGER stack.
pub fn float_yank(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        let corr_index = i32::max(
            i32::min((push_state.float_stack.size() as i32) - 1, index),
            0,
        ) as usize;
        push_state.float_stack.yank(corr_index as usize);
    }
}

/// FLOAT.YANKDUP: Pushes a copy of an indexed item "deep" in the stack onto the top of the stack,
/// without removing the deep item. The index is taken from the INTEGER stack.
pub fn float_yank_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(index) = push_state.int_stack.pop() {
        let corr_index = i32::max(
            i32::min((push_state.float_stack.size() as i32) - 1, index),
            0,
        ) as usize;
        if let Some(deep_item) = push_state.float_stack.copy(corr_index as usize) {
            push_state.float_stack.push(deep_item);
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
    fn float_modulus_pushes_result() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(-13.4);
        test_state.float_stack.push(1.0);
        float_modulus(&mut test_state, &icache());
        assert!(f32::abs(test_state.float_stack.pop().unwrap() + 0.4) < 0.001f32);
    }

    #[test]
    fn float_mult_pushes_result() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(-0.4);
        test_state.float_stack.push(1.0);
        float_mult(&mut test_state, &icache());
        assert!(f32::abs(test_state.float_stack.pop().unwrap() + 0.4) < 0.001f32);
    }

    #[test]
    fn float_add_pushes_result() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(-0.4);
        test_state.float_stack.push(0.4);
        float_add(&mut test_state, &icache());
        assert!(f32::abs(test_state.float_stack.pop().unwrap()) < 0.001f32);
    }

    #[test]
    fn float_subtract_pushes_result() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(0.4);
        test_state.float_stack.push(1.0);
        float_subtract(&mut test_state, &icache());
        assert!(f32::abs(test_state.float_stack.pop().unwrap() + 0.6) < 0.001f32);
    }

    #[test]
    fn float_divide_pushes_result() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(0.4);
        test_state.float_stack.push(1.0);
        float_divide(&mut test_state, &icache());
        assert!(f32::abs(test_state.float_stack.pop().unwrap() - 0.4) < 0.001f32);
    }

    #[test]
    fn float_smaller_pushes_result() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(0.4);
        test_state.float_stack.push(1.0);
        float_smaller(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
    }

    #[test]
    fn float_equal_pushes_result() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(0.4);
        test_state.float_stack.push(0.4);
        float_equal(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
    }

    #[test]
    fn float_greater_pushes_result() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(1.0);
        test_state.float_stack.push(0.4);
        float_greater(&mut test_state, &icache());
        assert_eq!(test_state.bool_stack.pop().unwrap(), true);
    }

    #[test]
    fn float_cosine_pushes_result() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(std::f32::consts::PI / 2.0);
        float_cosine(&mut test_state, &icache());
        assert!(f32::abs(test_state.float_stack.pop().unwrap()) < 0.001f32);
    }

    #[test]
    fn float_define_creates_name_binding() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(2.0);
        test_state.name_stack.push(String::from("TEST"));
        float_define(&mut test_state, &icache());
        assert_eq!(
            *test_state.name_bindings.get("TEST").unwrap().to_string(),
            Item::float(2.0).to_string()
        );
    }
    #[test]
    fn float_dup_copies_top_element() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(2.0);
        float_dup(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.to_string(), "2.0 2.0");
    }

    #[test]
    fn float_flush_empties_stack() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(213.1);
        test_state.float_stack.push(2.1);
        float_flush(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.to_string(), "");
    }

    #[test]
    fn float_from_boolean_pushes_one_if_true() {
        let mut test_state = PushState::new();
        test_state.bool_stack.push(true);
        float_from_boolean(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.to_string(), "1.0");
    }

    #[test]
    fn float_from_integer_pushes_one_if_true() {
        let mut test_state = PushState::new();
        test_state.int_stack.push(1);
        float_from_integer(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.to_string(), "1.0");
    }

    #[test]
    fn float_max_pushes_greater_item() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(1.0);
        test_state.float_stack.push(3.0);
        float_max(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.to_string(), "3.0");
    }

    #[test]
    fn float_min_pushes_smaller_item() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(1.0);
        test_state.float_stack.push(3.0);
        float_max(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.to_string(), "3.0");
    }

    #[test]
    fn float_pop_removes_top_element() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(2.4);
        test_state.float_stack.push(2.1);
        float_pop(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.to_string(), "2.4");
    }

    #[test]
    fn float_rand_generates_value() {
        let mut test_state = PushState::new();
        float_rand(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.size(), 1);
    }

    #[test]
    fn float_rot_shuffles_elements() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(3.0);
        test_state.float_stack.push(2.0);
        test_state.float_stack.push(1.0);
        assert_eq!(test_state.float_stack.to_string(), "1.0 2.0 3.0");
        float_rot(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.to_string(), "3.0 1.0 2.0");
    }

    #[test]
    fn float_shove_inserts_at_right_position() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(4.0);
        test_state.float_stack.push(3.0);
        test_state.float_stack.push(2.0);
        test_state.float_stack.push(1.0);
        assert_eq!(test_state.float_stack.to_string(), "1.0 2.0 3.0 4.0");
        test_state.int_stack.push(2);
        float_shove(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.to_string(), "2.0 3.0 1.0 4.0");
    }

    #[test]
    fn float_sine_pushes_result() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(std::f32::consts::PI);
        float_sine(&mut test_state, &icache());
        assert!(f32::abs(test_state.float_stack.pop().unwrap()) < 0.001f32);
    }

    #[test]
    fn float_stack_depth_returns_size() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(4.0);
        test_state.float_stack.push(3.0);
        test_state.float_stack.push(2.0);
        test_state.float_stack.push(1.0);
        float_stack_depth(&mut test_state, &icache());
        assert_eq!(test_state.int_stack.to_string(), "4");
    }

    #[test]
    fn float_swaps_top_elements() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(0.0);
        test_state.float_stack.push(1.0);
        assert_eq!(test_state.float_stack.to_string(), "1.0 0.0");
        float_swap(&mut test_state, &icache());
        assert_eq!(test_state.float_stack.to_string(), "0.0 1.0");
    }

    #[test]
    fn float_tan_pushes_result() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(std::f32::consts::PI);
        float_tan(&mut test_state, &icache());
        assert!(f32::abs(test_state.float_stack.pop().unwrap()) < 0.001f32);
    }

    #[test]
    fn float_yank_brings_item_to_top() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(5.0);
        test_state.float_stack.push(4.0);
        test_state.float_stack.push(3.0);
        test_state.float_stack.push(2.0);
        test_state.float_stack.push(1.0);
        assert_eq!(
            test_state.float_stack.to_string(),
            "1.0 2.0 3.0 4.0 5.0"
        );
        test_state.int_stack.push(3);
        float_yank(&mut test_state, &icache());
        assert_eq!(
            test_state.float_stack.to_string(),
            "4.0 1.0 2.0 3.0 5.0"
        );
    }

    #[test]
    fn float_yank_dup_copies_item_to_top() {
        let mut test_state = PushState::new();
        test_state.float_stack.push(5.0);
        test_state.float_stack.push(4.0);
        test_state.float_stack.push(3.0);
        test_state.float_stack.push(2.0);
        test_state.float_stack.push(1.0);
        assert_eq!(
            test_state.float_stack.to_string(),
            "1.0 2.0 3.0 4.0 5.0"
        );
        test_state.int_stack.push(3);
        float_yank_dup(&mut test_state, &icache());
        assert_eq!(
            test_state.float_stack.to_string(),
            "4.0 1.0 2.0 3.0 4.0 5.0"
        );
    }
}
