use crate::push::instructions::Instruction;
use crate::push::instructions::InstructionCache;
use crate::push::item::Item;
use crate::push::state::PushState;
use std::collections::HashMap;

/// Floating-point numbers (that is, numbers with decimal points).
pub fn load_float_instructions(map: &mut HashMap<String, Instruction>) {
    map.insert(String::from("FLOAT.+"), Instruction::new(float_add));
}

/// FLOAT.%: Pushes the second stack item modulo the top stack item. If the top item is zero this
/// acts as a NOOP. The modulus is computed as the remainder of the quotient, where the quotient
/// has first been truncated toward negative infinity.
fn float_modulus(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fitems) = push_state.float_stack.pop_vec(2) {
        if fitems[1] != 0f32 {
            push_state.float_stack.push(fitems[0] % fitems[1]);
        }
    }
}

/// FLOAT.*: Pushes the product of the top two items.
fn float_mult(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fitems) = push_state.float_stack.pop_vec(2) {
        push_state.float_stack.push(fitems[0] * fitems[1]);
    }
}

/// FLOAT.+: Pushes the sum of the top two items.
fn float_add(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fitems) = push_state.float_stack.pop_vec(2) {
        push_state.float_stack.push(fitems[0] + fitems[1]);
    }
}

/// FLOAT.-: Pushes the difference of the top two items; that is, the second item minus the top
/// item.
fn float_subtract(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fitems) = push_state.float_stack.pop_vec(2) {
        push_state.float_stack.push(fitems[0] - fitems[1]);
    }
}

/// FLOAT./: Pushes the quotient of the top two items; that is, the second item divided by the top
/// item. If the top item is zero this acts as a NOOP.
fn float_divide(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fitems) = push_state.float_stack.pop_vec(2) {
        if fitems[1] != 0f32 {
            push_state.float_stack.push(fitems[0] % fitems[1]);
        }
    }
}

/// FLOAT.<: Pushes TRUE onto the BOOLEAN stack if the second item is less than the top item, or
/// FALSE otherwise.
fn float_smaller(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fitems) = push_state.float_stack.pop_vec(2) {
        push_state.bool_stack.push(fitems[0] < fitems[1]);
    }
}

/// FLOAT.=: Pushes TRUE onto the BOOLEAN stack if the top two items are equal, or FALSE otherwise.
fn float_equal(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fitems) = push_state.float_stack.pop_vec(2) {
        push_state.bool_stack.push(fitems[0] == fitems[1]);
    }
}

/// FLOAT.>: Pushes TRUE onto the BOOLEAN stack if the second item is greater than the top item, or
/// FALSE otherwise.
fn float_greater(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fitems) = push_state.float_stack.pop_vec(2) {
        push_state.bool_stack.push(fitems[0] > fitems[1]);
    }
}

/// FLOAT.COS: Pushes the cosine of the top item.
fn float_cosine(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fitem) = push_state.float_stack.pop() {
        push_state.float_stack.push(fitem.cos());
    }
}

/// FLOAT.DEFINE: Defines the name on top of the NAME stack as an instruction that will push the
/// top item of the FLOAT stack onto the EXEC stack.
pub fn float_define(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(name) = push_state.name_stack.pop() {
        if let Some(fitem) = push_state.float_stack.pop() {
            push_state.name_bindings.insert(name, Item::float(fitem));
        }
    }
}

/// FLOAT.DUP: Duplicates the top item on the FLOAT stack. Does not pop its argument (which, if it
/// did, would negate the effect of the duplication!).
pub fn float_dup(push_state: &mut PushState, _instruction_cache: &InstructionCache) {
    if let Some(fitem) = push_state.float_stack.copy(0) {
        push_state.float_stack.push(fitem);
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
        test_state.float_stack.push(1.0);
        float_mult(&mut test_state, &icache());
        assert!(f32::abs(test_state.float_stack.pop().unwrap() + 0.4) < 0.001f32);
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
        test_state.name_stack.push(&"TEST");
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
        assert_eq!(test_state.float_stack.to_string(), "1:2; 2:2;");
    }
}
